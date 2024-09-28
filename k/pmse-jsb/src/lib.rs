//! pmse-jsb (API 前缀: `se` 仿真层)
#![deny(unsafe_code)]

use std::error::Error;

// re-export
pub use rquickjs;

use rquickjs::{Context, Runtime, Value};

/// 用来运行 JS 代码的虚拟机 (QuickJS Runtime)
pub struct SeVm {
    // no Debug
    实例: Runtime,
    语境: Context,
}

impl SeVm {
    /// 创建新实例
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let 实例 = Runtime::new()?;
        let 语境 = Context::full(&实例)?;

        Ok(Self { 实例, 语境 })
    }

    /// (内部) 运行一段 JS 代码, 返回结果.
    fn run_js<T, F>(&mut self, code: &str, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce(Value) -> T,
    {
        self.语境.with(move |c| {
            let r = c.eval(code)?;
            Ok(f(r))
        })
    }

    /// (封装) 运行一段 JS 代码, 返回结果.
    pub fn run_r<T, F>(&mut self, code: &str, f: F) -> Result<T, Box<dyn Error>>
    where
        F: FnOnce(Value) -> T,
    {
        self.run_js(code, f)
    }

    /// (封装) 运行一段 JS 代码, 无需结果.
    pub fn run(&mut self, code: &str) -> Result<(), Box<dyn Error>> {
        self.run_js(code, |_| ())?;
        Ok(())
    }

    /// 手动运行 GC (垃圾收集)
    pub fn gc(&self) {
        self.实例.run_gc();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vm = SeVm::new().unwrap();
        let r = vm.run_r("1 + 2", |r| r.as_int().unwrap()).unwrap();
        assert_eq!(r, 3);
    }
}
