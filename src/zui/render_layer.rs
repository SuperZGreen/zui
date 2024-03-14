use super::{
    primitives::Rectangle,
    renderers::{SimpleVertex, TextVertex},
};

/// A layer the contains vertices of different types, that will be rendered on top of other layers.
/// The layer also may contain a clipping rectangle, which will prevent contents outside of this
/// rectangle from being rendered
pub struct RenderLayer {
    /// The name of the RenderLayer, for debugging
    pub name: Option<String>,

    /// The SimpleVertices that will be rendered on this layer
    pub simple_vertices: Vec<SimpleVertex>,

    /// The TextVertices that will be rendered on this layer
    pub text_vertices: Vec<TextVertex>,

    /// The clipping rectangle in viewport pixels, contents outside of this will not be rendered
    pub clip_rectangle: Option<Rectangle<f32>>,
}

impl RenderLayer {
    pub fn new(
        simple_vertices: Vec<SimpleVertex>,
        text_vertices: Vec<TextVertex>,
        clip_rectangle: Option<Rectangle<f32>>,
    ) -> Self {
        Self {
            name: None,
            simple_vertices,
            text_vertices,
            clip_rectangle,
        }
    }

    pub fn with_name(mut self, name: Option<&str>) -> Self {
        self.name = name.map(|n| String::from(n));
        self
    }
}
