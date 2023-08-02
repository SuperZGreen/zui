use crate::Axis;

/// A PositionConstraint describes a rule by which to determine the position of the Widget
#[derive(Copy, Clone)]
pub enum PositionConstraint {
    /// Indicates that the position of the widget is controlled by the parent widget.
    ParentDetermined(PaddingWeights),

    /// Indicates that the position of the widget is floating
    Floating(i32, i32),
}

/// Determines the position of the widget in the parent. Is equivalent to a container with
/// Span::ParentWeight(x) in each of the corresponding positions
#[derive(Copy, Clone)]
pub struct PaddingWeights {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl PaddingWeights {
    /// Gives no padding weights
    pub const NONE: Self = Self {
        top: 0f32,
        bottom: 0f32,
        left: 0f32,
        right: 0f32,
    };

    /// Gives the padding with all positions
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    /// Gives the padding with shorthand for vertical/horizontal
    pub fn vh(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    /// Sums the two weights along the given axis
    pub fn sum_by_axis(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Vertical => self.top + self.bottom,
            Axis::Horizontal => self.left + self.right,
        }
    }
}
