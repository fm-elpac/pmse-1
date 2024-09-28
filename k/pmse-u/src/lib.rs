//! pmse-u 共用代码 (API 前缀: `yk` 基础层)
#![deny(unsafe_code)]

mod err;
mod sys_info;

pub use err::E;
pub use sys_info::YkSysInfo;

// TODO
