//! 通用数据类型

use super::ObjR;

/// (物理) 小宇宙
///
/// 可以同时存在多个小宇宙, 每个小宇宙里有若干物体.
/// 同一个小宇宙中的物体计算相互作用, 不同小宇宙中的物体互不影响.
#[derive(Debug, Clone)]
pub struct LuP {
    /// 小宇宙中的物体列表
    pub o: Vec<ObjP>,
    // TODO
}

impl Default for LuP {
    fn default() -> Self {
        Self { o: Vec::new() }
    }
}

/// (物理) 物体, 可能有多种类型
#[derive(Debug, Clone)]
pub enum ObjP {
    /// 刚体, 不会形变
    R(ObjR),
    // TODO
}
