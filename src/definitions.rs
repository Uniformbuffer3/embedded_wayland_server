pub use smithay::{
    backend::{
        allocator::{
            dmabuf::Dmabuf, Buffer, Format as DrmFormat, Fourcc as DrmFourcc,
            Modifier as DrmModifier,
        },
        input::KeyState,
        renderer::{buffer_type, BufferType},
    },
    reexports::{
        wayland_commons::user_data::UserDataMap,
        wayland_protocols::{
            unstable::{
                linux_dmabuf::v1::server::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1,
                linux_explicit_synchronization::v1::server::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1,
                xdg_shell::v6::server::zxdg_shell_v6::ZxdgShellV6,
            },
            xdg_shell::server::{
                xdg_positioner::{Anchor, ConstraintAdjustment, Gravity},
                xdg_toplevel::{ResizeEdge, State as SurfaceState},
                xdg_wm_base::XdgWmBase,
            },
        },
        wayland_server::{
            protocol::{
                wl_buffer::WlBuffer,
                wl_compositor::WlCompositor,
                wl_data_device_manager::WlDataDeviceManager,
                wl_keyboard::WlKeyboard,
                wl_output::{Subpixel, WlOutput},
                wl_pointer::{Axis, AxisSource, ButtonState, WlPointer},
                wl_seat::WlSeat,
                wl_shell::WlShell,
                wl_shm::{Format as ShmFormat, WlShm},
                wl_shm_pool::WlShmPool,
                wl_subcompositor::WlSubcompositor,
                wl_subsurface::WlSubsurface,
                wl_surface::WlSurface,
                wl_touch::WlTouch,
                *,
            },
            Client, Display, Filter, Global, Interface, Main, Resource,
        },
    },
    utils::{DeadResource, Logical, Point, Rectangle, Size},
    wayland::{
        compositor::*,
        data_device::DataDeviceEvent,
        data_device::{default_action_chooser, init_data_device},
        explicit_synchronization::init_explicit_synchronization_global,
        output::{Output, PhysicalProperties},
        seat::{
            AxisFrame, CursorImageStatus, FilterResult, GrabStartData, KeyboardHandle, PointerGrab,
            PointerHandle, PointerInnerHandle, Seat, XkbConfig,
        },
        shell::xdg::{
            Configure, PopupSurface, ShellState as XdgShellState, SurfaceCachedState,
            ToplevelState, ToplevelStateSet, ToplevelSurface, XdgRequest,
        },
        shm::{with_buffer_contents, BufferData},
        Serial, SERIAL_COUNTER,
    },
};

pub use wayland_cursor::{Cursor, CursorTheme};

pub use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum SeatRequest {
    CursorImage(CursorImageStatus),
    KeaybordFocus(Option<WlSurface>),
}

#[derive(Debug)]
pub enum WaylandRequest {
    Seat {
        seat: Seat,
        request: SeatRequest,
    },
    SurfaceRemoved {
        id: usize,
    },
    Commit {
        surface: WlSurface,
    },
    #[cfg(feature = "xdg_shell")]
    XdgRequest {
        request: XdgRequest,
    },
    #[cfg(feature = "dma_buf")]
    Dmabuf {
        buffer: Dmabuf,
    },
    #[cfg(feature = "dnd")]
    Dnd {
        dnd: DataDeviceEvent,
    },
}

#[derive(Default)]
pub struct Parameters {
    #[cfg(feature = "shm")]
    pub shm_formats: Vec<ShmFormat>,
    #[cfg(feature = "dma_buf")]
    pub drm_formats: Vec<DrmFormat>,
}

#[derive(Debug, Clone, Copy)]
pub struct ClientId(pub usize);
impl From<ClientId> for usize {
    fn from(id: ClientId) -> Self {
        id.0
    }
}
impl From<ClientId> for u32 {
    fn from(id: ClientId) -> Self {
        id.0 as u32
    }
}
impl From<u32> for ClientId {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}
impl From<usize> for ClientId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SeatId(pub usize);
impl From<SeatId> for usize {
    fn from(id: SeatId) -> Self {
        id.0
    }
}
impl From<SeatId> for u32 {
    fn from(id: SeatId) -> Self {
        id.0 as u32
    }
}
impl From<u32> for SeatId {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}
impl From<usize> for SeatId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SurfaceId(pub usize);
impl From<SurfaceId> for usize {
    fn from(id: SurfaceId) -> Self {
        id.0
    }
}
impl From<SurfaceId> for u32 {
    fn from(id: SurfaceId) -> Self {
        id.0 as u32
    }
}
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}
impl From<usize> for SurfaceId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub enum SurfaceKind {
    Toplevel(ToplevelSurface),
    Popup(PopupSurface),
    Cursor,
}
impl From<ToplevelSurface> for SurfaceKind {
    fn from(handle: ToplevelSurface) -> Self {
        Self::Toplevel(handle)
    }
}
impl From<PopupSurface> for SurfaceKind {
    fn from(handle: PopupSurface) -> Self {
        Self::Popup(handle)
    }
}

pub fn seat_id(seat: &WlSeat) -> Option<usize> {
    if let Some(seat) = Seat::from_resource(seat) {
        if let Some(seat_id) = seat.user_data().get::<SeatId>() {
            Some((*seat_id).into())
        } else {
            log::error!(target: "EWS","seat_id: Cannot retrieve SeatId from {:#?}",seat);
            None
        }
    } else {
        log::error!(target: "EWS","seat_id: Cannot retrieve Seat from {:#?}",seat);
        None
    }
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
pub fn surface_id(surface_data: &SurfaceData) -> Option<usize> {
    if let Some(surface_id) = surface_data.data_map.get::<SurfaceId>() {
        Some((*surface_id).into())
    } else {
        log::error!(target: "EWS","surface_id: Cannot retrieve SurfaceId from surface data");
        None
    }
}

pub fn surface_kind(surface_data: &SurfaceData) -> Option<&SurfaceKind> {
    if let Some(kind) = surface_data.data_map.get::<SurfaceKind>() {
        Some(kind)
    } else {
        log::error!(target: "EWS","surface_id: Cannot retrieve SurfaceId from surface data");
        None
    }
}
