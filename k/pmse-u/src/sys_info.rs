//! 系统信息

/// 运行平台的系统 (硬件配置) 信息
#[derive(Debug, Clone)]
pub struct YkSysInfo {
    /// CPU (核心) 数量 (单位: 1)
    pub cpu_n: u32,
    /// CPU (核心) (基准) 运行频率 (速度) (单位: MHz)
    pub cpu_mhz: u32,
    /// 设备支持的 vulkan 版本号
    pub vulkan_v: String,
    /// 内存容量 (单位: GB)
    pub ram_gb: u32,
    /// 存储 (磁盘/闪存) 容量 (单位: GB)
    pub md_gb: u32,
    /// 显示分辨率 (渲染视口宽高)
    pub viewport_wh: (u32, u32),
}

impl Default for YkSysInfo {
    /// 默认值 (获取失败)
    fn default() -> Self {
        Self {
            // 默认 8 核
            cpu_n: 8,
            // 默认 1.0GHz
            cpu_mhz: 1000,
            // 默认 vulkan1.1
            vulkan_v: "1.1".into(),
            // 默认 8GB
            ram_gb: 8,
            // 默认 256GB
            md_gb: 256,
            // 默认 1280x720
            viewport_wh: (1280, 720),
        }
    }
}
