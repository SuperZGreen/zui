#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    /// top left point of rectangle in screen space
    pub top_left: glam::Vec2,

    /// dimensions (width, height) in screen space
    pub dimensions: glam::Vec2,
}

impl Rectangle {
    pub fn new(top_left: glam::Vec2, dimensions: glam::Vec2) -> Self {
        Self {
            top_left,
            dimensions,
        }
    }
}
