//! (wayland) RawWindowHandle, RawDisplayHandle
#![allow(unsafe_code)]

use gdk4_wayland::wayland_client::{protocol::wl_surface::WlSurface, Proxy};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, WaylandWindowHandle,
};
use wayland_backend::sys::client::Backend;

/// 提供 RawWindowHandle, RawDisplayHandle (wayland)
#[derive(Debug, Clone)]
pub struct HandleBox {
    rd: RawDisplayHandle,
    rw: RawWindowHandle,
}

impl HandleBox {
    pub fn new(b: &Backend, s: &WlSurface) -> Self {
        let rd = b.raw_display_handle();

        // https://docs.rs/winit-gtk/0.29.1/src/winit/platform_impl/linux/window.rs.html
        let mut wh = WaylandWindowHandle::empty();
        wh.surface = s.id().as_ptr() as *mut _;
        let rw = RawWindowHandle::Wayland(wh);

        Self { rd, rw }
    }
}

unsafe impl HasRawDisplayHandle for HandleBox {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.rd
    }
}

unsafe impl HasRawWindowHandle for HandleBox {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.rw
    }
}

// TODO
unsafe impl Send for HandleBox {}
unsafe impl Sync for HandleBox {}
