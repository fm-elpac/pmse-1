//! pmse-v8
#![deny(unsafe_code)]

use std::error::Error;

// re-export
pub use v8;

use v8::{
    Context, ContextScope, CreateParams, HandleScope, Isolate, Local, OwnedIsolate, Script, Value,
    V8,
};

mod err;

pub use err::E;

/// 注意: 只能调用一次
fn 初始化_v8() {
    let 平台 = v8::new_default_platform(0, false).make_shared();
    V8::initialize_platform(平台);
    V8::initialize();
}

/// 创建 v8 实例 (Isolate)
fn 创建实例() -> OwnedIsolate {
    Isolate::new(CreateParams::default())
}

/// 编译 JS 代码字符串
fn 编译_js<'a>(
    范围: &mut HandleScope<'a>,
    代码: &str,
) -> Result<Local<'a, Script>, Box<dyn Error>> {
    // 代码转换为 v8 字符串
    let 代码 = v8::String::new(范围, 代码).ok_or(E("v8 String::new".into()))?;
    // 编译源代码
    let 脚本 = Script::compile(范围, 代码, None).ok_or(E("v8 Script::compile".into()))?;

    Ok(脚本)
}

/// 用来运行 JS 代码的虚拟机 (Isolate)
#[derive(Debug)]
pub struct Vm {
    隔离: OwnedIsolate,
}

impl Vm {
    /// 初始化 v8
    pub fn new() -> Self {
        初始化_v8();
        let 隔离 = 创建实例();

        Self { 隔离 }
    }

    /// (内部) 运行一段 JS 代码, 返回结果.
    fn run_js<T, F>(&mut self, code: &str, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce(&mut ContextScope<HandleScope>, Option<Local<Value>>) -> T,
    {
        // 创建 HandleScope, ContextScope
        // 注意: 这段代码不能拆分 (rusty_v8 的设计如此)
        let mut 柄范围 = HandleScope::new(&mut self.隔离);
        let 语境 = Context::new(&mut 柄范围, Default::default());
        let mut 范围 = ContextScope::new(&mut 柄范围, 语境);

        let 脚本 = 编译_js(&mut 范围, code)?;
        let 结果 = 脚本.run(&mut 范围);

        let o = f(&mut 范围, 结果);
        Ok(o)
    }

    /// (封装) 运行一段 JS 代码, 返回结果.
    pub fn run_r<T, F>(&mut self, code: &str, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce(&mut ContextScope<HandleScope>, Option<Local<Value>>) -> T,
    {
        self.run_js(code, f)
    }

    /// (封装) 运行一段 JS 代码, 无需结果.
    pub fn run(&mut self, code: &str) -> Result<(), Box<dyn Error>> {
        self.run_js(code, |_, _| ())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vm = Vm::new();
        let r = vm
            .run_r(
                "1 + 2",
                |s: &mut ContextScope<HandleScope>, v: Option<Local<Value>>| {
                    v.unwrap().uint32_value(s)
                },
            )
            .unwrap();
        assert_eq!(r, Some(3));
    }
}
