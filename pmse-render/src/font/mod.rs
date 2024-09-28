//! 文本 (字体) 渲染 (排版/字符绘制)

mod draw_glyph;
mod draw_rect;
mod font_img;
mod layout;
mod load;

pub use draw_glyph::SrDrawOp;
pub use font_img::{SrFontImg, SrFontImgSize};
pub use load::{SrFontLoader, SrGlyphCache, SrGlyphItem, SR_C_8192, SR_LANG_1, SR_SCRIPT_1};
