//! wayland subsurface
//!
//! https://github.com/Smithay/wayland-rs/pull/572
use std::error::Error;
use std::future::poll_fn;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;

use gdk4_wayland::wayland_client::{
    protocol::{wl_compositor, wl_registry, wl_subcompositor, wl_subsurface, wl_surface},
    Connection, Dispatch, EventQueue, QueueHandle,
};
use gtk4::glib::{self, ControlFlow};

use pmse_u::E;

use crate::{Cb, HandleBox};

/// wayland subsurface (vulkan)
#[derive(Debug, Clone)]
pub struct VulkanSurface {
    c: Connection,
    // toplevel window surface
    ws: wl_surface::WlSurface,
}

impl VulkanSurface {
    pub(crate) fn new(c: Connection, ws: wl_surface::WlSurface) -> Self {
        Self { c, ws }
    }

    /// 运行新的 wayland queue
    pub fn run(self, offset: (i32, i32), cb: Arc<Box<dyn Cb>>) {
        运行(&self.c, self.ws.clone(), offset, cb).unwrap();
    }
}

// 创建 subsurface
struct AppData {
    ws: wl_surface::WlSurface,
    偏移: (i32, i32),
    回调: Arc<Box<dyn Cb>>,
    wc: Option<wl_compositor::WlCompositor>,
    sc: Option<wl_subcompositor::WlSubcompositor>,
    s: Option<wl_surface::WlSurface>,
    ss: Option<wl_subsurface::WlSubsurface>,
}

impl AppData {
    pub fn 检查绑定(&mut self, c: &Connection, h: &QueueHandle<Self>) {
        // 注意: 只能调用一次, 不能重复创建
        if self.wc.is_some() && self.sc.is_some() && self.ss.is_none() {
            self.创建表面(c, h);
        }
    }

    /// 创建 subsurface
    fn 创建表面(&mut self, c: &Connection, h: &QueueHandle<Self>) {
        // debug
        println!("create subsurface {:?}", self.偏移);
        // 创建新的表面
        let s = self.wc.as_ref().unwrap().create_surface(h, ());
        // 创建下级表面 (设置上级表面)
        let ss = self
            .sc
            .as_ref()
            .unwrap()
            .get_subsurface(&s, &self.ws, h, ());

        // 设置下级表面 偏移
        ss.set_position(self.偏移.0, self.偏移.1);
        // 下级表面显示在上级表面前面 (上方)
        ss.place_above(&self.ws);

        // 分离下级表面 (不再等待上级表面提交)
        ss.set_desync();
        // 同步设置 (提交)
        s.commit();
        self.ws.commit(); // 上级表面也提交, 使设置生效

        // 回调
        let hb = HandleBox::new(&c.backend(), &s);
        self.回调.cb(hb);

        // 初始化完成, 保存结果
        self.s.replace(s);
        self.ss.replace(ss);
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        r: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        c: &Connection,
        h: &QueueHandle<AppData>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            //println!("    [{}] {} (v{})", name, interface, version);
            // 绑定感兴趣的接口
            match interface.as_str() {
                "wl_compositor" => {
                    let wc = r.bind::<wl_compositor::WlCompositor, _, _>(name, version, h, ());
                    // debug
                    println!("  {:?}", wc);
                    state.wc.replace(wc);
                }
                "wl_subcompositor" => {
                    let sc =
                        r.bind::<wl_subcompositor::WlSubcompositor, _, _>(name, version, h, ());
                    // debug
                    println!("  {:?}", sc);
                    state.sc.replace(sc);
                }
                _ => {}
            }

            // 检查绑定完成
            state.检查绑定(c, h);
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
    }
}

impl Dispatch<wl_subcompositor::WlSubcompositor, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_subcompositor::WlSubcompositor,
        _: wl_subcompositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        _: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        // TODO
    }
}

impl Dispatch<wl_subsurface::WlSubsurface, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_subsurface::WlSubsurface,
        _: wl_subsurface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        // TODO
    }
}

/// gtk4 运行 wayland queue
fn 运行队列1(c: &Connection) -> Result<EventQueue<AppData>, Box<dyn Error>> {
    let q = c.new_event_queue();
    let h = q.handle();

    let _r = c.display().get_registry(&h, ());
    // debug
    println!("wayland gtk4 read");
    let 连接 = c.clone();
    let fd = 连接
        .prepare_read()
        .ok_or(E("ERROR wayland prepare_read".into()))?
        .connection_fd()
        .as_raw_fd();
    glib::source::unix_fd_add_local(fd, glib::IOCondition::IN, move |_, _| {
        match 连接.prepare_read() {
            Some(g) => {
                g.read().unwrap();
            }
            None => {
                连接.backend().dispatch_inner_queue().unwrap();
            }
        }
        // TODO
        ControlFlow::Continue
    });

    Ok(q)
}

fn 运行队列2(mut q: EventQueue<AppData>, mut a: AppData) {
    glib::MainContext::default().spawn_local(async move {
        poll_fn(|cx| q.poll_dispatch_pending(cx, &mut a))
            .await
            .unwrap();
    });
}

/// 运行 wayland queue (subcompositor)
fn 运行(
    c: &Connection,
    ws: wl_surface::WlSurface,
    偏移: (i32, i32),
    回调: Arc<Box<dyn Cb>>,
) -> Result<(), Box<dyn Error>> {
    println!("wayland queue run");
    let q = 运行队列1(c)?;

    let a = AppData {
        ws,
        偏移,
        回调,
        wc: None,
        sc: None,
        s: None,
        ss: None,
    };
    println!("wayland registry global:");

    运行队列2(q, a);
    Ok(())
}
