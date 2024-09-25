//! Windows 窗口封装
#![allow(unsafe_code)]

use std::ffi::c_void;

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle,
    WindowsDisplayHandle,
};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{RedrawWindow, ValidateRect, RDW_INVALIDATE},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
            LoadCursorW, PostQuitMessage, RegisterClassExW, SetWindowLongPtrW, ShowWindow,
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, SW_SHOWNORMAL, WINDOW_EX_STYLE,
            WINDOW_LONG_PTR_INDEX, WM_DESTROY, WM_PAINT, WM_SIZE, WNDCLASSEXW, WS_CAPTION,
            WS_OVERLAPPED, WS_SYSMENU, WS_VISIBLE,
        },
    },
};

struct 窗口数据 {
    pub 绘制回调: Option<Box<dyn FnMut() -> () + 'static>>,
}

struct 窗口封装 {
    实例: HINSTANCE,
    窗口: HWND,

    数据: Box<窗口数据>,
}

impl 窗口封装 {
    /// 创建窗口
    pub unsafe fn new(宽高: (i32, i32), 标题: String) -> Self {
        let 实例: HINSTANCE = GetModuleHandleW(None).unwrap().into();

        let 窗口类名1 = HSTRING::from("pmse_window");
        let 窗口类名 = PCWSTR(窗口类名1.as_ptr());
        let 窗口类 = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            // SetWindowLongPtrW()
            cbWndExtra: std::mem::size_of::<*const c_void>() as i32,

            hInstance: 实例,
            lpszClassName: 窗口类名,
            lpfnWndProc: Some(pmse_win_wndproc),

            style: CS_HREDRAW | CS_VREDRAW,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),

            ..Default::default()
        };
        // 注册窗口类
        let a = RegisterClassExW(&窗口类);
        if 0 == a {
            panic!("RegisterClassExW()");
        }

        // 防止字符串内存被回收
        let 标题1 = HSTRING::from(标题);
        let 标题 = PCWSTR(标题1.as_ptr());
        // 创建窗口
        let 窗口 = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            窗口类名,
            标题,
            // 禁止改变窗口大小
            // WS_OVERLAPPEDWINDOW
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            宽高.0,
            宽高.1,
            None,
            None,
            实例,
            None,
        )
        .unwrap();

        // 窗口数据
        let 数据 = Box::new(窗口数据 { 绘制回调: None });
        // 设置数据指针
        let 窗口数据指针: *const _ = &*数据;
        SetWindowLongPtrW(窗口, WINDOW_LONG_PTR_INDEX(0), 窗口数据指针 as isize);

        Self {
            实例, 窗口, 数据
        }
    }

    pub fn 设绘制回调(&mut self, 回调: Option<Box<dyn FnMut() -> () + 'static>>) {
        self.数据.绘制回调 = 回调;
    }

    pub fn 获取指针(&self) -> HandleBox {
        HandleBox::new(self.实例.0 as *mut _, self.窗口.0 as *mut _)
    }

    /// 请求重绘窗口
    pub unsafe fn 请求绘制(&mut self) {
        let _ = RedrawWindow(self.窗口, None, None, RDW_INVALIDATE);
    }

    pub unsafe fn 主循环(&mut self) {
        // 显示窗口
        let _ = ShowWindow(self.窗口, SW_SHOWNORMAL);

        let mut 消息 = MSG::default();
        while GetMessageW(&mut 消息, HWND(std::ptr::null_mut()), 0, 0).into() {
            DispatchMessageW(&消息);
        }
    }
}

const fn loword(x: u32) -> u16 {
    (x & 0xffff) as u16
}

const fn hiword(x: u32) -> u16 {
    ((x >> 16) & 0xffff) as u16
}

unsafe extern "system" fn pmse_win_wndproc(
    窗口: HWND,
    消息: u32,
    w参数: WPARAM,
    l参数: LPARAM,
) -> LRESULT {
    fn 取窗口数据(窗口: HWND) -> *mut 窗口数据 {
        let 指针 = unsafe { GetWindowLongPtrW(窗口, WINDOW_LONG_PTR_INDEX(0)) };
        指针 as *mut _
    }

    match 消息 {
        WM_SIZE => {
            // TODO 窗口大小改变
            let 宽高 = (loword(l参数.0 as u32), hiword(l参数.0 as u32));
            println!("{:?}", 宽高);

            LRESULT(1)
        }

        WM_PAINT => {
            // 绘制回调
            let 数据 = 取窗口数据(窗口);
            match (*数据).绘制回调.as_mut() {
                Some(回调) => {
                    (回调)();
                }
                None => {}
            }

            let _ = ValidateRect(窗口, None);
            LRESULT(0)
        }

        WM_DESTROY => {
            // 关闭窗口
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(窗口, 消息, w参数, l参数),
    }
}

/// 提供 RawWindowHandle, RawDisplayHandle (Windows)
#[derive(Debug, Clone)]
pub struct HandleBox {
    rd: RawDisplayHandle,
    rw: RawWindowHandle,
}

impl HandleBox {
    pub fn new(hinstance: *mut c_void, hwnd: *mut c_void) -> Self {
        let mut h = Win32WindowHandle::empty();
        h.hinstance = hinstance;
        h.hwnd = hwnd;

        let rw = RawWindowHandle::Win32(h);
        let rd = RawDisplayHandle::Windows(WindowsDisplayHandle::empty());
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

pub trait 回调接口 {
    fn 初始化(&mut self, h: HandleBox);
    fn 绘制(&mut self);
}

/// 封装窗口执行入口
pub fn pmse_win_main<T: 回调接口 + 'static>(标题: String, 宽高: (i32, i32), mut 回调: T) {
    let mut 窗口 = unsafe { 窗口封装::new(宽高, 标题) };
    回调.初始化(窗口.获取指针());

    窗口.设绘制回调(Some(Box::new(move || {
        回调.绘制();
    })));

    unsafe { 窗口.主循环() }
}
