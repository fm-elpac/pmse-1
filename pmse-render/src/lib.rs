//! pmse-render 渲染层
#![deny(unsafe_code)]

pub use allsorts;
pub use tiny_skia;

mod err;
mod font;
mod t;
mod vulkan;

pub use err::E;
pub use font::{
    draw_char, DrawOp, FontImg, FontImgSize, FontLoader, GlyphCache, GlyphItem, C_8192,
};
pub use vulkan::{draw_t, PmseRenderHost, PmseRenderInit, PmseRenderSc, 提交_gpu_执行等待};

#[cfg(test)]
mod tests {
    // TODO
}
