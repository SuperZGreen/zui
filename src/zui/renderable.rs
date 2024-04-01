use super::{render_layer::RenderLayers, Context};

pub trait Renderable {
    fn to_render_layers(&self, context: &Context) -> RenderLayers;
}
