pub use smithay::{
    reexports::{
        wayland_server::{
            Main,Resource,Interface,Display,Filter,Global,Client,protocol::{
                wl_compositor::WlCompositor,
                wl_subcompositor::WlSubcompositor,
                wl_subsurface::WlSubsurface,
                wl_shm::{WlShm,Format as ShmFormat},
                wl_shm_pool::WlShmPool,
                wl_seat::WlSeat,
                wl_output::{WlOutput,Subpixel},
                wl_surface::WlSurface,
                wl_shell::WlShell,
                wl_pointer::WlPointer,
                wl_keyboard::WlKeyboard,
                wl_touch::WlTouch,
                wl_data_device_manager::WlDataDeviceManager,
                *
            }
        },
        wayland_commons::user_data::UserDataMap,
        wayland_protocols::{
            xdg_shell::server::xdg_wm_base::XdgWmBase,
            unstable::{
                xdg_shell::v6::server::zxdg_shell_v6::ZxdgShellV6,
                linux_dmabuf::v1::server::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1,
                linux_explicit_synchronization::v1::server::zwp_linux_explicit_synchronization_v1::ZwpLinuxExplicitSynchronizationV1,
            }
        }
    },
    wayland::{
        compositor::*,
        shell::xdg::{XdgRequest,
            ShellState as XdgShellState,
            Configure
        },
        output::{Output,PhysicalProperties},
        seat::{Seat,CursorImageStatus,XkbConfig},
        data_device::{init_data_device, default_action_chooser},
        explicit_synchronization::init_explicit_synchronization_global
    },
    backend::allocator::{Format as DrmFormat,dmabuf::Dmabuf},
    utils::DeadResource
};

pub use std::sync::{Arc,Mutex};

#[derive(Debug)]
pub enum SeatRequest {
    CursorImage(CursorImageStatus),
    KeaybordFocus(Option<WlSurface>)
}

#[derive(Debug)]
pub enum Request {
    Seat{seat: Seat, request: SeatRequest},
    Commit(WlSurface),
    XdgRequest(XdgRequest),
    Dmabuf(Dmabuf)
}

#[derive(Default)]
pub struct Parameters {
    #[cfg(feature="shm")]
    pub shm_formats: Vec<ShmFormat>,
    #[cfg(feature="dma_buf")]
    pub drm_formats: Vec<DrmFormat>
}
