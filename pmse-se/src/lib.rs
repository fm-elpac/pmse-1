//! pmse-se 仿真层 (近似模拟, 动力学, 物理引擎)
//!
//! TODO 主要功能模块:
//!
//! + `er`: 条件触发与事件上报系统
//! + `or`: 刚体运动计算
//! + `cd`: 碰撞检测
//!
//! + `pd`: 动力学 (牛顿力学)
//! + `ps`: 空间声学 (音效)
//! + `pe`: 电路 (数字电路/模拟电路) (电磁)
//! + `pl`: 光学 (应用光学)
//! + `pt`: 热力学
//! + `pf`: 流体力学 (空气动力学)
//! + `pq`: 量子力学 (微观)
//! + `pg`: 相对论 (宏观)
#![deny(unsafe_code)]

mod or;
mod t;

pub use or::{ObjLR, ObjR};
pub use t::{LuP, ObjP};

// TODO

#[cfg(test)]
mod tests {
    // TODO
}
