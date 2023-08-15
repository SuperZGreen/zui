use crate::Context;

use super::primitives::Dimensions;

/// A SpanConstraint describes a rule by which to determine the width or height in pixels of a
/// Widget
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum SpanConstraint {
    //
    //  Fixed Spans: can be calculated given some environmental constants (viewport dimensions),
    //               and the Span value itself
    //
    /// Fixed Span, Size as a portion of the view height, ie. the height of the application window
    /// surface is 1
    ViewWidth(ViewWidth),

    /// Fixed Span: Size as a portion of the view width, ie. the width of the application window
    /// surface is 1
    ViewHeight(ViewHeight),

    /// Fixed Span: Relative size with respect to the minimum dimension of the wgpu viewport
    ViewMin(ViewMin),

    /// Fixed Span: The length in pixels of the Span
    Pixels(f32),

    /// ALlows the widget to determine its own size
    FitContents,

    /// Fits the children of the widget
    FitChildren,

    // TODO: ViewMax?

    //
    //  Parent-based Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted measure of the remaining space after other children widgets have been laid out.
    /// Will be zero pixels if the parent container is overflowing. The remaining space in the
    /// parent container is divided amongst all children with Span::ParentWeight along the parent's
    /// axis according to their weights.
    ParentWeight(f32),

    /// Gives as a portion of the parent widget's width
    ParentWidth(ParentWidth),

    /// Gives as a portion of the parent widget's width
    ParentHeight(ParentHeight),

    /// Looks at the opposite (vorizontal/vertical) span, and multiplies this value by the aspect
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
    fn into_pixel_span(&self, _context: &Context) -> f32 {
        // TODO
        todo!()
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
