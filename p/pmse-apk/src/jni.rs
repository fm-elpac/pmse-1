//! Android JNI
#![allow(unsafe_code)]
use std::sync::Mutex;

use android_logger::Config;
use jni::{objects::JClass, JNIEnv};
use log::{debug, LevelFilter};
use ndk_sys::{ANativeWindow, ANativeWindow_fromSurface};
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, HasRawDisplayHandle, HasRawWindowHandle,
    RawDisplayHandle, RawWindowHandle,
};

use pmse_render::{
    draw_t::{PmseRenderT, 三角形},
    PmseRenderHost, PmseRenderInit,
};

struct 测试渲染 {
    pri: Option<PmseRenderInit>,
    pr: Option<PmseRenderHost>,
    t: Option<PmseRenderT>,
}

impl 测试渲染 {
    pub const fn new() -> Self {
        Self {
            pri: None,
            pr: None,
            t: None,
        }
    }

    pub fn init(&mut self) {
        let pri = PmseRenderInit::vulkan().unwrap();
        self.pri = Some(pri);
    }

    // after init()
    pub fn create(&mut self, h: HandleBox) {
        let pr = self.pri.take().unwrap().init_w(h.into()).unwrap();
        self.pr = Some(pr);
    }

    pub fn destroy(&mut self) {
        // TODO
    }

    // after create()
    pub fn resize(&mut self, w: u32, h: u32) {
        let pr = self.pr.take().unwrap();
        let t = PmseRenderT::new(pr, (w, h)).unwrap();
        self.t = Some(t);
    }

    pub fn draw(&mut self) {
        self.t
            .as_mut()
            .unwrap()
            .draw(vec![三角形::default()])
            .unwrap();
    }
}

// TODO 全局变量, 方便测试
static 测试1: Mutex<测试渲染> = Mutex::new(测试渲染::new());

/// io.github.fm_elpac.pmse_apk.vulkan_bridge.VulkanJNI.nativeInit()
#[no_mangle]
pub extern "system" fn Java_io_github_fm_1elpac_pmse_1apk_vulkan_1bridge_VulkanJNI_nativeInit<
    'local,
>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    // init android logger
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("pmse_apk"),
    );

    debug!("from rust: nativeInit()");

    测试1.lock().unwrap().init();
}

/// io.github.fm_elpac.pmse_apk.vulkan_bridge.VulkanJNI.nativeCreate(Surface)
#[no_mangle]
pub extern "system" fn Java_io_github_fm_1elpac_pmse_1apk_vulkan_1bridge_VulkanJNI_nativeCreate<
    'local,
>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    surface: JClass<'local>,
) {
    debug!("from rust: nativeCreate()");

    let nw = unsafe { ANativeWindow_fromSurface(env.get_raw(), **surface) };
    let h = HandleBox::new(nw);

    测试1.lock().unwrap().create(h);
}

/// io.github.fm_elpac.pmse_apk.vulkan_bridge.VulkanJNI.nativeDestroy()
#[no_mangle]
pub extern "system" fn Java_io_github_fm_1elpac_pmse_1apk_vulkan_1bridge_VulkanJNI_nativeDestroy<
    'local,
>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    debug!("from rust: nativeDestroy()");

    测试1.lock().unwrap().destroy();
}

/// io.github.fm_elpac.pmse_apk.vulkan_bridge.VulkanJNI.nativeResize(Int, Int)
#[no_mangle]
pub extern "system" fn Java_io_github_fm_1elpac_pmse_1apk_vulkan_1bridge_VulkanJNI_nativeResize<
    'local,
>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    w: i32,
    h: i32,
) {
    debug!("from rust: nativeResize({}, {})", w, h);

    测试1.lock().unwrap().resize(w as u32, h as u32);
}

/// io.github.fm_elpac.pmse_apk.vulkan_bridge.VulkanJNI.nativeDraw()
#[no_mangle]
pub extern "system" fn Java_io_github_fm_1elpac_pmse_1apk_vulkan_1bridge_VulkanJNI_nativeDraw<
    'local,
>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    debug!("from rust: nativeDraw()");

    测试1.lock().unwrap().draw();
}

/// 提供 RawWindowHandle, RawDisplayHandle (Android)
#[derive(Debug, Clone)]
pub struct HandleBox {
    rd: RawDisplayHandle,
    rw: RawWindowHandle,
}

impl HandleBox {
    pub fn new(w: *mut ANativeWindow) -> Self {
        let mut h = AndroidNdkWindowHandle::empty();
        h.a_native_window = w as *mut _;

        let rw = RawWindowHandle::AndroidNdk(h);
        let rd = RawDisplayHandle::Android(AndroidDisplayHandle::empty());
        Self { rd, rw }
    }
}

unsafe impl HasRawDisplayHandle for HandleBox {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.rd
    }
}

unsafe impl HasRawWindowHandle for HandleBox {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.rw
    }
}

// TODO
unsafe impl Send for HandleBox {}
unsafe impl Sync for HandleBox {}
