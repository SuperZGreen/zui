#[allow(dead_code)]
mod image_vertex;
#[allow(dead_code)]
mod texture_atlas;

pub use image_vertex::ImageVertex;
pub use texture_atlas::{SpriteId, TextureAtlas, TextureAtlasBuilder};
use wgpu::util::DeviceExt;

use crate::util;

#[allow(dead_code)]
pub struct ImageRenderer {
    render_pipeline: wgpu::RenderPipeline,
}

#[allow(dead_code)]
impl ImageRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        texture_atlas_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader_module = util::shader_module_from_file_path(
            device,
            "resources/zui/image_renderer.wgsl",
            "zui_image_renderer_shader_module",
        )
        .expect("failed to load zui image renderer shader!");

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

        Self { render_pipeline }
    }

    /// Takes an ImageRendererBuffer and draws contents to the screen.
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        texture_atlas: &'a TextureAtlas,
        buffer: &'a ImageRendererBuffer,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, texture_atlas.bind_group(), &[]);

        render_pass.set_vertex_buffer(0, buffer.vertices_buffer.slice(..));

        render_pass.draw(0..buffer.vertices_used, 0..1);

        // info!("vertices used: {}!", buffer.vertices_used);
    }
}

/// Contains all of the vertex information to be rendered by the ImageRenderer
pub struct ImageRendererBuffer {
    vertices_buffer: wgpu::Buffer,
    vertices_used: u32,
    // TODO: use indices.
    // indices_buffer: wgpu::Buffer,
    // indices_used: u32,
}

impl ImageRendererBuffer {
    /// Uploads the ImageRendererBuffer to the GPU
    pub fn upload(device: &wgpu::Device, vertices: &[ImageVertex]) -> Self {
        // raw data to send to wgpu
        let vertices_bytes = unsafe { util::slice_as_u8_slice(vertices) };

        // creating buffers
        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("image_render_buffer"),
            contents: vertices_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertices_buffer,
            vertices_used: vertices.len() as u32,
        }
    }
}
