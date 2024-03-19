use crate::zui::{primitives::Dimensions, Rectangle};

use super::texture_atlas::packed_sprite::PackedSprite;

#[allow(dead_code)]
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct ImageVertex {
    position: glam::Vec2,
    uv: glam::Vec2,
}

impl ImageVertex {
    pub fn new(position: glam::Vec2, uv: glam::Vec2) -> ImageVertex {
        ImageVertex { position, uv }
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x2, 1=> Float32x2];
        let vertex_buffer_layout: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ImageVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        };
        vertex_buffer_layout
    }

    pub fn from_rectangle_and_packed_sprite(
        rectangle: Rectangle<i32>,
        viewport_dimensions_px: Dimensions<u32>,
        packed_sprite: &PackedSprite,
    ) -> [Self; 6] {
        let rectangle_vertices = rectangle.vertices(viewport_dimensions_px);
        let sprite_uvs = packed_sprite.uvs();

        let a = ImageVertex::new(rectangle_vertices[0], sprite_uvs[0]);
        let b = ImageVertex::new(rectangle_vertices[1], sprite_uvs[1]);
        let c = ImageVertex::new(rectangle_vertices[2], sprite_uvs[2]);
        let d = ImageVertex::new(rectangle_vertices[3], sprite_uvs[3]);

        [
            a, c, b, //
            b, c, d, //
        ]
    }
}
