use winit::dpi::PhysicalPosition;

use super::Axis;

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
    /// The minimum bounding x value of the rectangle
    pub x_min: f32,
    /// The maximum bounding x value of the rectangle
    pub x_max: f32,
    /// The minimum bounding y value of the rectangle
    pub y_min: f32,
    /// The maximum bounding y value of the rectangle
    pub y_max: f32,
}

impl Rectangle {
    pub fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Returns true if the point is over this rectangle
    pub fn is_in(&self, position: ScreenSpacePosition) -> bool {
        position.x >= self.x_min
            && position.x <= self.x_max
            && position.y >= self.y_min
            && position.y <= self.y_max
    }

    /// Returns the screen space position vertices that make up the rectangle in the following order:
    ///
    ///   0 -----> 1
    ///          /
    ///        /
    ///      /
    ///    /
    ///   2 -----> 3
    pub fn vertices(&self) -> [glam::Vec2; 4] {
        let top_left = glam::Vec2::new(self.x_min, self.y_max);
        let top_right = glam::Vec2::new(self.x_max, self.y_max);
        let bottom_left = glam::Vec2::new(self.x_min, self.y_min);
        let bottom_right = glam::Vec2::new(self.x_max, self.y_min);

        [
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ]
    }
    
    /// Returns the space width of the rectangle
    pub fn width(&self) -> f32 {
        self.x_max - self.x_min
    }

    /// Returns the space width of the rectangle
    pub fn height(&self) -> f32 {
        self.y_max - self.y_min
    }
    
    /// Gives the span of the Rectangle by its axis
    pub fn span_by_axis(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Vertical => self.height(),
            Axis::Horizontal => self.width(),
        }
    }
}
