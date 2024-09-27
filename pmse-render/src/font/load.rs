//! 字体加载与初始化
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use allsorts::{
    binary::read::ReadScope,
    cff::CFF,
    font::{Font, GlyphTableFlags, MatchingPresentation},
    font_data::FontData,
    glyph_position::{GlyphLayout, GlyphPosition, TextDirection},
    gpos::Info,
    gsub::{FeatureMask, Features},
    outline::{OutlineBuilder, OutlineSink},
    pathfinder_geometry::{line_segment::LineSegment2F, vector::Vector2F},
    tables::{glyf::GlyfTable, loca::LocaTable, FontTableProvider, SfntVersion},
    tag,
    woff2::{Woff2Font, Woff2TableProvider},
};
use log::debug;

use super::draw_glyph::DrawOp;
use crate::E;

pub static C_8192: &'static str = include_str!("c_8105_8192.txt");

/// 字体文件加载器 (woff2)
pub struct FontLoader {
    // no Debug, no Clone
    字体: Font<Woff2TableProvider>,
    /// 字符单位 (设计单位, 单个字符 宽/高)
    em_size: u16,
    /// 字符包围框 (x_min, x_max, y_min, y_max)
    bbox: (i16, i16, i16, i16),
    /// 用于绘制字形
    c: GlyphCache,
}

impl FontLoader {
    /// 加载字体文件
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        debug!("load font {}", filename);

        // TODO 支持指定 table index
        let mut 字体 = 读取字体(filename, 0)?;
        let (em_size, bbox) = 读取头(&字体)?;

        // 读取字形数据
        let c = GlyphCache::new(&mut 字体)?;
        debug!("load font ok");
        Ok(Self {
            字体,
            em_size,
            bbox,
            c,
        })
    }

    /// 获取字符单位 (设计单位, 单个字符 宽/高)
    pub fn em_size(&self) -> u16 {
        self.em_size
    }

    /// 获取字符包围框 (x_min, x_max, y_min, y_max)
    pub fn bbox(&self) -> (i16, i16, i16, i16) {
        self.bbox
    }

    /// 获取单个字符数据
    pub fn get_c(&self, glyph_index: u16) -> Option<&GlyphItem> {
        self.c.get(glyph_index)
    }

    /// 文本排版
    pub fn shape(&mut self, text: &str) -> Result<Vec<(Info, GlyphPosition)>, Box<dyn Error>> {
        // TODO 支持指定 script, lang
        let script = tag::from_string("Han")?;
        let lang = tag::from_string("zh")?;

        // 根据输入字符串, 查找对应的字体字符
        let 字符 = self
            .字体
            .map_glyphs(text, script, MatchingPresentation::NotRequired);
        let 形状 = self
            .字体
            .shape(
                字符,
                script,
                Some(lang),
                &Features::Mask(FeatureMask::default()),
                // TODO fvar variation axis
                None,
                true,
            )
            .or_else(|(_, i)| -> Result<Vec<Info>, Box<dyn Error>> { Ok(i) })?;

        let mut 布局 = GlyphLayout::new(&mut self.字体, &形状, TextDirection::LeftToRight, false);
        let 位置 = 布局.glyph_positions()?;

        let o: Vec<(Info, GlyphPosition)> = 形状
            .iter()
            .zip(&位置)
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect();
        Ok(o)
    }
}

/// 读取字体文件 (woff2)
fn 读取字体(
    文件名: &str, 表序号: usize
) -> Result<Font<Woff2TableProvider>, Box<dyn Error>> {
    let 缓冲区 = fs::read(文件名)?;
    let 范围 = ReadScope::new(&缓冲区);
    let 字体数据 = 范围.read::<FontData<'_>>()?;
    let 字体文件: Woff2Font<'_> = match 字体数据 {
        FontData::Woff2(w) => w,
        _ => {
            return Err(Box::new(E("load font".into())));
        }
    };
    let 提供 = 字体文件.table_provider(表序号)?;
    let 字体 = Font::new(提供)?;

    Ok(字体)
}

/// 从字体头部获取信息
fn 读取头(
    字体: &Font<Woff2TableProvider>,
) -> Result<(u16, (i16, i16, i16, i16)), Box<dyn Error>> {
    let 头表 = 字体.head_table()?.ok_or(E("font no head_table".into()))?;
    let em_size = 头表.units_per_em;
    let bbox = (头表.x_min, 头表.x_max, 头表.y_min, 头表.y_max);

    Ok((em_size, bbox))
}

/// 单个字形的数据
#[derive(Debug, Clone)]
pub struct GlyphItem {
    /// 字符绘制命令
    命令: Vec<DrawOp>,
    /// 字符包围盒 (x_min, x_max, y_min, y_max)
    bb: Option<(i16, i16, i16, i16)>, // TODO 更多 字符数据
}

impl GlyphItem {
    pub fn new(命令: Vec<DrawOp>, bb: Option<(i16, i16, i16, i16)>) -> Self {
        Self { 命令, bb }
    }

    /// 获取字符包围盒 (x_min, x_max, y_min, y_max)
    pub fn bb(&self) -> Option<(i16, i16, i16, i16)> {
        self.bb
    }

    /// 获取字符绘制命令
    pub fn 命令(&self) -> &Vec<DrawOp> {
        &self.命令
    }
}

/// 字形缓存, 用于快速查询字形数据
#[derive(Debug, Clone)]
pub struct GlyphCache {
    // u16: glyph_index
    c: HashMap<u16, GlyphItem>,
}

