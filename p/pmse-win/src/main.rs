//! pmse-win
#![deny(unsafe_code)]

use env_logger;

use pmse_render::{
    draw_t::{PmseRenderT, 三角形},
    PmseRenderInit,
};

mod w;

use w::{pmse_win_main, HandleBox, 回调接口};

struct 回调 {
    pri: PmseRenderInit,
    t: Option<PmseRenderT>,
}

impl 回调 {
    pub fn new() -> Self {
        let pri = PmseRenderInit::vulkan().unwrap();
        Self { pri, t: None }
    }
}

impl 回调接口 for 回调 {
    fn 初始化(&mut self, h: HandleBox) {
        println!("cb init");

        let pr = self.pri.clone().init_w(h.into()).unwrap();
        let t = PmseRenderT::new(pr, (1280, 720)).unwrap();
        self.t = Some(t);
    }

    fn 绘制(&mut self) {
        println!("cb draw");

        self.t
            .as_mut()
            .unwrap()
            .draw(vec![三角形::default()])
            .unwrap();
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    println!("main");

    pmse_win_main("测试窗口 (vulkan)".into(), (1280, 720), 回调::new());
}
