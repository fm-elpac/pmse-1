//! pmse-render 渲染层
#![deny(unsafe_code)]

mod err;
mod t;
mod vulkan;

pub use err::E;
pub use vulkan::{draw_t, PmseRenderHost, PmseRenderInit, PmseRenderSc, 提交_gpu_执行等待};

#[cfg(test)]
mod tests {
    // TODO
}
