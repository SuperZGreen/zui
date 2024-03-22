use super::{render_layer::RenderLayer, Context};

pub trait Renderable
{
    fn to_render_layers(
        &self,
        context: &Context,
    ) -> Vec<RenderLayer>;
}
