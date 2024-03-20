use winit::dpi::PhysicalSize;

use crate::{Axis, Rectangle};

/// A width and height pair, basically a Rectangle without a position
#[derive(Copy, Clone, Debug)]
pub struct Dimensions<T> {
    pub width: T,
    pub height: T,
}

impl<T> Dimensions<T>
where
    T: Copy,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// Returns the relevant span of the Dimensions
    pub fn span_by_axis(&self, axis: Axis) -> T {
        match axis {
            Axis::Vertical => self.height,
            Axis::Horizontal => self.width,
        }
    }
}

/// Returns true if both of the dimensions are non-zero.
impl Dimensions<u32> {
    pub fn has_non_zero_area(&self) -> bool {
        self.width > 0u32 && self.height > 0u32
    }
}

impl<T> From<&Rectangle<T>> for Dimensions<T>
where
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + Copy,
{
    fn from(value: &Rectangle<T>) -> Self {
        Dimensions {
            width: value.width(),
            height: value.height(),
        }
    }
}

impl<T> From<Rectangle<T>> for Dimensions<T>
where
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + Copy,
{
    fn from(value: Rectangle<T>) -> Self {
        Dimensions {
            width: value.width(),
            height: value.height(),
        }
    }
}

impl From<PhysicalSize<u32>> for Dimensions<u32> {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<&PhysicalSize<u32>> for Dimensions<u32> {
    fn from(value: &PhysicalSize<u32>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
