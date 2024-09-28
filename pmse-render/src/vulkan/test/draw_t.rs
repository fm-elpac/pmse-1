//! vulkan 渲染测试: 绘制三角形
use std::error::Error;
use std::sync::Arc;

use log::debug;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
        SubpassEndInfo,
    },
    device::{Device, Queue},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, RenderPass, Subpass},
    shader::EntryPoint,
    swapchain::Swapchain,
};

use pmse_u::E;

use super::super::{shader, CreateCommand, SrVk1, SrVkSwapchain};

/// 要绘制的三角形顶点数据
#[derive(Debug, Clone)]
pub struct 三角形 {
    /// 顶点位置坐标 (x, y, z)
    位置: [[f32; 3]; 3],
    /// 顶点颜色 (RGB)
    颜色: [[f32; 3]; 3],
}

impl 三角形 {
    pub fn new(位置: [[f32; 3]; 3], 颜色: [[f32; 3]; 3]) -> Self {
        Self { 位置, 颜色 }
    }

    /// 输出顶点
    pub(self) fn 生成顶点(&self, 输出: &mut Vec<顶点>) {
        for i in 0..3 {
            输出.push(顶点 {
                p: self.位置[i],
                color: self.颜色[i],
            });
        }
    }
}

impl Default for 三角形 {
    /// 默认测试用三角形顶点数据
    fn default() -> Self {
        Self {
            位置: [[0.1, 0.8, 0.0], [-0.8, -0.6, 0.0], [0.9, -0.9, 0.0]],
            颜色: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

/// 顶点数据结构
#[derive(Debug, Clone, BufferContents, Vertex)]
#[repr(C)]
struct 顶点 {
    /// 位置坐标 (x, y, z)
    #[format(R32G32B32_SFLOAT)]
    pub p: [f32; 3],
    /// 颜色
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}

/// vulkan 绘制三角形
#[derive(Debug)]
pub struct SrT {
    h: SrVk1,
    /// 交换链
    sc: SrVkSwapchain,
    /// 命令缓冲区 分配器
    ca: Arc<StandardCommandBufferAllocator>,
    /// 图形管线
    管线: Arc<GraphicsPipeline>,
}

impl SrT {
    /// 初始化
    pub fn new(h: SrVk1, size: (u32, u32)) -> Result<Self, Box<dyn Error>> {
        // 创建交换链
        let mut sc = SrVkSwapchain::new(&h, size.into())?;

        let (阶段, 顶点入口) = 加载着色器(h.d())?;
        let (渲染过程, 分过程, 布局) = 创建渲染过程(h.d(), &阶段, sc.sc())?;
        // 初始化 帧缓冲区
        sc.init_framebuffer(&渲染过程)?;

        let 视口 = Viewport {
            offset: [0.0, 0.0],
            extent: [size.0 as f32, size.1 as f32],
            depth_range: 0.0..=1.0,
        };
        let 管线 = 创建图形管线(h.d(), 顶点入口, 阶段, 视口, 分过程, 布局)?;

        let ca = Arc::new(h.ca());
        Ok(Self { h, sc, ca, 管线 })
    }

    /// 绘制三角形
    pub fn draw(&self, 列表: Vec<三角形>) -> Result<(), Box<dyn Error>> {
        debug!("vulkan_test T");

        let 顶点数据 = 创建顶点缓冲区(self.h.ma(), 列表)?;
        let c = 命令生成器::new(
            self.ca.clone(),
            self.h.q().clone(),
            self.管线.clone(),
            顶点数据,
        );

        self.sc.execute(&c)?;
        Ok(())
    }
}

struct 命令生成器 {
    ca: Arc<StandardCommandBufferAllocator>,
    q: Arc<Queue>,
    管线: Arc<GraphicsPipeline>,
    /// 顶点数据
    顶点缓冲区: Subbuffer<[顶点]>,
}

impl 命令生成器 {
    pub fn new(
        ca: Arc<StandardCommandBufferAllocator>,
        q: Arc<Queue>,
        管线: Arc<GraphicsPipeline>,
        顶点缓冲区: Subbuffer<[顶点]>,
    ) -> Self {
        Self {
            ca,
            q,
            管线,
            顶点缓冲区,
        }
    }
}

impl CreateCommand for 命令生成器 {
    fn c(&self, fb: &Arc<Framebuffer>) -> Result<Arc<PrimaryAutoCommandBuffer>, Box<dyn Error>> {
        创建命令缓冲区(&self.ca, &self.q, &self.管线, fb, &self.顶点缓冲区)
    }
}

/// 初始化 (加载/编译) 着色器
fn 加载着色器(
    设备: &Arc<Device>,
) -> Result<([PipelineShaderStageCreateInfo; 2], EntryPoint), Box<dyn Error>> {
    let 顶点着色器 = shader::test_v::load(设备.clone())?;
    let 片段着色器 = shader::test_f::load(设备.clone())?;

    // 着色器 入口函数
    let 顶点入口 = 顶点着色器
        .entry_point("main")
        .ok_or(E("ERROR vulkan shader vs main".into()))?;
    let 片段入口 = 片段着色器
        .entry_point("main")
        .ok_or(E("ERROR vulkan shader fs main".into()))?;

    let 阶段 = [
        PipelineShaderStageCreateInfo::new(顶点入口.clone()),
        PipelineShaderStageCreateInfo::new(片段入口),
    ];

    Ok((阶段, 顶点入口))
}

/// 创建渲染过程
fn 创建渲染过程(
    设备: &Arc<Device>,
    阶段: &[PipelineShaderStageCreateInfo; 2],
    交换链: &Arc<Swapchain>,
) -> Result<(Arc<RenderPass>, Subpass, Arc<PipelineLayout>), Box<dyn Error>> {
    let 布局 = PipelineLayout::new(
        设备.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(阶段)
            .into_pipeline_layout_create_info(设备.clone())?,
    )?;
    let 渲染过程 = vulkano::single_pass_renderpass!(
        设备.clone(),
        attachments: {
            color: {
                format: 交换链.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {},
        }
    )?;
    let 分过程 = Subpass::from(渲染过程.clone(), 0).unwrap();

    Ok((渲染过程, 分过程, 布局))
}

/// 创建图形渲染管线
fn 创建图形管线(
    设备: &Arc<Device>,
    顶点入口: EntryPoint,
    阶段: [PipelineShaderStageCreateInfo; 2],
    视口: Viewport,
    分过程: Subpass,
    布局: Arc<PipelineLayout>,
) -> Result<Arc<GraphicsPipeline>, Box<dyn Error>> {
    let 顶点输入状态 = 顶点::per_vertex().definition(&顶点入口.info().input_interface)?;
    let 管线 = GraphicsPipeline::new(
        设备.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: 阶段.into_iter().collect(),
            vertex_input_state: Some(顶点输入状态),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState {
                viewports: [视口].into_iter().collect(),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::with_attachment_states(
                分过程.num_color_attachments(),
                ColorBlendAttachmentState::default(),
            )),
            subpass: Some(分过程.into()),
            ..GraphicsPipelineCreateInfo::layout(布局)
        },
    )?;
    Ok(管线)
}

/// 创建 顶点数据缓冲区 (三角形)
fn 创建顶点缓冲区(
    ma: &Arc<StandardMemoryAllocator>,
    数据: Vec<三角形>,
) -> Result<Subbuffer<[顶点]>, Box<dyn Error>> {
    let mut 顶点数据: Vec<顶点> = Vec::new();
    for i in 数据 {
        i.生成顶点(&mut 顶点数据);
    }

    Ok(Buffer::from_iter(
        ma.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        顶点数据,
    )?)
}

/// 创建 命令缓冲区
fn 创建命令缓冲区(
    ca: &StandardCommandBufferAllocator,
    队列: &Arc<Queue>,
    图形管线: &Arc<GraphicsPipeline>,
    帧缓冲区: &Arc<Framebuffer>,
    顶点缓冲区: &Subbuffer<[顶点]>,
) -> Result<Arc<PrimaryAutoCommandBuffer>, Box<dyn Error>> {
    let mut b = AutoCommandBufferBuilder::primary(
        ca,
        队列.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )?;
    // 渲染命令
    b.begin_render_pass(
        RenderPassBeginInfo {
            clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
            ..RenderPassBeginInfo::framebuffer(帧缓冲区.clone())
        },
        SubpassBeginInfo {
            contents: SubpassContents::Inline,
            ..Default::default()
        },
    )?
    .bind_pipeline_graphics(图形管线.clone())?
    .bind_vertex_buffers(0, 顶点缓冲区.clone())?
    .draw(顶点缓冲区.len() as u32, 1, 0, 0)?
    .end_render_pass(SubpassEndInfo::default())?;

    Ok(b.build()?)
}
