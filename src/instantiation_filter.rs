use crate::definitions::*;
use std::cell::RefCell;
use crate::DispatchContext;

pub fn filter()->Filter<Instantiation>{
    Filter::new(|event: Instantiation,_filter,mut dispatch_data|{
        let dispatch_context: &mut DispatchContext = dispatch_data.get().unwrap();
        match &event {
            Instantiation::Compositor((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().compositor = Some(handle.clone());
                }
            },
            Instantiation::Subcompositor((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().subcompositor = Some(handle.clone());
                }
            },
            Instantiation::Shell((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().shells.push(handle.clone());
                }
            },
            Instantiation::Seat((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().seats.push(Seat::new(handle.clone()));
                }
            },
            Instantiation::Pointer((seat,handle))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    if let Some(seat) = resources.borrow_mut().seats.iter_mut().find(|current_seat|&current_seat.handle==seat) {
                        seat.pointers.push(handle.clone());
                    }
                }
            },
            Instantiation::Keyboard((seat,handle))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    if let Some(seat) = resources.borrow_mut().seats.iter_mut().find(|current_seat|&current_seat.handle==seat) {
                        seat.keyboards.push(handle.clone());
                    }
                }
            },
            Instantiation::Touch((seat,handle))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    if let Some(seat) = resources.borrow_mut().seats.iter_mut().find(|current_seat|&current_seat.handle==seat) {
                        seat.touchs.push(handle.clone());
                    }
                }
            },
            Instantiation::Output((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().outputs.push(handle.clone());
                }
            },
            #[cfg(feature="shm")]
            Instantiation::Shm((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().shm = Some(handle.clone());
                }
            },
            #[cfg(feature="shm")]
            Instantiation::ShmPool((handle,_version))=>{
                println!("Called shm pool!");
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().shm_pools.push(handle.clone());
                }
            },
            #[cfg(feature="xdg_shell")]
            Instantiation::XdgWmBase((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().xdg_wm_base = Some(handle.clone());
                }
            },
            #[cfg(feature="xdg_shell")]
            Instantiation::XdgSurface((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().xdg_surfaces.push(handle.clone());
                }
            },
            #[cfg(feature="xdg_shell")]
            Instantiation::XdgPopup((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().xdg_popups.push(handle.clone());
                }
            },
            #[cfg(feature="xdg_shell")]
            Instantiation::XdgPositioner((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().xdg_positioners.push(handle.clone());
                }
            },
            #[cfg(feature="xdg_shell")]
            Instantiation::XdgToplevel((handle,_version))=>{
                handle.assign(dispatch_context.request_filter.clone());
                handle.assign_destructor(dispatch_context.destruction_filter.clone());
                if let Some(client) = handle.as_ref().client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().xdg_top_levels.push(handle.clone());
                }
            },
        };
        dispatch_context.events.push(WaylandRequest::Instantiation(event));
    })
}
