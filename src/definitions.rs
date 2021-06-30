pub use wayland_server::{protocol,Main,Resource,Interface,Display,Filter,Global,Client};
pub use protocol::{
    wl_compositor::WlCompositor,
    wl_subcompositor::WlSubcompositor,
    wl_shm::{WlShm,Format},
    wl_shm_pool::WlShmPool,
    wl_seat::WlSeat,
    wl_output::WlOutput,
    wl_surface::WlSurface,
    wl_shell::WlShell,
    wl_pointer::WlPointer,
    wl_keyboard::WlKeyboard,
    wl_touch::WlTouch,
    *
};

pub use wayland_protocols::xdg_shell::server::{
    xdg_wm_base,
    xdg_wm_base::XdgWmBase,
    xdg_surface,
    xdg_surface::XdgSurface,
    xdg_popup,
    xdg_popup::XdgPopup,
    xdg_positioner,
    xdg_positioner::XdgPositioner,
    xdg_toplevel,
    xdg_toplevel::XdgToplevel
};

/// This is a slightly modified version of the samely named macro in `wayland-rs` of the Smithay project (https://github.com/Smithay/wayland-rs),
/// so it follow the owner licence (https://github.com/Smithay/wayland-rs/blob/master/LICENSE.txt).
/*
Copyright (c) 2015 Victor Berger

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/
/// Generate an enum joining several objects requests
///
/// This macro allows you to easily create a enum type for use with your message Filters. It is
/// used like so:
///
/// ```no_run
/// # use wayland_server::protocol::{wl_surface::WlSurface, wl_keyboard::WlKeyboard, wl_pointer::WlPointer};
/// # use wayland_server::request_enum;
/// request_enum!(
///     MyEnum |
///     Pointer => WlPointer,
///     Keyboard => WlKeyboard,
///     Surface => WlSurface
/// );
/// ```
///
/// This will generate the following enum, unifying the requests from each of the provided interface:
///
/// ```ignore
/// pub enum MyEnum {
///     Pointer { request: WlPointer::Request, object: Main<WlPointer> },
///     Keyboard { request: WlKeyboard::Request, object: Main<WlKeyboard> },
///     Surface { request: WlSurface::Request, object: Main<WlSurface> }
/// }
/// ```
///
/// It will also generate the appropriate `From<_>` implementation so that a `Filter<MyEnum>` can be
/// used as assignation target for `WlPointer`, `WlKeyboard` and `WlSurface`.
///
/// If you want to add custom messages to the enum, the macro also supports it:
///
/// ```no_run
/// # use wayland_server::protocol::{wl_surface::WlSurface, wl_keyboard::WlKeyboard, wl_pointer::WlPointer};
/// # use wayland_server::request_enum;
/// # struct SomeType;
/// # struct OtherType;
/// request_enum!(
///     MyEnum |
///     Pointer => WlPointer,
///     Keyboard => WlKeyboard,
///     Surface => WlSurface |
///     MyMessage => SomeType,
///     OtherMessage => OtherType
/// );
/// ```
///
/// will generate the following enum:
///
/// ```ignore
/// pub enum MyEnum {
///     Pointer { request: WlPointer::Request, object: Main<WlPointer> },
///     Keyboard { request: WlKeyboard::Request, object: Main<WlKeyboard> },
///     Surface { request: WlSurface::Request, object: Main<WlSurface> },
///     MyMessage(SomeType),
///     OtherMessage(OtherType)
/// }
/// ```
///
/// as well as implementations of `From<SomeType>` and `From<OtherType>`, so that these types can
/// directly be provided into a `Filter<MyEnum>`.

#[macro_export]
macro_rules! request_enum(
    ($(#[$attrs:meta])* $enu:ident | $($(#[$member_attrs:meta])* $evt_name:ident => $iface:ty),*) => {
        $crate::request_enum!($(#[$attrs])* $enu | $($(#[$member_attrs])* $evt_name => $iface),* | );
    };
    ($(#[$attrs:meta])* $enu:ident | $($(#[$member_attrs:meta])* $evt_name:ident => $iface:ty),* | $($(#[$member_attrs2:meta])* $name:ident => $value:ty),*) => {
        $(#[$attrs])*
        pub enum $enu {
            $(
                $(#[$member_attrs])*
                $evt_name { request: <$iface as $crate::Interface>::Request, object: $crate::Main<$iface> },
            )*
            $(
                $(#[$member_attrs2])*
                $name($value),
            )*
        }

        $(
            $(#[$member_attrs])*
            impl From<($crate::Main<$iface>, <$iface as $crate::Interface>::Request)> for $enu {
                fn from((object, request): ($crate::Main<$iface>, <$iface as $crate::Interface>::Request)) -> $enu {
                    $enu::$evt_name { request, object }
                }
            }
        )*
        $(
            $(#[$member_attrs2])*
            impl From<$value> for $enu {
                fn from(value: $value) -> $enu {
                    $enu::$name(value)
                }
            }
        )*
    };
);

request_enum!(
    #[derive(Debug)]Instantiation | |
    Compositor => (Main<WlCompositor>,u32),
    Subcompositor => (Main<WlSubcompositor>,u32),
    Shell => (Main<WlShell>,u32),

    Seat => (Main<WlSeat>,u32),
    Pointer => (Main<WlSeat>,Main<WlPointer>),
    Keyboard => (Main<WlSeat>,Main<WlKeyboard>),
    Touch => (Main<WlSeat>,Main<WlTouch>),
    Output => (Main<WlOutput>,u32),

    #[cfg(feature="shm")] Shm => (Main<WlShm>,u32),
    #[cfg(feature="shm")] ShmPool => (Main<WlShmPool>,u32),

    #[cfg(feature="xdg_shell")] XdgWmBase => (Main<XdgWmBase>,u32),
    #[cfg(feature="xdg_shell")] XdgSurface => (Main<XdgSurface>,u32),
    #[cfg(feature="xdg_shell")] XdgPopup => (Main<XdgPopup>,u32),
    #[cfg(feature="xdg_shell")] XdgPositioner => (Main<XdgPositioner>,u32),
    #[cfg(feature="xdg_shell")] XdgToplevel => (Main<XdgToplevel>,u32)
);

request_enum!(
    #[derive(Debug)]Destruction | |
    Compositor => Resource<WlCompositor>,
    Subcompositor => Resource<WlSubcompositor>,
    Shell => Resource<WlShell>,

    Seat => Resource<WlSeat>,
    Pointer => Resource<WlPointer>,
    Keyboard => Resource<WlKeyboard>,
    Touch => Resource<WlTouch>,
    Output => Resource<WlOutput>,

    #[cfg(feature="shm")] Shm => Resource<WlShm>,
    #[cfg(feature="shm")] ShmPool => Resource<WlShmPool>,

    #[cfg(feature="xdg_shell")] XdgWmBase => Resource<XdgWmBase>,
    #[cfg(feature="xdg_shell")] XdgSurface => Resource<XdgSurface>,
    #[cfg(feature="xdg_shell")] XdgPopup => Resource<XdgPopup>,
    #[cfg(feature="xdg_shell")] XdgPositioner => Resource<XdgPositioner>,
    #[cfg(feature="xdg_shell")] XdgToplevel => Resource<XdgToplevel>
);

request_enum!(
    #[derive(Debug)] WaylandRequest |
    Compositor => WlCompositor,
    Subcompositor => WlSubcompositor,
    Shell => WlShell,

    Seat => WlSeat,
    Pointer => WlPointer,
    Keyboard => WlKeyboard,
    Touch => WlTouch,
    Output => WlOutput,

    #[cfg(feature="shm")] Shm => WlShm,
    #[cfg(feature="shm")] ShmPool => WlShmPool,

    #[cfg(feature="xdg_shell")] XdgWmBase => XdgWmBase,
    #[cfg(feature="xdg_shell")] XdgSurface => XdgSurface,
    #[cfg(feature="xdg_shell")] XdgPopup => XdgPopup,
    #[cfg(feature="xdg_shell")] XdgPositioner => XdgPositioner,
    #[cfg(feature="xdg_shell")] XdgToplevel => XdgToplevel |
    Instantiation => Instantiation,
    Destruction => Destruction
);


#[derive(Default)]
pub struct ClientResources {
    pub compositor: Option<Main<WlCompositor>>,
    pub subcompositor: Option<Main<WlSubcompositor>>,
    pub shells: Vec<Main<WlShell>>,

    pub seats: Vec<Seat>,
    pub outputs: Vec<Main<WlOutput>>,

    #[cfg(feature="shm")]
    pub shm: Option<Main<WlShm>>,
    #[cfg(feature="shm")]
    pub shm_pools: Vec<Main<WlShmPool>>,

    #[cfg(feature="xdg_shell")]
    pub xdg_wm_base: Option<Main<XdgWmBase>>,
    #[cfg(feature="xdg_shell")]
    pub xdg_surfaces: Vec<Main<XdgSurface>>,
    #[cfg(feature="xdg_shell")]
    pub xdg_popups: Vec<Main<XdgPopup>>,
    #[cfg(feature="xdg_shell")]
    pub xdg_positioners: Vec<Main<XdgPositioner>>,
    #[cfg(feature="xdg_shell")]
    pub xdg_top_levels: Vec<Main<XdgToplevel>>
}

pub struct Seat {
    pub handle: Main<WlSeat>,
    pub pointers: Vec<Main<WlPointer>>,
    pub keyboards: Vec<Main<WlKeyboard>>,
    pub touchs: Vec<Main<WlTouch>>,
}
impl Seat {
    pub fn new(handle: Main<WlSeat>)->Self {
        let pointers = Vec::new();
        let keyboards = Vec::new();
        let touchs = Vec::new();
        Self {handle,pointers,keyboards,touchs}
    }
}
