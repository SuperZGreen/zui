use super::render_layer::RenderLayer;
use std::collections::VecDeque;

pub trait Renderable {
    fn to_render_layers(&self, viewport_dimensions_px: glam::Vec2) -> VecDeque<RenderLayer>;
}
