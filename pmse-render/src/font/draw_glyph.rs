//! 绘制单个字符 (使用 tiny-skia)
use std::error::Error;

use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use pmse_u::E;

/// 一条绘制命令
#[derive(Debug, Clone)]
pub enum SrDrawOp {
    /// 移动画笔 (x, y)
    MoveTo(f32, f32),
    /// 直线 (x, y)
    LineTo(f32, f32),
    /// 二次贝塞尔曲线 (3 个控制点) (x1, y1, x, y)
    QuadTo(f32, f32, f32, f32),
    /// 三次贝塞尔曲线 (4 个控制点) (x1, y1, x2, y2, x, y)
    CubicTo(f32, f32, f32, f32, f32, f32),
    /// 关闭路径
    Close,
}

impl SrDrawOp {
    /// 对点的 x, y 坐标进行 缩放, 平移
    pub fn map<T: Fn(f32, f32) -> (f32, f32)>(&self, f: T) -> Self {
        match self {
            Self::MoveTo(x, y) => {
                let (x, y) = f(*x, *y);
                Self::MoveTo(x, y)
            }
            Self::LineTo(x, y) => {
                let (x, y) = f(*x, *y);
                Self::LineTo(x, y)
            }
            Self::QuadTo(x1, y1, x, y) => {
                let (x1, y1) = f(*x1, *y1);
                let (x, y) = f(*x, *y);
                Self::QuadTo(x1, y1, x, y)
            }
            Self::CubicTo(x1, y1, x2, y2, x, y) => {
                let (x1, y1) = f(*x1, *y1);
                let (x2, y2) = f(*x2, *y2);
                let (x, y) = f(*x, *y);
                Self::CubicTo(x1, y1, x2, y2, x, y)
            }
            Self::Close => Self::Close,
        }
    }
}

/// 创建画布
pub fn 绘制字符_初始化(宽高: (f32, f32)) -> Result<Pixmap, Box<dyn Error>> {
    // 创建图片 (绘制) 缓冲区
    let 缓冲 = Pixmap::new(宽高.0 as u32, 宽高.1 as u32).ok_or(E("Pixmap::new()".into()))?;
    Ok(缓冲)
}

/// 绘制单个字符
pub fn 绘制字符<F: Fn(f32, f32) -> (f32, f32)>(
    缓冲: &mut Pixmap,
    命令: &Vec<SrDrawOp>,
    f: F,
) -> Result<(), Box<dyn Error>> {
    let mut 画笔 = Paint::default();
    画笔.set_color_rgba8(255, 255, 255, 255);
    画笔.anti_alias = true;

    let mut 路径 = PathBuilder::new();
    for c in 命令 {
        match c.map(&f) {
            SrDrawOp::MoveTo(x, y) => {
                路径.move_to(x, y);
            }
            SrDrawOp::LineTo(x, y) => {
                路径.line_to(x, y);
            }
            SrDrawOp::QuadTo(x1, y1, x, y) => {
                路径.quad_to(x1, y1, x, y);
            }
            SrDrawOp::CubicTo(x1, y1, x2, y2, x, y) => {
                路径.cubic_to(x1, y1, x2, y2, x, y);
            }
            // 关闭路径
            SrDrawOp::Close => {
                路径.close();
            }
        }
    }
    // 在所有路径结束后, 填充路径
    match 路径.finish() {
        Some(p) => {
            // 填充路径
            缓冲.fill_path(&p, &画笔, FillRule::Winding, Transform::identity(), None);
        }
        // TODO
        None => {}
    }
    Ok(())
}
