#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub mod definitions;
pub use definitions::*;

use std::time::Duration;
use std::collections::HashMap;

pub struct DispatchContext {
    wayland_filter: Filter<WaylandRequest>,
    events: Vec<WaylandRequest>,
    seats: HashMap<u32,Seat>,
    outputs: HashMap<u32,Output>
}
impl DispatchContext {
    pub fn new(wayland_filter: Filter<WaylandRequest>)->Self {
        let events = Vec::new();
        let seats = HashMap::new();
        let outputs = HashMap::new();
        Self {wayland_filter,events,seats,outputs}
    }
}

pub struct EmbeddableWaylandServer {
    global_instantiation_filter: Filter<GlobalInstantiation>,
    dispatch_context: DispatchContext,
    display: Display,

    compositor_global: Global<WlCompositor>,
    subcompositor_global: Global<WlSubcompositor>,
    shm_global: Global<WlShm>,
    shell_global: Global<WlShell>,
    #[cfg(feature="xdg_shell")]
    xdg_wm_base_global: Global<XdgWmBase>,
    #[cfg(feature="xdg_shell")]
    xdg_surface_global: Global<XdgSurface>,
    #[cfg(feature="xdg_shell")]
    xdg_popup_global: Global<XdgPopup>,
    #[cfg(feature="xdg_shell")]
    xdg_positioner_global: Global<XdgPositioner>,
    #[cfg(feature="xdg_shell")]
    xdg_toplevel_global: Global<XdgToplevel>,
}

