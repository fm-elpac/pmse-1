//! pmse-bin
#![deny(unsafe_code)]

use std::env;
use std::sync::Arc;

use env_logger;
use log::debug;

use pmse_gtk::{pmse_gtk_main, Cb, ExitCode, HandleBox};
use pmse_render::{
    draw_t::{SrT, 三角形},
    SrFontImg, SrFontLoader, SrVkInit, SR_C_8192,
};

#[derive(Debug, Clone)]
struct 回调 {
    ri: SrVkInit,
}

impl Cb for 回调 {
    fn cb(&self, h: HandleBox) {
        let r = self.ri.clone().init_w(h.into()).unwrap();
        let t = SrT::new(r, (1280, 720)).unwrap();
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
    let mut 字体 = SrFontLoader::new(&字体文件).unwrap();

    let 测试 = &a[1];
    debug!("  {}", 测试);
    let mut 图片 = SrFontImg::new().unwrap();
    let 字符: Vec<char> = SR_C_8192.chars().collect();
    图片.draw32(&mut 字体, &字符[0..1024]).unwrap();
    图片.save(测试).unwrap();

    // TODO
    // debug!("  before shape  {}  {:?}", 字体.em_size(), 字体.bbox());
    // let 排版 = 字体.shape(文本).unwrap();
    // println!("{:#?}", 排版);
    // debug!("  after shape");

    // init vulkan
    // let ri = SrVkInit::vulkan().unwrap();
    // let 回调: Arc<Box<dyn Cb>> = Arc::new(Box::new(回调 { ri }));

    // pmse_gtk_main(
    //     "io.github.fm_elpac.pmse_bin".into(),
    //     "测试 (vulkan)".into(),
    //     (1280, 720, 62, 56),
    //     (44, 8, 8, 8),
    //     回调,
    // )
    ExitCode::SUCCESS
}
