//! pmse-bin
#![deny(unsafe_code)]

use std::env;
use std::sync::Arc;

use env_logger;
use log::debug;

use pmse_gtk::{pmse_gtk_main, Cb, ExitCode, HandleBox};
use pmse_render::{
    draw_char,
    draw_t::{PmseRenderT, 三角形},
    FontImg, FontLoader, PmseRenderInit, C_8192,
};

#[derive(Debug, Clone)]
struct 回调 {
    pri: PmseRenderInit,
}

impl Cb for 回调 {
    fn cb(&self, h: HandleBox) {
        let pr = self.pri.clone().init_w(h.into()).unwrap();
        let t = PmseRenderT::new(pr, (1280, 720)).unwrap();
        t.draw(vec![三角形::default()]).unwrap();
        // TODO
    }
}

fn main() -> ExitCode {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let a: Vec<String> = env::args().collect();

    // load font
    let 字体文件 = env::var("PMSE_FONT_FILE").unwrap();
    debug!("{}", 字体文件);
    let mut 字体 = FontLoader::new(&字体文件).unwrap();

    let 测试 = &a[1];
    debug!("  {}", 测试);
    let mut 图片 = FontImg::new().unwrap();
    let 字符: Vec<char> = C_8192.chars().collect();
    图片.draw32(&mut 字体, &字符[0..1024]).unwrap();
    图片.save(测试).unwrap();

    // TODO
    // debug!("  before shape  {}  {:?}", 字体.em_size(), 字体.bbox());
    // let 排版 = 字体.shape(文本).unwrap();
    // println!("{:#?}", 排版);
    // debug!("  after shape");

    // let i = 排版[0].0.glyph.glyph_index;
    // println!("  {}", i);
    // let 字形 = 字体.get_c(i).unwrap();
    // let s = 64.0;
    // let em = 字体.em_size() as f32;
    // // 缩放系数
    // let z = s / em;
    // let 图片 = draw_char(字形.命令(), (s, s), |x, y| (x * z, (em - y) * z)).unwrap();
    // 图片.save_png(&a[2]).unwrap();

    // test FontImg
    //let font_img = env::var("PMSE_FONT_IMG").unwrap();
    //debug!("{}", font_img);
    //let i = FontImg::new();
    //i.save(&font_img).unwrap();

    // init vulkan
    // let pri = PmseRenderInit::vulkan().unwrap();
    // let 回调: Arc<Box<dyn Cb>> = Arc::new(Box::new(回调 { pri }));

    // pmse_gtk_main(
    //     "io.github.fm_elpac.pmse_bin".into(),
    //     "测试 (vulkan)".into(),
    //     (1280, 720, 62, 56),
    //     (44, 8, 8, 8),
    //     回调,
    // )
    ExitCode::SUCCESS
}
