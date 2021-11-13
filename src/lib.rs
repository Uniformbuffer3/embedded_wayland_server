#[cfg(test)]
mod tests;

pub mod definitions;
pub use definitions::*;

use std::time::Duration;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Cell;

#[derive(Debug)]
pub struct DispatchContext {
    requests: Vec<WaylandRequest>,
}
impl DispatchContext {
    pub fn new()->Self {
        let requests = Vec::new();

        Self {
            requests,
        }
    }
}

#[derive(Debug)]
pub struct EmbeddedWaylandServer {
    dispatch_context: Rc<RefCell<DispatchContext>>,
    display: Display,

    compositor_global: Global<WlCompositor>,
    subcompositor_global: Global<WlSubcompositor>,

    seat_globals: HashMap<usize,(Seat,Global<WlSeat>)>,
    output_globals: HashMap<usize,(Output,Global<WlOutput>)>,

    #[cfg(feature="shm")]
    shm_global: Global<WlShm>,

    #[cfg(feature="xdg_shell")]
    xdg_shell_state: Arc<Mutex<XdgShellState>>,
    #[cfg(feature="xdg_shell")]
    xdg_wm_base_global: Global<XdgWmBase>,

    #[cfg(feature="dma_buf")]
    zwp_linux_dmabuf_v1_global: Global<ZwpLinuxDmabufV1>,

    #[cfg(feature="dnd")]
    dnd_global: Global<WlDataDeviceManager>,

    #[cfg(feature="explicit_synchronization")]
    explicit_synchronization_global: Global<ZwpLinuxExplicitSynchronizationV1>
}

impl EmbeddedWaylandServer {
    pub fn new(parameters: Parameters)->Self
    {
        let mut display = Display::new();
        display.add_socket_auto().expect("Failed to bind wayland socket");

        let dispatch_context = Rc::new(RefCell::new(DispatchContext::new()));

        let seat_globals = HashMap::new();
        let output_globals = HashMap::new();


        let (compositor_global,subcompositor_global) = smithay::wayland::compositor::compositor_init(&mut display,|surface, mut dispatch_data|{
            let dispatch_context: &mut Rc<RefCell<DispatchContext>> = dispatch_data.get().unwrap();
            dispatch_context.borrow_mut().requests.push(WaylandRequest::Commit{surface});
        },None);

        #[cfg(feature="shm")]
        let shm_global = smithay::wayland::shm::init_shm_global(&mut display,parameters.shm_formats,None);

        #[cfg(feature="xdg_shell")]
        let (xdg_shell_state,xdg_wm_base_global) = smithay::wayland::shell::xdg::xdg_shell_init(&mut display,|request,mut dispatch_data|{
            match &request {
                XdgRequest::NewToplevel{surface}=>{
                    surface.get_surface().map(|surface|{
                        let id: u32 = SERIAL_COUNTER.next_serial().into();
                        let result = with_states(surface,|surface_data|{
                            surface_data.data_map.insert_if_missing(||SurfaceId::from(id));
                        });
                        log::info!(target: "EWS","Assigned id {} to {:#?}",id,surface);
                        match result {
                            Ok(_)=>(),
                            Err(err)=>log::error!(target: "EWS","Error while setting NewPopup surface id: {:#?}",err)
                        }
                    });
                }
                XdgRequest::NewPopup{surface,positioner}=>{
                    surface.get_surface().map(|surface|{
                        let id: u32 = SERIAL_COUNTER.next_serial().into();
                        let result = with_states(surface,|surface_data|{
                            surface_data.data_map.insert_if_missing(||SurfaceId::from(id));
                        });
                        log::info!(target: "EWS","Assigned id {} to {:#?}",id,surface);
                        match result {
                            Ok(_)=>(),
                            Err(err)=>log::error!(target: "EWS","Error while setting NewPopup surface id: {:#?}",err)
                        }
                    });
                }
                _=>()
            }


            let dispatch_context: &mut Rc<RefCell<DispatchContext>> = dispatch_data.get().unwrap();
            dispatch_context.borrow_mut().requests.push(WaylandRequest::XdgRequest{request});
        },None);

        #[cfg(feature="dma_buf")]
        let zwp_linux_dmabuf_v1_global = smithay::wayland::dmabuf::init_dmabuf_global(&mut display,parameters.drm_formats,|dmabuf,mut dispatch_data|{
            let buffer = dmabuf.clone();
            let dispatch_context: &mut Rc<RefCell<DispatchContext>> = dispatch_data.get().unwrap();
            dispatch_context.borrow_mut().requests.push(WaylandRequest::Dmabuf{buffer});
            true
        },None);

        #[cfg(feature="dnd")]
        let dnd_global = {
            let dispatch_context = dispatch_context.clone();
            init_data_device(
                &mut display,
                move|dnd| {
                    dispatch_context.borrow_mut().requests.push(WaylandRequest::Dnd{dnd});
                },
                default_action_chooser,
                None
            )
        };

        #[cfg(feature="explicit_synchronization")]
        let explicit_synchronization_global = init_explicit_synchronization_global(&mut display,None);

        Self {
            display,

            dispatch_context,

            seat_globals,
            output_globals,

            compositor_global,
            subcompositor_global,

            #[cfg(feature="shm")]
            shm_global,

            #[cfg(feature="xdg_shell")]
            xdg_shell_state,
            #[cfg(feature="xdg_shell")]
            xdg_wm_base_global,

            #[cfg(feature="dma_buf")]
            zwp_linux_dmabuf_v1_global,

            #[cfg(feature="dnd")]
            dnd_global,

            #[cfg(feature="explicit_synchronization")]
            explicit_synchronization_global,
        }
    }

