use std::ops::Range;
use crate::{util, zui::texture_atlas::TextureAtlas, Dimensions, Rectangle};
use super::resizable_buffer::ResizableBuffer;

// use super::{
//     primitives::{Dimensions, Rectangle},
//     texture_atlas::TextureAtlas,
//     util,
// };

#[allow(dead_code)]
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct TextVertex {
    // The colour of the text to be rendered
    pub colour: glam::Vec4,

    // The UV of the TextVertex on the text texture atlas texture
    pub uv: glam::Vec2,

    // The screen space position of the text vertex
    pub position: glam::Vec2,

    // The bounds of the parent widget rectangle that contains the text, used for clipping text to
    // prevent visual overflow
    pub clip_bounds: glam::Vec4,
}

impl TextVertex {
    pub fn new(
        position: glam::Vec2,
        uv: glam::Vec2,
        colour: glam::Vec4,
        parent_rectangle: &Rectangle<i32>,
        viewport_dimensions_px: Dimensions<u32>,
    ) -> TextVertex {
        TextVertex {
            position,
            uv,
            colour,
            clip_bounds: glam::Vec4::new(
                parent_rectangle.x_min as f32,
                parent_rectangle.x_max as f32,
                // Note that we use bottom-left (0, 0) coordinates for widget positions, but
                // framebuffer coordinates use top-left (0, 0) coordinates, so this transformation
                // for the y min/max must be performed.
                (viewport_dimensions_px.height as i32 - parent_rectangle.y_max) as f32,
                (viewport_dimensions_px.height as i32 - parent_rectangle.y_min) as f32,
            ),
        }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 4] =
            wgpu::vertex_attr_array![0 => Float32x4, 1=> Float32x2, 2=> Float32x2, 3 => Float32x4];
        let vertex_buffer_layout: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        };
        vertex_buffer_layout
    }

    /// Translates the vertex by pixels
    pub fn translate_by_pixels(
        &mut self,
        pixel_translation: glam::IVec2,
        viewport_dimensions_px: Dimensions<u32>,
    ) {
        let screen_space_pixel_height = 2f32 / viewport_dimensions_px.height as f32;
        let screen_space_pixel_width = 2f32 / viewport_dimensions_px.height as f32;

        let screen_space_translation = glam::Vec2::new(
            screen_space_pixel_width * pixel_translation.x as f32,
            screen_space_pixel_height * pixel_translation.y as f32,
        );

        self.position += screen_space_translation;
        self.clip_bounds.x += pixel_translation.x as f32;
        self.clip_bounds.y += pixel_translation.x as f32;
        self.clip_bounds.z -= pixel_translation.y as f32;
        self.clip_bounds.w -= pixel_translation.y as f32;
    }
}

pub struct TextRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertices_buffer: ResizableBuffer<TextVertex>,
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

        Self {
            render_pipeline,
            vertices_buffer: ResizableBuffer::new(
                device,
                "zui_text_vertices_buffer",
                wgpu::BufferUsages::VERTEX,
            ),
        }
    }

    /// Uploads the verticies to the GPU For rendering
    // TODO: Change this to dynamically resize buffer only if too large, therefore not reinitialising the buffer every upload
    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, vertices: &[TextVertex]) {
        self.vertices_buffer.reupload(device, queue, vertices);
    }

    /// Renders the previously uploaded vertices on the GPU
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        texture_atlas: &'a TextureAtlas,
        text_vertices_range: Range<u32>,
    ) {
        let texture_atlas_bind_group = match texture_atlas.bind_group().as_ref() {
            Some(tabg) => tabg,
            None => return,
        };

        // drawing
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, texture_atlas_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertices_buffer.buffer().slice(..));
        render_pass.draw(text_vertices_range, 0..1);
    }
}
