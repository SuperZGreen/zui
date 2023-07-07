use super::{render_layer::RenderLayer, Context};
use std::collections::VecDeque;

pub trait Renderable
{
    fn to_render_layers(
        &self,
        context: &Context,
    ) -> VecDeque<RenderLayer>;
}
