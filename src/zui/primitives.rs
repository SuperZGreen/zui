use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::{renderer::SimpleVertex, Axis, Colour};

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
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> Self {
        Self {
            x: cursor_position.x as f32 / viewport_dimensions_px.width as f32 * 2f32 - 1f32,
            y: -(cursor_position.y as f32 / viewport_dimensions_px.height as f32 * 2f32 - 1f32),
        }
    }

    // TODO: check equality edge cases
    /// Returns true if in the range -1 to 1 for x and y axis
    pub fn is_in_viewport_bounds(&self) -> bool {
        self.x >= -1f32 && self.x <= 1f32 && self.y >= -1f32 && self.y <= 1f32
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

        [top_left, top_right, bottom_left, bottom_right]
    }

    /// Converts the Rectangle into a Vec of SimpleVertexs, that can be easily rendererd
    pub fn to_simple_vertices(&self, colour: Colour) -> Vec<SimpleVertex> {
        let mut simple_vertices = Vec::with_capacity(6);
        let rectangle_vertices = self.vertices();

        let a = SimpleVertex::new(rectangle_vertices[0], colour.into());
        let b = SimpleVertex::new(rectangle_vertices[1], colour.into());
        let c = SimpleVertex::new(rectangle_vertices[2], colour.into());
        let d = SimpleVertex::new(rectangle_vertices[3], colour.into());

        simple_vertices.push(a);
        simple_vertices.push(c);
        simple_vertices.push(b);

        simple_vertices.push(b);
        simple_vertices.push(c);
        simple_vertices.push(d);
        
        simple_vertices
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
