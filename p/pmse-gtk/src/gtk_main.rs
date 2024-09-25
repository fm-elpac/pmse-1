use std::sync::Arc;

use adw::Application;
use gtk4::{prelude::*, ApplicationWindow};

use crate::{ExitCode, HandleBox, WaylandConn};

/// 窗口回调
pub trait Cb {
    fn cb(&self, h: HandleBox);
}

/// 创建窗口
///
/// rect 矩形: (x宽, y高, x偏移, y偏移)
/// margin 边距: (上, 右, 下, 左)
pub fn pmse_gtk_main(
    app_id: String,
    title: String,
    rect: (i32, i32, i32, i32),
    margin: (i32, i32, i32, i32),
    cb: Arc<Box<dyn Cb>>,
) -> ExitCode {
    let app = Application::builder().application_id(&app_id).build();
    // 计算窗口宽高
    let w = rect.0 + margin.1 + margin.3;
    let h = rect.1 + margin.0 + margin.2;
    let 偏移 = (margin.3 + rect.2, margin.0 + rect.3);
    // debug
    println!(
        "pmse_gtk_main: {:?} {:?} w = {}, h = {} {:?}",
        rect, margin, w, h, 偏移
    );

    app.connect_activate(move |app| {
        let w = ApplicationWindow::builder()
            .application(app)
            .default_width(w)
            .default_height(h)
            .title(&title)
            // TODO
            .resizable(false)
            .build();
        // 窗口显示前的初始化
        let c = WaylandConn::new(&w).unwrap();
        // 显示窗口
        w.present();
        // 注意: 必须在显示窗口后调用, 否则没有 wayland surface
        let vs = c.surface().unwrap();

        vs.run(偏移, cb.clone());
    });

    app.run()
}
