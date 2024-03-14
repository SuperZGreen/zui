use self::texture_atlas::TextureAtlas;

mod texture_atlas;

pub struct ImageRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertices_buffer: wgpu::Buffer,
    vertices_used: u32,
}

impl ImageRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        texture_atlas: TextureAtlas
    ) -> Self {
        todo!()
    }
}
