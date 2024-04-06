mod image_vertex;
mod texture_atlas;

use std::ops::Range;

pub use image_vertex::ImageVertex;
pub use texture_atlas::{SpriteId, TextureAtlas, TextureAtlasBuilder};

use super::resizeable_buffer::ResizeableBuffer;

pub struct ImageRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertices_buffer: ResizeableBuffer<ImageVertex>,
}

impl ImageRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        texture_atlas_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        // included in source rather than loading dynamically, as dynamic loading complicates the
        // crate
        let shader_source = std::include_str!(std::concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/resources/zui/image_renderer.wgsl"
        ));

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("zui_simple_renderer_shader_module"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let vertex_buffer_layout = ImageVertex::vertex_buffer_layout();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("zui_image_renderer_pipeline_layout"),
            bind_group_layouts: &[texture_atlas_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("zui_image_renderer_render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_configuration.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            render_pipeline,
            vertices_buffer: ResizeableBuffer::new(
                device,
                "zui_image_vertices_buffer",
                wgpu::BufferUsages::VERTEX,
            ),
        }
    }

    /// Uploads the verticies to the GPU For rendering via the internal ResizeableBuffer
    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, vertices: &[ImageVertex]) {
        self.vertices_buffer.reupload(device, queue, vertices);
    }

    /// Takes an ImageRendererBuffer and draws contents to the screen.
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        texture_atlas: &'a TextureAtlas,
        vertices_range: Range<u32>,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, texture_atlas.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertices_buffer.buffer().slice(..));
        render_pass.draw(vertices_range, 0..1);
    }
}
