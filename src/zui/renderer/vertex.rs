#[allow(dead_code)]
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
    colour: glam::Vec4,
    position: glam::Vec2,
}

impl SimpleVertex {
    pub fn new(position: glam::Vec2, colour: glam::Vec4) -> SimpleVertex {
        SimpleVertex { position, colour }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x4, 1=> Float32x2];
        let vertex_buffer_layout: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SimpleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        };
        vertex_buffer_layout
    }
}