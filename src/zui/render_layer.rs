use super::{primitives::Rectangle, simple_renderer::SimpleVertex, text_renderer::TextVertex};

pub struct RenderLayer {
    /// The SimpleVertices that will be rendered on this layer
    pub simple_vertices: Vec<SimpleVertex>,

    /// The TextVertices that will be rendered on this layer
    pub text_vertices: Vec<TextVertex>,

    /// The clipping rectangle, contents outside of this will not be rendered
    pub clip_rectangle: Option<Rectangle<i32>>,
}

impl RenderLayer {
    pub fn new(
        simple_vertices: Vec<SimpleVertex>,
        text_vertices: Vec<TextVertex>,
        clip_rectangle: Option<Rectangle<i32>>,
    ) -> Self {
        Self {
            simple_vertices,
            text_vertices,
            clip_rectangle,
        }
    }
}
