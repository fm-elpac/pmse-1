//! wayland connection: 从 gtk4 window 获取连接
#![allow(unsafe_code)]

use std::error::Error;

use gdk4::prelude::DisplayExtManual;
use gdk4_wayland::{
    prelude::WaylandSurfaceExtManual,
    wayland_client::{protocol::wl_surface::WlSurface, Connection},
    WaylandDisplay, WaylandSurface,
};
use gtk4::{
    glib::{object::Cast, translate::ToGlibPtr},
    prelude::{NativeExt, RootExt},
    ApplicationWindow,
};

use pmse_u::E;

use crate::VulkanSurface;

/// wayland connection
#[derive(Debug, Clone)]
pub struct WaylandConn {
    // raw
    w: ApplicationWindow,
    // wayland 连接
    c: Connection,
}

impl WaylandConn {
    /// 从 gtk4 window 获取连接
    pub fn new(w: &ApplicationWindow) -> Result<Self, Box<dyn Error>> {
        let wd = 获取wd(w)?;
        let c = 获取连接(&wd);
        // debug
        println!("  {:?}", c);

        Ok(Self { w: w.clone(), c })
    }

    /// 创建 VulkanSurface
    ///
    /// 注意: 必须在窗口显示之后调用
    pub fn surface(&self) -> Result<VulkanSurface, Box<dyn Error>> {
        let ws = 获取窗口表面(&self.w)?;
        Ok(VulkanSurface::new(self.c.clone(), ws))
    }
}

// (函数)

/// 获取 WaylandDisplay
fn 获取wd(w: &ApplicationWindow) -> Result<WaylandDisplay, Box<dyn Error>> {
    let gdk_d = w.display();
    let 后端 = gdk_d.backend();
    // debug
    println!("gtk4 backend = {:?}", 后端);

    let wd = gdk_d
        .downcast::<WaylandDisplay>()
        .ok()
        .ok_or(E("ERROR wayland cast display".into()))?;
    println!("  {:?}", wd);

    Ok(wd)
}

// 获取 WlDisplay, WlCompositor
// fn 获取dwc(wd: &WaylandDisplay) -> Result<(WlDisplay, WlCompositor), Box<dyn Error>> {
//     let d = wd
//         .wl_display()
//         .ok_or(E("ERROR wayland wl_display".into()))?;
//     let wc = wd
//         .wl_compositor()
//         .ok_or(E("ERROR wayland wl_compositor".into()))?;
//     println!("  {:?}", d);
//     println!("  {:?}", wc);
//     Ok((d, wc))
// }

/// 获取 wayland connection
///
/// 注意: 只能调用一次
///
/// https://gtk-rs.org/gtk4-rs/stable/latest/docs/src/gdk4_wayland/wayland_display.rs.html#91
fn 获取连接(wd: &WaylandDisplay) -> Connection {
    use gdk4_wayland::ffi;
    unsafe {
        let display_ptr = ffi::gdk_wayland_display_get_wl_display(wd.to_glib_none().0);
        let backend =
            wayland_backend::sys::client::Backend::from_foreign_display(display_ptr as *mut _);
        Connection::from_backend(backend)
    }
}

/// 获取窗口的顶层表面 WlSurface
fn 获取窗口表面(w: &ApplicationWindow) -> Result<WlSurface, Box<dyn Error>> {
    let gdk_s = w.surface().ok_or("ERROR wayland no surface")?;
    let ws = gdk_s
        .downcast::<WaylandSurface>()
        .ok()
        .ok_or(E("ERROR wayland cast surface".into()))?;
    println!("  {:?}", ws);

    let s = ws
        .wl_surface()
        .ok_or(E("ERROR wayland wl_surface".into()))?;
    Ok(s)
}
