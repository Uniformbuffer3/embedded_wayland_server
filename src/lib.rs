#[cfg(test)]
mod tests;

pub mod definitions;
pub use definitions::*;

use std::time::Duration;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DispatchContext {
    requests: Vec<Request>,
}
impl DispatchContext {
    pub fn new()->Self {
        let requests = Vec::new();

        Self {
            requests,
        }
    }
}

pub struct EmbeddedWaylandServer {
    dispatch_context: Rc<RefCell<DispatchContext>>,
    display: Display,

    compositor_global: Global<WlCompositor>,
    subcompositor_global: Global<WlSubcompositor>,

    seat_globals: HashMap<String,(Seat,Global<WlSeat>)>,
    output_globals: HashMap<String,(Output,Global<WlOutput>)>,

    #[cfg(feature="shm")]
    shm_global: Global<WlShm>,

    #[cfg(feature="xdg_shell")]
    xdg_shell_state: Arc<Mutex<XdgShellState>>,
    #[cfg(feature="xdg_shell")]
    xdg_wm_base_global: Global<XdgWmBase>,

    #[cfg(feature="dma_buf")]
    zwp_linux_dmabuf_v1_global: Global<ZwpLinuxDmabufV1>,

    #[cfg(feature="drag_and_drop")]
    drag_and_drop_global: Global<WlDataDeviceManager>,

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
            with_states(&surface,|surface_data|{
                let pending = surface_data.cached_state.pending::<SurfaceAttributes>();
                println!("From init_compositor callback {:#?}: {:#?}",surface,pending);

                if surface_data.cached_state.has::<SurfaceAttributes>() {
                    println!("Current: {:#?}",surface_data.cached_state.current::<SurfaceAttributes>());
                }
                else{println!("Current: None")}
            }).unwrap();

            let dispatch_context: &mut Rc<RefCell<DispatchContext>> = dispatch_data.get().unwrap();
            dispatch_context.borrow_mut().requests.push(Request::Commit(surface));
        },None);

        #[cfg(feature="shm")]
        let shm_global = smithay::wayland::shm::init_shm_global(&mut display,parameters.shm_formats,None);

        #[cfg(feature="xdg_shell")]
        let (xdg_shell_state,xdg_wm_base_global) = smithay::wayland::shell::xdg::xdg_shell_init(&mut display,|request,mut dispatch_data|{
            match &request {
                XdgRequest::NewToplevel {
                    surface
                }=>{
                    add_commit_hook(surface.get_surface().unwrap(),|surface|{
                        with_states(&surface,|surface_data|{
                            let pending = surface_data.cached_state.pending::<SurfaceAttributes>();
                            println!("From the commit hook for {:#?}: {:#?}",surface,pending);

                            if surface_data.cached_state.has::<SurfaceAttributes>() {
                                println!("Current: {:#?}",surface_data.cached_state.current::<SurfaceAttributes>());
                            }
                            else{println!("Current: None")}
                        }).unwrap();
                    });

                    surface.with_pending_state(|surface_state|{
                        surface_state.size = Some((400,400).into());
                    }).unwrap();
                    surface.send_configure();

                }
                XdgRequest::AckConfigure{
                    surface: _,
                    configure: Configure::Toplevel(_configure),
                    ..
                }=>{

                }
                _=>{}
            }



            let dispatch_context: &mut Rc<RefCell<DispatchContext>> = dispatch_data.get().unwrap();
            dispatch_context.borrow_mut().requests.push(Request::XdgRequest(request));
        },None);

        #[cfg(feature="dma_buf")]
        let zwp_linux_dmabuf_v1_global = smithay::wayland::dmabuf::init_dmabuf_global(&mut display,parameters.drm_formats,|dmabuf,mut dispatch_data|{
            let dispatch_context: &mut Rc<RefCell<DispatchContext>> = dispatch_data.get().unwrap();
            dispatch_context.borrow_mut().requests.push(Request::Dmabuf(dmabuf.clone()));
            true
        },None);

        #[cfg(feature="drag_and_drop")]
        let drag_and_drop_global = init_data_device(
            &mut display,
            |dnd_event| {

            },
            default_action_chooser,
            None
        );

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

            #[cfg(feature="drag_and_drop")]
            drag_and_drop_global,

            #[cfg(feature="explicit_synchronization")]
            explicit_synchronization_global,
        }
    }

    pub fn dispatch(&mut self, duration: Duration)->Vec<Request>{
        match self.display.dispatch(duration,&mut self.dispatch_context){
            Ok(())=>{}
            Err(_err)=>{}
        }
        self.display.flush_clients(&mut self.dispatch_context);
        self.dispatch_context.borrow_mut().requests.drain(..).collect()
    }

    pub fn create_seat(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.seat_globals.insert(name.clone(),Seat::new(&mut self.display,name,None));
    }
    pub fn destroy_seat(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.seat_globals.remove(&name);
    }
    pub fn list_seats(&self)->impl Iterator<Item=&String>{
        self.seat_globals.keys()
    }

    pub fn add_keyboard(&mut self,
        seat_name: impl Into<String>,
        repeat_delay: i32,
        repeat_rate: i32
    ) {
        let name = seat_name.into();
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&name){
            if seat.get_keyboard().is_none(){
                let dispatch_context = self.dispatch_context.clone();

                seat.add_keyboard(
                    XkbConfig::default(),
                    repeat_delay,
                    repeat_rate,
                    move|seat,focus|{
                        dispatch_context.borrow_mut().requests.push(Request::Seat{
                            seat: seat.clone(),
                            request: SeatRequest::KeaybordFocus(focus.cloned())
                        });
                    }
                ).unwrap();
            }
        }
    }

    pub fn add_cursor(&mut self, name: impl Into<String>) {
        let name = name.into();
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&name){
            if seat.get_pointer().is_none(){
                let dispatch_context = self.dispatch_context.clone();
                let seat_cloned = seat.clone();
                seat.add_pointer(move |cursor_image_status|{
                    let seat_cloned = &seat_cloned;
                    dispatch_context.borrow_mut().requests.push(Request::Seat{
                        seat: seat_cloned.clone(),
                        request: SeatRequest::CursorImage(cursor_image_status)
                    });
                });
            }
        }
    }
    pub fn del_cursor(&mut self, name: impl Into<String>) {
        let name = name.into();
        if let Some((seat,_seat_global)) = self.seat_globals.get_mut(&name){
            seat.remove_pointer();
        }
    }


    pub fn create_output(&mut self,name: impl Into<String>, physical_properties: PhysicalProperties) {
        let name = name.into();
        self.output_globals.insert(name.clone(),Output::new(&mut self.display,name,physical_properties,None));
    }
    pub fn destroy_output(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.output_globals.remove(&name);
    }
}


