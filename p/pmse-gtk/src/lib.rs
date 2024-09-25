//! pmse-gtk
#![deny(unsafe_code)]

mod err;
mod gtk_main;
mod raw_handle;
mod wayland_conn;
mod wayland_subsurface;

pub use err::E;
pub use gtk4::glib::ExitCode;
pub use gtk_main::{pmse_gtk_main, Cb};
pub use raw_handle::HandleBox;
pub use wayland_conn::WaylandConn;
pub use wayland_subsurface::VulkanSurface;

#[cfg(test)]
mod tests {
    // TODO
}
