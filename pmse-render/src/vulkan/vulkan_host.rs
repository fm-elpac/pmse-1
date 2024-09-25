//! 初始化后的 vulkan
use std::sync::Arc;

use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    device::{physical::PhysicalDevice, Device, Queue},
    memory::allocator::StandardMemoryAllocator,
    swapchain::Surface,
};

/// vulkan 宿主
#[derive(Debug, Clone)]
pub struct PmseRenderHost {
    /// 物理设备
    p: Arc<PhysicalDevice>,
    /// vulkan 设备
    d: Arc<Device>,
    /// vulkan 队列
    q: Arc<Queue>,
    /// 内存分配器
    ma: Arc<StandardMemoryAllocator>,
    /// 窗口表面
    s: Arc<Surface>,
}

impl PmseRenderHost {
    pub(crate) fn new(
        p: Arc<PhysicalDevice>,
        d: Arc<Device>,
        q: Arc<Queue>,
        ma: Arc<StandardMemoryAllocator>,
        s: Arc<Surface>,
    ) -> Self {
        Self { p, d, q, ma, s }
    }

    /// vulkan PhysicalDevice
    pub fn p(&self) -> &Arc<PhysicalDevice> {
        &self.p
    }

    /// vulkan Device
    pub fn d(&self) -> &Arc<Device> {
        &self.d
    }

    /// vulkan Queue
    pub fn q(&self) -> &Arc<Queue> {
        &self.q
    }

    /// 内存分配器
    pub fn ma(&self) -> &Arc<StandardMemoryAllocator> {
        &self.ma
    }

    /// vulkan Surface
    pub fn s(&self) -> &Arc<Surface> {
        &self.s
    }

    /// 创建 命令缓冲区 分配器
    pub fn ca(&self) -> StandardCommandBufferAllocator {
        StandardCommandBufferAllocator::new(
            self.d.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        )
    }
}
