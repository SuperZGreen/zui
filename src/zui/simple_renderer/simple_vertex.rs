use winit::dpi::PhysicalSize;

use crate::zui::{Colour, Rectangle};

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

    pub fn from_rectangle(
        rectangle: Rectangle<f32>,
        colour: Colour,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> [Self; 6] {
        let rectangle_vertices = rectangle.vertices(viewport_dimensions_px);

        let a = SimpleVertex::new(rectangle_vertices[0], colour.into());
        let b = SimpleVertex::new(rectangle_vertices[1], colour.into());
        let c = SimpleVertex::new(rectangle_vertices[2], colour.into());
        let d = SimpleVertex::new(rectangle_vertices[3], colour.into());

        [
            a, c, b, //
            b, c, d, //
        ]
    }
}
