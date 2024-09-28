//! 错误类型
use std::error::Error;
use std::fmt::{Display, Formatter};

/// 自定义简单错误信息
#[derive(Debug, Clone)]
pub struct E(pub String);

impl Error for E {}

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
