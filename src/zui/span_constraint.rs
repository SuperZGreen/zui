use crate::Context;

use super::primitives::Dimensions;

/// A SpanConstraint describes a rule by which to determine the width or height in pixels of a
/// Widget
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum SpanConstraint {
    /// The span will be this portion of the view width, ie. the width of the application window
    /// surface if 1
    ViewWidth(ViewWidth),

    /// The span will be this portion of the view height, ie. the height of the application window
    /// surface if 1
    ViewHeight(ViewHeight),

    /// The span will be this portion of the minimal (height/width) dimension of the wgpu viewport
    ViewMin(ViewMin),

    /// The span will be this length in pixels.
    Pixels(f32),

    /// ALlows the widget to determine its own size. Useful for text and other widgets of variable/
    /// unknown size.
    FitContents,

    /// Shrinks to fit the widget's children.
    FitChildren,

    /// Weighted portion of the remaining space after other children widgets have been laid out.
    /// Will be zero pixels if the parent container is overflowing. The remaining space in the
    /// parent container is divided amongst all children with Span::ParentWeight along the parent's
    /// axis according to their weights.
    ParentWeight(f32),

    /// The resulting span will be the parent's width multiplied by this value.
    ParentWidth(ParentWidth),

    /// The resulting span will be the parent's height multiplied by this value
    ParentHeight(ParentHeight),

    /// The resulting span will be the opposite (horizontal/vertical) span, multiplied by this
    /// value.
    Aspect(f32),
}

/// Allows transformation of SpanConstraints into floating pixel span values
pub trait IntoPixelSpan {
    fn into_pixel_span(&self, context: &Context) -> f32;
}

/// Describes a span in terms of a viewport height coefficient
#[derive(Copy, Clone)]
pub struct ViewHeight {
    pub value: f32,
}

impl ViewHeight {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl IntoPixelSpan for ViewHeight {
    fn into_pixel_span(&self, context: &Context) -> f32 {
        self.value as f32 * context.viewport_dimensions_px.height as f32
    }
}

/// Describes a span in terms of a viewport width coefficient
#[derive(Copy, Clone)]
pub struct ViewWidth {
    pub value: f32,
}

impl ViewWidth {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl IntoPixelSpan for ViewWidth {
    fn into_pixel_span(&self, context: &Context) -> f32 {
        self.value as f32 * context.viewport_dimensions_px.width as f32
    }
}

/// Describes a span in terms of the smaller of the view width or height as a coefficient
#[derive(Copy, Clone)]
pub struct ViewMin {
    pub value: f32,
}

impl ViewMin {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl IntoPixelSpan for ViewMin {
    fn into_pixel_span(&self, context: &Context) -> f32 {
        if context.viewport_dimensions_px.width < context.viewport_dimensions_px.height {
            ViewWidth::new(self.value).into_pixel_span(context)
        } else {
            ViewHeight::new(self.value).into_pixel_span(context)
        }
    }
}

/// Describes a span in terms of the parent's width
#[derive(Copy, Clone)]
pub struct ParentWidth {
    pub value: f32,
}

impl ParentWidth {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn into_pixel_span(&self, parent_dimensions: Dimensions<i32>) -> f32 {
        parent_dimensions.width as f32 * self.value
    }
}

/// Describes a span in terms of the parent's height
#[derive(Copy, Clone)]
pub struct ParentHeight {
    pub value: f32,
}

impl ParentHeight {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn into_pixel_span(&self, parent_dimensions: Dimensions<i32>) -> f32 {
        parent_dimensions.height as f32 * self.value
    }
}
