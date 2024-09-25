//! pmse-bin
#![deny(unsafe_code)]

use std::sync::Arc;

use env_logger;

use pmse_gtk::{pmse_gtk_main, Cb, ExitCode, HandleBox};
use pmse_render::{
    draw_t::{PmseRenderT, 三角形},
    PmseRenderInit,
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

    let pri = PmseRenderInit::vulkan().unwrap();
    let 回调: Arc<Box<dyn Cb>> = Arc::new(Box::new(回调 { pri }));

    pmse_gtk_main(
        "io.github.fm_elpac.pmse_bin".into(),
        "测试 (vulkan)".into(),
        (1280, 720, 62, 56),
        (44, 8, 8, 8),
        回调,
    )
}
