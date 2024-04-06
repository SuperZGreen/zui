mod simple_vertex;

use std::ops::Range;

pub use simple_vertex::SimpleVertex;

use super::resizeable_buffer::ResizeableBuffer;

pub struct SimpleRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertices_buffer: ResizeableBuffer<SimpleVertex>,
}

impl SimpleRenderer {
    pub fn new(device: &wgpu::Device, surface_configuration: &wgpu::SurfaceConfiguration) -> Self {
        // included in source rather than loading dynamically, as dynamic loading complicates the
        // crate
        let shader_source = std::include_str!(std::concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/resources/zui/simple_renderer.wgsl"
        ));

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("zui_simple_renderer_shader_module"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let vertex_buffer_layout = SimpleVertex::vertex_buffer_layout();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("zui_simple_renderer_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("zui_simple_renderer_render_pipeline"),
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
                "zui_simple_vertex_buffer",
                wgpu::BufferUsages::VERTEX,
            ),
        }
    }

    /// Uploads the verticies to the GPU For rendering
    // TODO: Change this to dynamically resize buffer only if too large, therefore not
    // reinitialising the buffer every upload
    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[SimpleVertex],
    ) {
        self.vertices_buffer.reupload(device, queue, vertices);
    }

    /// Renders the previously uploaded vertices on the GPU within a range provided by a render
    /// layer.
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        simple_vertices_range: Range<u32>,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertices_buffer.buffer().slice(..));
        render_pass.draw(simple_vertices_range, 0..1);
    }
}
