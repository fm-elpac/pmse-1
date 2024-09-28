//! 刚体 (object rigid) 运动计算

/// (物体) 刚体: 不会形变
#[derive(Debug, Clone)]
pub struct SeObjR {
    // + (1) 物体固有物理量
    /// 质量 (单位: kg)
    pub m: f32,
    /// 转动惯量 (单位: kg/m2)
    pub rm: f32,

    // + (2) 平动物理量
    /// 位置 (单位: m) (x, y, z)
    pub x: [f32; 3],
    /// 速度 (单位: m/s) (x, y, z) None 表示 固定不动 的物体.
    pub v: Option<[f32; 3]>,
    /// 加速度 (单位: m/s2) (x, y, z) None 表示 匀速直线运动 的物体.
    pub a: Option<[f32; 3]>,

    // + (3) 转动物理量
    /// 角度 (旋转) (单位: rad) (x, y, z)
    pub r: [f32; 3],
    /// 角速度 (单位: rad/s) (x, y, z) None 表示 不旋转 的物体.
    pub rv: Option<[f32; 3]>,
    /// 角加速度 (单位: rad/s2) (x, y, z) None 表示 匀速旋转 的物体.
    pub ra: Option<[f32; 3]>,

    // + (4) 加速计算物理量
    /// 最大半径 (单位: m): 物体任意一点, 距离重心的最大距离.
    /// None 表示半径为 0.
    pub cr: Option<f32>,
    /// 物体的体积 (单位: m3). None 表示体积为 0.
    pub cv: Option<f32>,
}

impl Default for SeObjR {
    fn default() -> Self {
        Self {
            m: 0.0,
            rm: 0.0,
            x: [0.0, 0.0, 0.0],
            v: None,
            a: None,
            r: [0.0, 0.0, 0.0],
            rv: None,
            ra: None,
            cr: None,
            cv: None,
        }
    }
}

/// 刚体 模拟计算的数值限制 (下限, 上限)
#[derive(Debug, Clone)]
pub struct SeObjLR {
    /// 模拟计算的 时间 间隔 (单位: s) [最小值, 最大值]
    pub t: [f32; 2],

    /// 物体质量 (单位: kg)
    pub m: [f32; 2],
    /// 转动惯量 (单位: kg/m2)
    pub rm: [f32; 2],
    /// 物体位置坐标值 (单位: m)
    pub x: [f32; 2],
    /// 物体的速度 (单位: m/s)
    pub v: [f32; 2],
    /// 物体的加速度 (单位: m/s2)
    pub a: [f32; 2],

    // TODO 角度 (单位: rad) (x, y, z) None 表示不限制.
    //r: Option<[[f32; 2]; 3]>,
    /// 角速度 (单位: rad/s)
    pub rv: [f32; 2],
    /// 角加速度 (单位: rad/s2)
    pub ra: [f32; 2],
}

impl Default for SeObjLR {
    fn default() -> Self {
        Self {
            // 模拟计算中允许的时间间隔: 最小 0.001s (1ms), 最大 1s
            t: [0.001, 1.0],
            // 允许的物体质量: 最小 0.001kg (1g), 最大 100 万吨 (1Gkg)
            m: [0.001, 1.0e9],
            // 允许的物体转动惯量: 最小 1u kg/m2, 最大 1T kg/m2
            rm: [1.0e-6, 1.0e12],
            // 允许的物体位置坐标: 最小 1um, 最大 200km.
            x: [1.0e-6, 2.0e5],
            // 允许的物体速度: 最小 1um/s, 最大 300m/s.
            v: [1.0e-6, 300.0],
            // 允许的物体加速度: 最小 1um/s2, 最大 300km/s.
            a: [1.0e-6, 3.0e5],
            // TODO 角度: 不限制.
            //r: None,
            // 角速度: 最小 1u rad/s, 最大 10k rad/s.
            rv: [1.0e-6, 1.0e4],
            // 角加速度: 最小 1u rad/s2, 最大 100k rad/s2.
            ra: [1.0e-6, 1.0e5],
        }
    }
}
