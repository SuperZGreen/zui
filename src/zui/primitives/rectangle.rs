use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{util, Axis, Dimensions};

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
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + Copy,
{
    pub fn new(x_min: T, x_max: T, y_min: T, y_max: T) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Returns the width of the rectangle
    pub fn width(&self) -> T {
        self.x_max - self.x_min
    }

    /// Returns the height of the rectangle
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

    fn max(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    fn min(a: T, b: T) -> T {
        if a < b {
            a
        } else {
            b
        }
    }

    /// Returns the intersection of two rectangles
    pub fn intersection(&self, other: &Rectangle<T>) -> Option<Rectangle<T>> {
        let min_max_x = Self::min(self.x_max, other.x_max);
        let max_min_x = Self::max(self.x_min, other.x_min);

        if max_min_x > min_max_x {
            return None;
        }

        let min_max_y = Self::min(self.y_max, other.y_max);
        let max_min_y = Self::max(self.y_min, other.y_min);

        if max_min_y > min_max_y {
            return None;
        }

        Some(Rectangle {
            x_min: max_min_x,
            x_max: min_max_x,
            y_min: max_min_y,
            y_max: min_max_y,
        })
    }
}

impl<T> PartialEq for Rectangle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x_min == other.x_min
            && self.x_max == other.x_max
            && self.y_min == other.y_min
            && self.y_max == other.y_max
    }
}

impl Rectangle<i32> {
    /// Returns the viewport pixel (y up, (0,0) in bottom left) position vertices that make up the
    /// rectangle in the following order:
    ///
    ///   0 -----> 1
    ///          /
    ///        /
    ///      /
    ///    /
    ///   2 -----> 3
    pub fn vertices(&self, viewport_dimensions_px: Dimensions<u32>) -> [glam::Vec2; 4] {
        let top_left = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_min as f32,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_max as f32,
                viewport_dimensions_px.height,
            ),
        );
        let top_right = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_max as f32,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_max as f32,
                viewport_dimensions_px.height,
            ),
        );
        let bottom_left = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_min as f32,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_min as f32,
                viewport_dimensions_px.height,
            ),
        );
        let bottom_right = glam::Vec2::new(
            util::viewport_px_to_normalised_device_coordinates(
                self.x_max as f32,
                viewport_dimensions_px.width,
            ),
            util::viewport_px_to_normalised_device_coordinates(
                self.y_min as f32,
                viewport_dimensions_px.height,
            ),
        );

        [top_left, top_right, bottom_left, bottom_right]
    }

    /// Returns true if the PhysicalPosition is within the bounds of the Rectangle
    /// TODO: this will not work correctly for negative values of position (?) Might not be strictly
    /// necessary either, but something to look into for the future
    pub fn contains_position(&self, position: &PhysicalPosition<f64>) -> bool {
        (position.x as i32) >= self.x_min
            && (position.x as i32) < self.x_max
            && (position.y as i32) >= self.y_min
            && (position.y as i32) < self.y_max
    }

    pub fn has_non_zero_area(&self) -> bool {
        self.width() > 0i32 && self.height() > 0i32
    }

    /// Translates the region by a glam::IVec2
    pub fn translate(self, translation: glam::IVec2) -> Self {
        Self {
            x_min: self.x_min + translation.x,
            x_max: self.x_max + translation.x,
            y_min: self.y_min + translation.y,
            y_max: self.y_max + translation.y,
        }
    }
}

impl Rectangle<f32> {
    /// Returns true if the point is over this rectangle
    pub fn is_in(&self, position: &PhysicalPosition<f64>) -> bool {
        (position.x as f32) >= self.x_min
            && (position.x as f32) < self.x_max
            && (position.y as f32) >= self.y_min
            && (position.y as f32) < self.y_max
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

impl From<&Rectangle<f32>> for PhysicalSize<f32> {
    fn from(value: &Rectangle<f32>) -> Self {
        PhysicalSize {
            width: value.width(),
            height: value.height(),
        }
    }
}
