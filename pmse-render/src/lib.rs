//! pmse-render (API 前缀: `sr` 渲染层)
#![deny(unsafe_code)]

// re-export
pub use allsorts;
pub use tiny_skia;

mod font;
mod t;
mod vulkan;

pub use font::{
    SrDrawOp, SrFontImg, SrFontImgSize, SrFontLoader, SrGlyphCache, SrGlyphItem, SR_C_8192,
    SR_LANG_1, SR_SCRIPT_1,
};
pub use vulkan::{draw_t, sr_提交_gpu_执行等待, SrVk1, SrVkInit, SrVkSwapchain};

#[cfg(test)]
mod tests {
    // TODO
}
