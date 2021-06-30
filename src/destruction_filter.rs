use crate::definitions::*;
use std::cell::RefCell;

pub fn filter()->Filter<Destruction>{
    Filter::new(|event: Destruction,_filter,_dispatch_data|{
        match &event {
            Destruction::Compositor(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().compositor = None;
                }
            },
            Destruction::Subcompositor(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().subcompositor = None;
                }
            },
            Destruction::Shell(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.shells.iter().position(|shell|shell.as_ref() == resource){
                        resources.shells.remove(index);
                    }
                }
            },
            Destruction::Seat(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(seat_index) = resources.seats.iter().position(|seat|seat.handle.as_ref() == resource){
                        resources.seats.remove(seat_index);
                    }
                }
            },
            Destruction::Pointer(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    for seat in &mut resources.seats {
                        if let Some(index) = seat.pointers.iter().position(|pointer|pointer.as_ref() == resource){
                            seat.pointers.remove(index);
                            break;
                        }
                    }
                }
            },
            Destruction::Keyboard(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    for seat in &mut resources.seats {
                        if let Some(index) = seat.keyboards.iter().position(|keyboard|keyboard.as_ref() == resource){
                            seat.keyboards.remove(index);
                            break;
                        }
                    }
                }
            },
            Destruction::Touch(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    for seat in &mut resources.seats {
                        if let Some(index) = seat.touchs.iter().position(|touch|touch.as_ref() == resource){
                            seat.touchs.remove(index);
                            break;
                        }
                    }
                }
            },
            Destruction::Output(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.outputs.iter().position(|output|output.as_ref() == resource){
                        resources.outputs.remove(index);
                    }
                }
            },
            #[cfg(feature="shm")]
            Destruction::Shm(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().shm = None;
                }
            },
            #[cfg(feature="shm")]
            Destruction::ShmPool(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.shm_pools.iter().position(|shm_pool|shm_pool.as_ref() == resource){
                        resources.shm_pools.remove(index);
                    }
                }
            },
            #[cfg(feature="xdg_shell")]
            Destruction::XdgWmBase(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    resources.borrow_mut().xdg_wm_base = None;
                }
            },
            #[cfg(feature="xdg_shell")]
            Destruction::XdgSurface(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.xdg_surfaces.iter().position(|xdg_surface|xdg_surface.as_ref() == resource){
                        resources.xdg_surfaces.remove(index);
                    }
                }
            },
            #[cfg(feature="xdg_shell")]
            Destruction::XdgPopup(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.xdg_popups.iter().position(|xdg_popup|xdg_popup.as_ref() == resource){
                        resources.xdg_popups.remove(index);
                    }
                }
            },
            #[cfg(feature="xdg_shell")]
            Destruction::XdgPositioner(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.xdg_positioners.iter().position(|xdg_positioner|xdg_positioner.as_ref() == resource){
                        resources.xdg_positioners.remove(index);
                    }
                }
            },
            #[cfg(feature="xdg_shell")]
            Destruction::XdgToplevel(resource)=>{
                if let Some(client) = resource.client() {
                    let resources: &RefCell<ClientResources> = client.data_map().get().unwrap();
                    let mut resources = resources.borrow_mut();
                    if let Some(index) = resources.xdg_top_levels.iter().position(|xdg_top_level|xdg_top_level.as_ref() == resource){
                        resources.xdg_top_levels.remove(index);
                    }
                }
            },
        };
    })
}