impl EmbeddableWaylandServer {
    pub fn new()->Self
    {
        let mut display = wayland_server::Display::new();
        display.add_socket_auto().expect("Failed to bind wayland socket");

        let global_instantiation_filter = Filter::new(|event: GlobalInstantiation,_filter,mut dispatch_data|{
            let dispatch_context: &mut DispatchContext = dispatch_data.get().unwrap();
            match event {
                GlobalInstantiation::CompositorInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::SubcompositorInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::ShmInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::SeatInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::PointerInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::KeyboardInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::TouchInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::OutputInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                GlobalInstantiation::ShellInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                #[cfg(feature="xdg_shell")]
                GlobalInstantiation::XdgWmBaseInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                #[cfg(feature="xdg_shell")]
                GlobalInstantiation::XdgSurfaceInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                #[cfg(feature="xdg_shell")]
                GlobalInstantiation::XdgPopupInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                #[cfg(feature="xdg_shell")]
                GlobalInstantiation::XdgPositionerInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
                #[cfg(feature="xdg_shell")]
                GlobalInstantiation::XdgToplevelInstantiation((handle,_version))=>handle.assign(dispatch_context.wayland_filter.clone()),
            }
        });

        let global_instantiation_filter_clone = global_instantiation_filter.clone();
        let wayland_filter: Filter<WaylandRequest> = Filter::new(move |event,_filter, mut dispatch_data|{
            let instantiation_filter = &global_instantiation_filter_clone;
            let dispatch_context: &mut DispatchContext = dispatch_data.get().unwrap();
            match event {
                WaylandRequest::Seat{ref request, ref object} => {
                    let global_instantiation = match request {
                        wl_seat::Request::GetPointer{id}=>{
                            if let Some(seat) = dispatch_context.seats.get_mut(&object.as_ref().id()){
                                seat.pointers.push(id.clone());
                            }
                            Some(GlobalInstantiation::PointerInstantiation((object.clone(),id.clone())))
                        }
                        wl_seat::Request::GetKeyboard{id}=>{
                            if let Some(seat) = dispatch_context.seats.get_mut(&object.as_ref().id()){
                                seat.keyboards.push(id.clone());
                            }
                            Some(GlobalInstantiation::KeyboardInstantiation((object.clone(),id.clone())))
                        }
                        wl_seat::Request::GetTouch{id}=>{
                            if let Some(seat) = dispatch_context.seats.get_mut(&object.as_ref().id()){
                                seat.touchs.push(id.clone());
                            }
                            Some(GlobalInstantiation::TouchInstantiation((object.clone(),id.clone())))
                        }
                        _=>{None}
                    };
                    if let Some(global_instantiation) = global_instantiation {
                        instantiation_filter.send(global_instantiation,dispatch_data);
                    }
                }
                _=>{dispatch_context.events.push(event);}
            }
        });

        let dispatch_context = DispatchContext::new(wayland_filter.clone());

        let compositor_global: Global<WlCompositor> = display.create_global(4,global_instantiation_filter.clone());
        let subcompositor_global: Global<WlSubcompositor> = display.create_global(1,global_instantiation_filter.clone());
        let shm_global: Global<WlShm> = display.create_global(1,global_instantiation_filter.clone());
        let shell_global: Global<WlShell> = display.create_global(1,global_instantiation_filter.clone());

        #[cfg(feature="xdg_shell")]
        let xdg_wm_base_global: Global<XdgWmBase> = display.create_global(1,global_instantiation_filter.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_surface_global: Global<XdgSurface> = display.create_global(1,global_instantiation_filter.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_popup_global: Global<XdgPopup> = display.create_global(1,global_instantiation_filter.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_positioner_global: Global<XdgPositioner> = display.create_global(1,global_instantiation_filter.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_toplevel_global: Global<XdgToplevel> = display.create_global(1,global_instantiation_filter.clone());

        Self {
            global_instantiation_filter,
            dispatch_context,

            display,
            compositor_global,
            subcompositor_global,
            shm_global,
            shell_global,

            #[cfg(feature="xdg_shell")]
            xdg_wm_base_global,
            #[cfg(feature="xdg_shell")]
            xdg_surface_global,
            #[cfg(feature="xdg_shell")]
            xdg_popup_global,
            #[cfg(feature="xdg_shell")]
            xdg_positioner_global,
            #[cfg(feature="xdg_shell")]
            xdg_toplevel_global,
        }
    }

    pub fn dispatch(&mut self, duration: Duration)->Vec<WaylandRequest>{
        self.display.flush_clients(&mut self.dispatch_context);

        match self.display.dispatch(duration,&mut self.dispatch_context){
            Ok(())=>{}
            Err(_err)=>{}
        }

        self.dispatch_context.events.drain(..).collect()
    }

    pub fn create_seat(&mut self)->u32 {
        let global: Global<WlSeat> = self.display.create_global(1,self.global_instantiation_filter.clone());
        let seat = Seat {
            global,
            pointers: Vec::new(),
            keyboards: Vec::new(),
            touchs: Vec::new(),
        };
        let id = 0; //TODO Get the id of the underlying resource of the global
        self.dispatch_context.seats.insert(id,seat);
        id
    }
    pub fn destroy_seat(&mut self, id: u32) {
        if let Some(seat) = self.dispatch_context.seats.remove(&id){
            seat.global.destroy();
        }
    }
    pub fn get_seat(&self,id: u32)->Option<&Seat>{
        self.dispatch_context.seats.get(&id)
    }
    pub fn list_seats(&self)->impl Iterator<Item=(&u32,&Seat)>{
        self.dispatch_context.seats.iter()
    }

    pub fn create_output(&mut self)->u32 {
        let global: Global<WlOutput> = self.display.create_global(1,self.global_instantiation_filter.clone());
        let output = Output {
            global
        };
        let id = 0; //TODO Get the id of the underlying resource of the global
        self.dispatch_context.outputs.insert(id,output);
        id
    }
    pub fn destroy_output(&mut self, id: u32) {
        if let Some(output) = self.dispatch_context.outputs.remove(&id){
            output.global.destroy();
        }
    }
    pub fn get_output(&self,id: u32)->Option<&Output>{
        self.dispatch_context.outputs.get(&id)
    }
    pub fn list_outputs(&self)->impl Iterator<Item=(&u32,&Output)>{
        self.dispatch_context.outputs.iter()
    }
}


