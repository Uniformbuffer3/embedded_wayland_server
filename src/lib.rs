#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub mod definitions;
pub use definitions::*;

mod destruction_filter;
mod instantiation_filter;

use std::time::Duration;
//use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use slab::Slab;

pub struct DispatchContext {
    request_filter: Filter<WaylandRequest>,
    destruction_filter: Filter<Destruction>,
    events: Vec<WaylandRequest>,
}
impl DispatchContext {
    pub fn new(request_filter: Filter<WaylandRequest>,destruction_filter: Filter<Destruction>)->Self {
        let events = Vec::new();

        Self {
            request_filter,
            events,
            destruction_filter,
        }
    }
}

pub struct EmbeddedWaylandServer {
    instantiation_filter: Filter<Instantiation>,
    dispatch_context: DispatchContext,
    display: Display,

    clients: Rc<RefCell<Vec<Client>>>,

    seat_globals: Slab<Global<WlSeat>>,
    output_globals: Slab<Global<WlOutput>>,

    compositor_global: Global<WlCompositor>,
    subcompositor_global: Global<WlSubcompositor>,
    shell_global: Global<WlShell>,

    #[cfg(feature="shm")]
    shm_global: Global<WlShm>,
    #[cfg(feature="shm")]
    shm_pool_global: Global<WlShmPool>,

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

impl EmbeddedWaylandServer {
    pub fn new()->Self
    {
        let mut display = wayland_server::Display::new();
        display.add_socket_auto().expect("Failed to bind wayland socket");

        // Clients
        let clients = Rc::new(RefCell::new(Vec::new()));
        let clients_clone = clients.clone();
        let client_callback = move |client: Client|{
            client.data_map().insert_if_missing(||RefCell::new(ClientResources::default()));
            clients_clone.borrow_mut().push(client);
            true
        };

        // Filters
        let instantiation_filter = instantiation_filter::filter();
        let destruction_filter = destruction_filter::filter();
        let request_filter: Filter<WaylandRequest> = Filter::new(move |event,_filter, mut dispatch_data|{
            let dispatch_context: &mut DispatchContext = dispatch_data.get().unwrap();
            dispatch_context.events.push(event);
        });

        let dispatch_context = DispatchContext::new(request_filter.clone(),destruction_filter);

        let seat_globals: Slab<Global<WlSeat>> = Slab::new();
        let output_globals: Slab<Global<WlOutput>> = Slab::new();

        let compositor_global: Global<WlCompositor> = display.create_global_with_filter(4,instantiation_filter.clone(),client_callback.clone());
        let subcompositor_global: Global<WlSubcompositor> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());
        let shell_global: Global<WlShell> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());

        #[cfg(feature="shm")]
        let shm_global: Global<WlShm> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());
        #[cfg(feature="shm")]
        let shm_pool_global: Global<WlShmPool> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());

        #[cfg(feature="xdg_shell")]
        let xdg_wm_base_global: Global<XdgWmBase> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_surface_global: Global<XdgSurface> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_popup_global: Global<XdgPopup> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_positioner_global: Global<XdgPositioner> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());
        #[cfg(feature="xdg_shell")]
        let xdg_toplevel_global: Global<XdgToplevel> = display.create_global_with_filter(1,instantiation_filter.clone(),client_callback.clone());

        Self {
            display,
            clients,

            instantiation_filter,
            dispatch_context,

            seat_globals,
            output_globals,

            compositor_global,
            subcompositor_global,
            shell_global,

            #[cfg(feature="shm")]
            shm_global,
            #[cfg(feature="shm")]
            shm_pool_global,

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
        let global: Global<WlSeat> = self.display.create_global(1,self.instantiation_filter.clone());
        self.seat_globals.insert(global) as u32
    }
    pub fn destroy_seat(&mut self, id: u32) {
        let id = id as usize;
        self.seat_globals.remove(id);
    }

    pub fn create_output(&mut self)->u32 {
        let global: Global<WlOutput> = self.display.create_global(1,self.instantiation_filter.clone());
        self.output_globals.insert(global) as u32
    }
    pub fn destroy_output(&mut self, id: u32) {
        let id = id as usize;
        if self.output_globals.contains(id){self.output_globals.remove(id);}
    }

    pub fn create_shm_pool(&mut self)->u32 {
        let global: Global<WlOutput> = self.display.create_global(1,self.instantiation_filter.clone());
        self.output_globals.insert(global) as u32
    }
    pub fn destroy_shm_pool(&mut self, id: u32) {
        let id = id as usize;
        if self.output_globals.contains(id){self.output_globals.remove(id);}
    }
}


