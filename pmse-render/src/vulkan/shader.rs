//! vulkan shader (GLSL)

/// 计算着色器
pub mod test_c {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shader/t/test_c.glsl",
    }
}

/// 顶点着色器
pub mod test_v {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shader/t/test_v.glsl",
    }
}

/// 片段着色器
pub mod test_f {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shader/t/test_f.glsl",
    }
}
