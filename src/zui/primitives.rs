use winit::dpi::PhysicalPosition;

#[derive(Debug, Copy, Clone)]
pub struct ScreenSpacePosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenSpacePosition {
    pub fn _new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_cursor_physical_position(
        cursor_position: PhysicalPosition<f64>,
        viewport_width_px: u32,
        viewport_height_px: u32,
    ) -> Self {
        Self {
            x: cursor_position.x as f32 / viewport_width_px as f32 * 2f32 - 1f32,
            y: -(cursor_position.y as f32 / viewport_height_px as f32 * 2f32 - 1f32),
        }
    }
}

/// A rectangular region of screen-space using WGPU coordinates, where top left: (-1, 1), and bottom
/// right: (1, -1)
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

    /// Returns true if the cursor is over this rectangle
    pub fn cursor_is_over(&self, cursor: ScreenSpacePosition) -> bool {
        cursor.x >= self.top_left.x()
            && cursor.x <= self.top_left.x() + self.dimensions.x()
            && cursor.y <= self.top_left.y()
            && cursor.y >= self.top_left.y() - self.dimensions.y()
    }
}
