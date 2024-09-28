//! 工具函数
use std::error::Error;
use std::sync::Arc;

use vulkano::{
    command_buffer::PrimaryAutoCommandBuffer,
    device::{Device, Queue},
    sync::{self, GpuFuture},
};

/// 提交 GPU 执行命令, 等待执行完毕
pub fn sr_提交_gpu_执行等待(
    设备: &Arc<Device>,
    队列: &Arc<Queue>,
    命令: &Arc<PrimaryAutoCommandBuffer>,
) -> Result<(), Box<dyn Error>> {
    let f = sync::now(设备.clone())
        .then_execute(队列.clone(), 命令.clone())?
        .then_signal_fence_and_flush()?;
    f.wait(None)?;
    Ok(())
}
