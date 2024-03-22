use super::{
    primitives::Rectangle,
    renderers::{SimpleVertex, TextVertex},
};

/// A layer the contains vertices of different types, that will be rendered on top of other layers.
/// The layer also may contain a clipping rectangle, which will prevent contents outside of this
/// rectangle from being rendered
pub struct RenderLayer {
    /// The SimpleVertices that will be rendered on this layer
    pub simple_vertices: Vec<SimpleVertex>,

    /// The TextVertices that will be rendered on this layer
    pub text_vertices: Vec<TextVertex>,

    /// The clipping rectangle in viewport pixels, contents outside of this will not be rendered
    pub clip_rectangle: Option<Rectangle<i32>>,
}

impl RenderLayer {
    pub fn new(
        clip_rectangle: Option<Rectangle<i32>>,
    ) -> Self {
        Self {
            simple_vertices: Vec::new(),
            text_vertices: Vec::new(),
            clip_rectangle,
        }
    }
}