impl GlyphCache {
    /// 初始化 (加载数据)
    pub fn new(字体: &mut Font<Woff2TableProvider>) -> Result<Self, Box<dyn Error>> {
        let mut c = HashMap::new();
        debug!("GlyphCache load");
        let sfnt_version = 字体.font_table_provider.sfnt_version();
        debug!("  sfnt_version = {}", sfnt_version);

        // 读取字形数据
        //&& 字体.font_table_provider.sfnt_version() == tag::OTTO
        if 字体.glyph_table_flags.contains(GlyphTableFlags::CFF) {
            debug!("CFF");
            // 读取 CFF 数据
            let cff_data: Vec<u8> = 字体
                .font_table_provider
                .read_table_data(tag::CFF)?
                .into_owned();
            let mut cff = ReadScope::new(&cff_data).read::<CFF<'_>>()?;

            debug!("  name_index.len() = {}", cff.name_index.len());
            debug!("  string_index.len() = {}", cff.string_index.len());
            debug!(
                "  global_subr_index.len() = {}",
                cff.global_subr_index.len()
            );
            debug!("  fonts.len() = {}", cff.fonts.len());
            // 检查每个 font
            for f in &cff.fonts {
                debug!("font");
                debug!("  top_dict.len() = {}", f.top_dict.len());
                debug!(
                    "  char_strings_index.len() = {}",
                    f.char_strings_index.len()
                );
                //debug!("  charset = {:?}", f.charset);
                //debug!("  .len() = {}", f..len());
            }

            // 处理 C_8192
            for j in C_8192.chars() {
                let s = String::from(j);
                // TODO
                let script = tag::from_string("Han")?;
                let lang = tag::from_string("zh")?;

                let 字符 = 字体.map_glyphs(&s, script, MatchingPresentation::NotRequired);
                for r in 字符 {
                    let i = r.glyph_index;
                    // TODO 字符包围盒
                    let bb = None;

                    // 绘制字符
                    let mut 记录 = 记录器::new();
                    记录.绘制(&mut cff, vec![i])?;

                    let 命令 = 记录.命令();
                    c.insert(i, GlyphItem::new(命令, bb));
                }
            }
        } else if 字体.glyph_table_flags.contains(GlyphTableFlags::GLYF) {
            debug!("glyf");
            // 读取 loca 数据
            let loca_data = 字体.font_table_provider.read_table_data(tag::LOCA)?;
            // 读取 glyf 数据
            let glyf_data = 字体.font_table_provider.read_table_data(tag::GLYF)?;

            let loca = ReadScope::new(&loca_data).read_dep::<LocaTable<'_>>((
                usize::from(字体.maxp_table.num_glyphs),
                字体
                    .head_table()?
                    .ok_or("no head_table")?
                    .index_to_loc_format,
            ))?;
            let mut glyf = ReadScope::new(&glyf_data).read_dep::<GlyfTable<'_>>(&loca)?;

            // 处理每一个字符
            let n = glyf.num_glyphs();
            debug!("  glyf.num_glyphs() = {}", n);
            for i in 0..n {
                let g = glyf.get_parsed_glyph(i)?;
                // 字符包围盒
                let bb = g
                    .bounding_box()
                    .map(|b| (b.x_min, b.x_max, b.y_min, b.y_max));

                // 绘制字符
                let mut 记录 = 记录器::new();
                记录.绘制(&mut glyf, vec![i])?;

                let 命令 = 记录.命令();
                c.insert(i, GlyphItem::new(命令, bb));
            }
        } else {
            return Err(Box::new(E("no CFF or GLYF table".into())));
        }

        Ok(Self { c })
    }

    pub fn get(&self, glyph_index: u16) -> Option<&GlyphItem> {
        self.c.get(&glyph_index)
    }
}

/// 记录 allsorts 字符绘制命令
#[derive(Debug, Clone)]
struct 记录器 {
    命令: Vec<DrawOp>,
}

impl 记录器 {
    pub fn new() -> Self {
        Self { 命令: Vec::new() }
    }

    pub fn 命令(self) -> Vec<DrawOp> {
        self.命令
    }

    pub fn 绘制<T: OutlineBuilder<Error = E>, E: Error + 'static>(
        &mut self,
        构造: &mut T,
        序号: Vec<u16>,
    ) -> Result<(), Box<dyn Error>> {
        for i in 序号 {
            构造.visit(i, self)?;
        }
        Ok(())
    }
}

impl OutlineSink for 记录器 {
    fn move_to(&mut self, to: Vector2F) {
        self.命令.push(DrawOp::MoveTo(to.x(), to.y()));
    }

    fn line_to(&mut self, to: Vector2F) {
        self.命令.push(DrawOp::LineTo(to.x(), to.y()));
    }

    fn quadratic_curve_to(&mut self, control: Vector2F, to: Vector2F) {
        self.命令
            .push(DrawOp::QuadTo(control.x(), control.y(), to.x(), to.y()));
    }

    fn cubic_curve_to(&mut self, control: LineSegment2F, to: Vector2F) {
        self.命令.push(DrawOp::CubicTo(
            control.from_x(),
            control.from_y(),
            control.to_x(),
            control.to_y(),
            to.x(),
            to.y(),
        ));
    }

    fn close(&mut self) {
        self.命令.push(DrawOp::Close);
    }
}
