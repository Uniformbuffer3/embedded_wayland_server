pub use smithay::{
    reexports::{
        wayland_server::{
            Main,Resource,Interface,Display,Filter,Global,Client,
            protocol::{
                wl_compositor::WlCompositor,
                wl_subcompositor::WlSubcompositor,
                wl_subsurface::WlSubsurface,
                wl_shm::{WlShm,Format as ShmFormat},
                wl_shm_pool::WlShmPool,
                wl_seat::WlSeat,
                wl_output::{WlOutput,Subpixel},
                wl_surface::WlSurface,
                wl_shell::WlShell,
                wl_pointer::{WlPointer,ButtonState},
                wl_keyboard::WlKeyboard,
                wl_touch::WlTouch,
                wl_data_device_manager::WlDataDeviceManager,
                *
            }
        },
        wayland_commons::user_data::UserDataMap,
        wayland_protocols::{
            xdg_shell::server::{
                xdg_wm_base::XdgWmBase,
                xdg_toplevel::{
                    ResizeEdge,
                    State as SurfaceState,
                },
            },
            unstable::{
                xdg_shell::v6::server::zxdg_shell_v6::ZxdgShellV6,
                linux_dmabuf::v1::server::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1,
                linux_explicit_synchronization::v1::server::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1,
            }
        },
    },
    wayland::{
        compositor::*,
        shell::xdg::{
            XdgRequest,
            ShellState as XdgShellState,
            Configure,
            ToplevelState,
            ToplevelStateSet,
            SurfaceCachedState
        },
        shm::{BufferData,with_buffer_contents},
        output::{Output,PhysicalProperties},
        seat::{Seat,CursorImageStatus,XkbConfig,KeyboardHandle,PointerHandle,PointerInnerHandle,PointerGrab,GrabStartData,AxisFrame,FilterResult},
        data_device::{init_data_device, default_action_chooser},
        explicit_synchronization::init_explicit_synchronization_global,
        data_device::DataDeviceEvent,
        Serial,
        SERIAL_COUNTER
    },
    backend::{
        allocator::{
            Format as DrmFormat,
            dmabuf::Dmabuf,
            Fourcc as DrmFourcc,
            Modifier as DrmModifier,
            Buffer
        },
        input::KeyState,
        renderer::{
            buffer_type,
            BufferType
        },
    },
    utils::{DeadResource,Point,Logical}
};

pub use wayland_cursor::{
    Cursor,
    CursorTheme
};

pub use std::sync::{Arc,Mutex};

#[derive(Debug)]
pub enum SeatRequest {
    CursorImage(CursorImageStatus),
    KeaybordFocus(Option<WlSurface>)
}

#[derive(Debug)]
pub enum WaylandRequest {
    Seat{seat: Seat, request: SeatRequest},
    Commit{surface: WlSurface},
    #[cfg(feature="xdg_shell")]
    XdgRequest{request: XdgRequest},
    #[cfg(feature="dma_buf")]
    Dmabuf{buffer: Dmabuf},
    #[cfg(feature="dnd")]
    Dnd{dnd: DataDeviceEvent}
}

#[derive(Default)]
pub struct Parameters {
    #[cfg(feature="shm")]
    pub shm_formats: Vec<ShmFormat>,
    #[cfg(feature="dma_buf")]
    pub drm_formats: Vec<DrmFormat>,
}

#[derive(Debug, Clone, Copy)]
pub struct ClientId(pub usize);
impl From<ClientId> for usize {
    fn from(id: ClientId) -> Self {id.0}
}
impl From<ClientId> for u32 {
    fn from(id: ClientId) -> Self {id.0 as u32}
}
impl From<u32> for ClientId {
    fn from(id: u32) -> Self {Self(id as usize)}
}
impl From<usize> for ClientId {
    fn from(id: usize) -> Self {Self(id)}
}

#[derive(Debug, Clone, Copy)]
pub struct SeatId(pub usize);
impl From<SeatId> for usize {
    fn from(id: SeatId) -> Self {id.0}
}
impl From<SeatId> for u32 {
    fn from(id: SeatId) -> Self {id.0 as u32}
}
impl From<u32> for SeatId {
    fn from(id: u32) -> Self {Self(id as usize)}
}
impl From<usize> for SeatId {
    fn from(id: usize) -> Self {Self(id)}
}

#[derive(Debug, Clone, Copy)]
pub struct SurfaceId(pub usize);
impl From<SurfaceId> for usize {
    fn from(id: SurfaceId) -> Self {id.0}
}
impl From<SurfaceId> for u32 {
    fn from(id: SurfaceId) -> Self {id.0 as u32}
}
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {Self(id as usize)}
}
impl From<usize> for SurfaceId {
    fn from(id: usize) -> Self {Self(id)}
}


pub fn seat_id(seat: &WlSeat)->Option<usize>{
    if let Some(seat) = Seat::from_resource(seat) {
        if let Some(seat_id) = seat.user_data().get::<SeatId>() {
            Some((*seat_id).into())
        }
        else{log::error!(target: "EWS","seat_id: Cannot retrieve SeatId from {:#?}",seat);None}
    }
    else{log::error!(target: "EWS","seat_id: Cannot retrieve Seat from {:#?}",seat);None}
}
/*
pub fn cursor_id(seat: &WlSeat)->Option<usize>{
    if let Some(seat) = Seat::from_resource(seat) {
        if let Some(cursor_id) = seat.user_data().get::<std::cell::Cell<Option<SurfaceId>>>() {
            cursor_id.get().map(|cursor_id|cursor_id.into())
        }
        else{
            panic!("cursor_id: Cannot retrieve cursor SurfaceId from {:#?}",seat);
            log::error!(target: "EWS","cursor_id: Cannot retrieve cursor SurfaceId from {:#?}",seat);
            None
        }
    }
    else{
        panic!("cursor_id: Cannot retrieve Seat from {:#?}",seat);
        log::error!(target: "EWS","cursor_id: Cannot retrieve Seat from {:#?}",seat);
        None
    }
}
*/
pub fn surface_id(surface_data: &SurfaceData)->Option<usize>{
    if let Some(surface_id) = surface_data.data_map.get::<SurfaceId>(){
        Some((*surface_id).into())
    }
    else{log::error!(target: "EWS","surface_id: Cannot retrieve SurfaceId from surface data");None}
}