    pub fn dispatch(&mut self, duration: Duration)->Vec<WaylandRequest>{
        match self.display.dispatch(Duration::from_millis(0),&mut self.dispatch_context){
            Ok(())=>{}
            Err(_err)=>{}
        }
        self.display.flush_clients(&mut self.dispatch_context);
        self.dispatch_context.borrow_mut().requests.drain(..).collect()
    }

    pub fn create_seat(&mut self, id: usize, name: impl Into<String>) {
        let name = name.into();
        let seat = Seat::new(&mut self.display,name,None);
        let seat_id = SeatId(id);
        seat.0.user_data().insert_if_missing(||seat_id);

        let cursor_surface: Cell<Option<SurfaceId>> = Cell::new(None);
        seat.0.user_data().insert_if_missing(||cursor_surface);
        self.seat_globals.insert(id,seat);
    }
    pub fn destroy_seat(&mut self,id: usize) {
        self.seat_globals.remove(&id);
    }
    pub fn list_seats(&self)->impl Iterator<Item=&Seat>{
        self.seat_globals.values().map(|(seat,global)|seat)
    }

    pub fn add_keyboard(&mut self,seat_id: usize,repeat_delay: i32,repeat_rate: i32) {
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&seat_id){
            if seat.get_keyboard().is_none(){
                let dispatch_context = self.dispatch_context.clone();

                seat.add_keyboard(
                    XkbConfig::default(),
                    repeat_delay,
                    repeat_rate,
                    move|seat,focus|{
                        dispatch_context.borrow_mut().requests.push(WaylandRequest::Seat{
                            seat: seat.clone(),
                            request: SeatRequest::KeaybordFocus(focus.cloned())
                        });
                    }
                ).unwrap();
            }
        }
    }
    pub fn del_keyboard(&mut self, seat_id: usize) {
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&seat_id){
            seat.remove_keyboard();
        }
    }
    pub fn get_keyboard(&self,seat_id: usize)->Option<KeyboardHandle> {
        self.seat_globals.get(&seat_id).map(|seat|seat.0.get_keyboard()).flatten()
    }

    pub fn add_cursor(&mut self, seat_id: usize) {
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&seat_id){
            if seat.get_pointer().is_none(){
                let dispatch_context = self.dispatch_context.clone();
                let seat_cloned = seat.clone();
                seat.add_pointer(move |cursor_image_status|{
                    let seat_cloned = &seat_cloned;

                    match &cursor_image_status {
                        CursorImageStatus::Image(surface)=>{
                            let id: u32 = SERIAL_COUNTER.next_serial().into();
                            let result = with_states(&surface,|surface_data|{
                                surface_data.data_map.insert_if_missing(||SurfaceId::from(id));
                            });
                            match result {
                                Ok(_)=>(),
                                Err(err)=>println!("Error while setting NewPopup surface id: {:#?}", err)
                            }

                        }
                        _=>()
                    }

                    dispatch_context.borrow_mut().requests.push(WaylandRequest::Seat{
                        seat: seat_cloned.clone(),
                        request: SeatRequest::CursorImage(cursor_image_status)
                    });
                });
            }
        }
    }
    pub fn del_cursor(&mut self, seat_id: usize) {
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&seat_id){
            seat.remove_pointer();
        }
    }
    pub fn get_cursor(&self,seat_id: usize)->Option<PointerHandle> {
        self.seat_globals.get(&seat_id).map(|seat|seat.0.get_pointer()).flatten()
    }
    pub fn load_cursor_image(&self)->WlSurface {
        unimplemented!()
        //CursorTheme::load(24, self.)
    }


    pub fn create_output(&mut self, output_id: usize, name: impl Into<String>, physical_properties: PhysicalProperties) {
        let output = Output::new(&mut self.display,name.into(),physical_properties,None);
        self.output_globals.insert(output_id,output);
    }
    pub fn destroy_output(&mut self, output_id: usize) {
        self.output_globals.remove(&output_id);
    }
    pub fn list_outputs(&self)->impl Iterator<Item=&Output>{
        self.output_globals.values().map(|(output,global)|output)
    }

    #[cfg(feature="xdg_shell")]
    pub fn set_configure_callback(&mut self){

    }
}

impl std::os::unix::io::AsRawFd for EmbeddedWaylandServer {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {self.display.get_poll_fd()}
}
