use wgpu::util::DeviceExt;

use super::{texture_atlas::TextureAtlas, util};

#[allow(dead_code)]
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct TextVertex {
    colour: glam::Vec4,
    uv: glam::Vec2,
    position: glam::Vec2,
}

impl TextVertex {
    pub fn new(position: glam::Vec2, uv: glam::Vec2, colour: glam::Vec4) -> TextVertex {
        TextVertex {
            position,
            uv,
            colour,
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] =
            wgpu::vertex_attr_array![0 => Float32x4, 1=> Float32x2, 2=> Float32x2];
        let vertex_buffer_layout: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        };
        vertex_buffer_layout
    }
}

pub struct TextRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertices_buffer: wgpu::Buffer,
    vertices_used: u32,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        texture_atlas_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader_module = util::shader_module_from_file_path(
            device,
            "resources/zui/text_renderer.wgsl",
            "zui_shader_module",
        )
        .expect("failed to load zui shader!");

        let vertex_buffer_layout = TextVertex::vertex_buffer_layout();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("zui_text_renderer_pipeline_layout"),
            bind_group_layouts: &[texture_atlas_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("zui_text_renderer_render_pipeline"),
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

        let vertices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("zui_vertices_buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        Self {
            render_pipeline,
            vertices_buffer,
            vertices_used: 0u32,
        }
    }

    /// Uploads the verticies to the GPU For rendering
    // TODO: Change this to dynamically resize buffer only if too large, therefore not reinitialising the buffer every upload
    pub fn upload(&mut self, device: &wgpu::Device, vertices: &[TextVertex]) {
        // uploading vertices buffer
        let vertices_bytes = unsafe { util::slice_as_u8_slice(&vertices) };
        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("zui_vertices_buffer"),
            contents: vertices_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertices_used = vertices.len() as u32;
        self.vertices_buffer = vertices_buffer;
    }

    /// Renders the previously uploaded vertices on the GPU
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        texture_atlas: &'a TextureAtlas,
    ) {
        // drawing
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &texture_atlas.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertices_buffer.slice(..));
        render_pass.draw(0..self.vertices_used as u32, 0..1);
    }
}