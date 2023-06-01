use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::{simple_renderer::SimpleVertex, util, Axis, Colour};

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

/// A rectangular region, often representing screen-space using WGPU coordinates, where top left:
/// (-1, 1), and bottom right: (1, -1)
#[derive(Debug, Clone, Copy)]
pub struct Rectangle<T> {
    /// The minimum bounding x value of the rectangle
    pub x_min: T,
    /// The maximum bounding x value of the rectangle
    pub x_max: T,
    /// The minimum bounding y value of the rectangle
    pub y_min: T,
    /// The maximum bounding y value of the rectangle
    pub y_max: T,
}

impl<T> Rectangle<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    pub fn new(x_min: T, x_max: T, y_min: T, y_max: T) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Returns the space width of the rectangle
    pub fn width(&self) -> T {
        self.x_max - self.x_min
    }

    /// Returns the space width of the rectangle
    pub fn height(&self) -> T {
        self.y_max - self.y_min
    }

    /// Gives the span of the Rectangle by its axis
    pub fn span_by_axis(&self, axis: Axis) -> T {
        match axis {
            Axis::Vertical => self.height(),
            Axis::Horizontal => self.width(),
        }
    }
}

impl Rectangle<i32> {
    pub fn is_in(&self, position: glam::IVec2) -> bool {
        position.x >= self.x_min
            && position.x <= self.x_max
            && position.y >= self.y_min
            && position.y <= self.y_max
    }
}

impl Rectangle<f32> {
    /// Returns true if the point is over this rectangle
    pub fn is_in(&self, position: &PhysicalPosition<f64>) -> bool {
        position.x as f32 >= self.x_min
            && position.x as f32 <= self.x_max
            && position.y as f32 >= self.y_min
            && position.y as f32 <= self.y_max
    }

    /// Returns the viewport pixel (y up, (0,0) in bottom left) position vertices that make up the
    /// rectangle in the following order:
    ///
    ///   0 -----> 1
    ///          /
    ///        /
    ///      /
    ///    /
    ///   2 -----> 3
    pub fn vertices(&self, viewport_dimensions_px: PhysicalSize<u32>) -> [glam::Vec2; 4] {
        let top_left = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_min,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_max,
                viewport_dimensions_px.height,
            ),
        );
        let top_right = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_max,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_max,
                viewport_dimensions_px.height,
            ),
        );
        let bottom_left = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_min,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_min,
                viewport_dimensions_px.height,
            ),
        );
        let bottom_right = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_max,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_min,
                viewport_dimensions_px.height,
            ),
        );

        [top_left, top_right, bottom_left, bottom_right]
    }

    /// Converts the rectangle from screen space to framebuffer coordinates
    pub fn screen_space_to_framebuffer(
        &self,
        viewport_dimensions_px: glam::Vec2,
    ) -> Rectangle<f32> {
        Rectangle {
            x_min: util::normalised_device_space_to_frame_buffer_space_x(
                self.x_min,
                viewport_dimensions_px.x,
            ),
            x_max: util::normalised_device_space_to_frame_buffer_space_x(
                self.x_max,
                viewport_dimensions_px.x,
            ),
            // the swapping of y_min and y_max is intended, as it will swap as frambuffers are y-down
            y_min: util::normalised_device_space_to_frame_buffer_space_y(
                self.y_max,
                viewport_dimensions_px.y,
            ),
            y_max: util::normalised_device_space_to_frame_buffer_space_y(
                self.y_min,
                viewport_dimensions_px.y,
            ),
        }
    }
}
