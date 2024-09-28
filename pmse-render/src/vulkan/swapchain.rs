//! 交换链 (swapchain) 创建/初始化
use std::error::Error;
use std::sync::Arc;

use log::{debug, error};
use vulkano::{
    command_buffer::PrimaryAutoCommandBuffer,
    device::{physical::PhysicalDevice, Device, Queue},
    image::{view::ImageView, Image, ImageUsage},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{self, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
    sync::{self, GpuFuture},
    Validated, VulkanError,
};

use pmse_u::E;

use super::SrVk1;

/// 交换链 (swapchain) 管理
#[derive(Debug)]
pub struct SrVkSwapchain {
    /// 设备
    设备: Arc<Device>,
    /// 队列
    队列: Arc<Queue>,
    /// (no Clone) 交换链
    交换链: Arc<Swapchain>,
    /// 图像
    图像: Vec<Arc<Image>>,
    /// 帧缓冲区
    帧缓冲: Vec<Arc<Framebuffer>>,
}

impl SrVkSwapchain {
    pub fn new(h: &SrVk1, size: [u32; 2]) -> Result<Self, Box<dyn Error>> {
        let (交换链, 图像) = 创建交换链(h.d(), h.p(), h.s(), size)?;
        // 稍后初始化 帧缓冲
        Ok(Self {
            设备: h.d().clone(),
            队列: h.q().clone(),
            交换链,
            图像,
            帧缓冲: vec![],
        })
    }

    /// 获取 交换链
    pub fn sc(&self) -> &Arc<Swapchain> {
        &self.交换链
    }

    /// 创建帧缓冲区
    pub fn init_framebuffer(&mut self, rp: &Arc<RenderPass>) -> Result<(), Box<dyn Error>> {
        self.帧缓冲 = 创建帧缓冲区(&self.图像, rp)?;
        Ok(())
    }

    /// 交换链 执行命令
    pub fn execute<T: CreateCommand>(&self, c: &T) -> Result<(), Box<dyn Error>> {
        交换链执行(&self.设备, &self.队列, &self.交换链, &self.帧缓冲, c)
    }
}

/// 生成命令
pub trait CreateCommand {
    /// 生成绘制命令
    fn c(&self, fb: &Arc<Framebuffer>) -> Result<Arc<PrimaryAutoCommandBuffer>, Box<dyn Error>>;
}

/// 创建交换链
fn 创建交换链(
    设备: &Arc<Device>,
    物理设备: &Arc<PhysicalDevice>,
    表面: &Arc<Surface>,
    image_extent: [u32; 2],
) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), Box<dyn Error>> {
    let 能力 = 物理设备.surface_capabilities(表面, Default::default())?;
    let composite_alpha = 能力.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = 物理设备.surface_formats(表面, Default::default())?[0].0;
    debug!("  image format: {:?}", image_format);
    debug!("  min_image_count {}", 能力.min_image_count);

    Ok(Swapchain::new(
        设备.clone(),
        表面.clone(),
        SwapchainCreateInfo {
            min_image_count: 能力.min_image_count + 1,
            image_format,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )?)
}

/// 创建帧缓冲区
fn 创建帧缓冲区(
    图像: &Vec<Arc<Image>>,
    渲染过程: &Arc<RenderPass>,
) -> Result<Vec<Arc<Framebuffer>>, Box<dyn Error>> {
    let mut o: Vec<Arc<Framebuffer>> = Vec::new();
    for i in 图像 {
        let 视图 = ImageView::new_default(i.clone())?;
        o.push(Framebuffer::new(
            渲染过程.clone(),
            FramebufferCreateInfo {
                attachments: vec![视图],
                ..Default::default()
            },
        )?)
    }
    Ok(o)
}

/// 从交换链获取一个图像, 绘制
fn 交换链执行<T: CreateCommand>(
    设备: &Arc<Device>,
    队列: &Arc<Queue>,
    交换链: &Arc<Swapchain>,
    帧缓冲: &Vec<Arc<Framebuffer>>,
    生成命令: &T,
) -> Result<(), Box<dyn Error>> {
    // 从交换链获取下一个图像
    let (序号, _退化, 获取未来) =
        match swapchain::acquire_next_image(交换链.clone(), None).map_err(Validated::unwrap) {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                // TODO 重新创建交换链
                error!("ERROR swapchain acquire OutOfDate");
                return Err(Box::new(E("vulkan OutOfDate".into())));
            }
            Err(e) => {
                // TODO unknown error
                error!("ERROR swapchain acquire {}", e);
                return Err(Box::new(E("unknown error".into())));
            }
        };
    // 生成命令
    let 命令 = 生成命令.c(&帧缓冲[序号 as usize])?;
    // 绘制
    let 执行 = sync::now(设备.clone())
        .join(获取未来)
        .then_execute(队列.clone(), 命令)?
        .then_swapchain_present(
            队列.clone(),
            SwapchainPresentInfo::swapchain_image_index(交换链.clone(), 序号),
        )
        .then_signal_fence_and_flush();
    // 错误处理
    match 执行.map_err(Validated::unwrap) {
        Ok(f) => {
            f.wait(None)?;
        }
        Err(e) => {
            // TODO unknown error
            error!("ERROR flush {}", e);
        }
    }
    Ok(())
}
