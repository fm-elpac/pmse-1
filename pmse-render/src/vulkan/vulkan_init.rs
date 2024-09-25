//! vulkan 初始化
use std::error::Error;
use std::sync::Arc;

use log::debug;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use vulkano::{
    device::{
        physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::StandardMemoryAllocator,
    swapchain::Surface,
    VulkanLibrary,
};

use super::PmseRenderHost;
use crate::E;

/// 正在初始化的 vulkan 渲染器
#[derive(Debug, Clone)]
pub struct PmseRenderInit {
    库: Arc<VulkanLibrary>,
}

impl PmseRenderInit {
    /// 初始化 vulkan (加载 vulkan 库 .so)
    pub fn vulkan() -> Result<Self, Box<dyn Error>> {
        debug!("init vulkan .. .");

        let 库 = VulkanLibrary::new()?;
        Ok(Self { 库 })
    }

    /// 窗口初始化, 传入窗口
    pub fn init_w(
        self,
        w: Arc<impl HasRawDisplayHandle + HasRawWindowHandle + Send + Sync + 'static>,
    ) -> Result<PmseRenderHost, Box<dyn Error>> {
        let 实例扩展 = Surface::required_extensions(w.as_ref());
        // 创建 vulkan 实例
        let 实例 = Instance::new(
            self.库.clone(),
            InstanceCreateInfo {
                enabled_extensions: 实例扩展,
                ..Default::default()
            },
        )?;

        // 创建设备队列
        let 设备扩展 = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let (物理设备, 队列序号) = 选择设备(&实例, 设备扩展.clone())?;
        let (设备, 队列) = 创建设备队列(&物理设备, 队列序号, 设备扩展)?;

        // 创建内存分配器
        let ma = Arc::new(StandardMemoryAllocator::new_default(设备.clone()));
        // 创建窗口表面
        let 表面 = Surface::from_window(实例.clone(), w)?;

        // 初始化 (这部分) 完成
        Ok(PmseRenderHost::new(物理设备, 设备, 队列, ma, 表面))
    }
}

// (函数)

/// 选择 vulkan 设备
fn 选择设备(
    实例: &Arc<Instance>,
    扩展: DeviceExtensions,
) -> Result<(Arc<PhysicalDevice>, u32), Box<dyn Error>> {
    // TODO 优化设备选择功能

    // 列出 (枚举) vulkan 设备
    let 设备列表 = 实例
        .enumerate_physical_devices()?
        .filter(|p| p.supported_extensions().contains(&扩展));
    let mut d1: Option<Arc<PhysicalDevice>> = None;
    for i in 设备列表 {
        // 输出设备列表, 选择第一个 vulkan 设备
        debug!("  {}", i.properties().device_name);
        if d1.is_none() {
            d1.replace(i);
        }
    }
    let 设备 = d1.ok_or(E("ERROR vulkan list device".into()))?;

    // 列出 (查找) vulkan 队列
    for f in 设备.queue_family_properties() {
        debug!("vulkan device queue {:?}", f.queue_count);
    }
    let queue_family_index = 设备
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_i, q)| q.queue_flags.contains(QueueFlags::GRAPHICS))
        .ok_or(E("ERROR vulkan find queue".into()))? as u32;
    debug!("vulkan queue index {}", queue_family_index);

    Ok((设备, queue_family_index))
}

/// 创建 vulkan 设备, 队列
fn 创建设备队列(
    设备: &Arc<PhysicalDevice>,
    queue_family_index: u32,
    enabled_extensions: DeviceExtensions,
) -> Result<(Arc<Device>, Arc<Queue>), Box<dyn Error>> {
    let (d, mut 队列) = Device::new(
        设备.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions,
            ..Default::default()
        },
    )?;
    let q = 队列.next().unwrap();
    Ok((d, q))
}
