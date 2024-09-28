//! vulkan render

mod shader;
mod swapchain;
mod test;
mod util;
mod vulkan_host;
mod vulkan_init;

pub use swapchain::{CreateCommand, SrVkSwapchain};
pub use test::draw_t;
pub use vulkan_host::SrVk1;
pub use vulkan_init::SrVkInit;

// TODO
pub use util::sr_提交_gpu_执行等待;
