//! 字体图片
use std::error::Error;

use image::GrayImage;
use tiny_skia::Pixmap;

use pmse_u::E;

use super::{
    draw_glyph::{绘制字符, 绘制字符_初始化},
    load::SrFontLoader,
};

/// 字体图片的分辨率: 2K (2048), 4K (4096)
#[derive(Debug, Clone, Copy)]
pub enum SrFontImgSize {
    /// 2048 x 2048
    P2k,
    /// 4096 x 4096
    P4k,
}

impl Default for SrFontImgSize {
    fn default() -> Self {
        Self::P2k
    }
}

impl From<SrFontImgSize> for u32 {
    fn from(value: SrFontImgSize) -> u32 {
        match value {
            SrFontImgSize::P2k => 2048,
            SrFontImgSize::P4k => 4096,
        }
    }
}

impl From<SrFontImgSize> for f32 {
    fn from(value: SrFontImgSize) -> f32 {
        let v: u32 = value.into();
        v as f32
    }
}

/// 字体图片: 绘制有许多字符的一张图片
#[derive(Debug, Clone)]
pub struct SrFontImg {
    s: SrFontImgSize,
    img: GrayImage,
    // TODO
    p: Pixmap,
}

impl SrFontImg {
    /// 创建实例
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // TODO 支持 4K
        let s = SrFontImgSize::default();

        let img = GrayImage::new(s.into(), s.into());
        let p = 绘制字符_初始化((s.into(), s.into()))?;
        Ok(Self { s, img, p })
    }

    /// 绘制文字, 32x32 阵列
    pub fn draw32(&mut self, font: &mut SrFontLoader, text: &[char]) -> Result<(), Box<dyn Error>> {
        // 绘制 32 行, 每行 32 个字符
        const L: u8 = 32;
        // 单个字符的高度 (字体文件里面的值)
        //let em = font.em_size() as f32;
        // 行高
        let em = font.line_height();
        // 要绘制的字符高度 (像素)
        let s: f32 = self.s.into();
        let em1: f32 = s / (L as f32);
        // 字符缩放比例
        let z = em1 / em;
        // y 坐标偏移
        let dy = (font.hhea().descender as f32) * z;

        // 当前绘制第 j 行, 第 i 列
        let mut i = 0;
        let mut j = 0;
        for c in text {
            let s = String::from(*c);
            // 当前字符 左上角的坐标
            let x0 = (i as f32) * em1;
            let y0 = (j as f32) * em1 + dy;

            // 绘制单个字符
            let 排版 = font.shape(&s)?;
            let 字形 = font
                .get_c(排版[0].0.glyph.glyph_index)
                .ok_or(E("no glyph".into()))?;
            绘制字符(&mut self.p, 字形.命令(), |x, y| {
                ((x * z) + x0, (em - y) * z + y0)
            })?;

            // 绘制完成一个字符, 更新 i, j 序号
            i += 1;
            if i >= L {
                i = 0;
                j += 1;
            }
        }
        Ok(())
    }

    /// 保存图片文件
    pub fn save(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        // 进行图片格式转换
        let s: u32 = self.s.into();
        for i in 0..s {
            for j in 0..s {
                // 复制单个像素
                match self.p.pixel(i, j) {
                    Some(p) => {
                        self.img.put_pixel(i, j, [p.alpha()].into());
                    }
                    None => {}
                }
            }
        }

        self.img.save(filename)?;
        Ok(())
    }
}
