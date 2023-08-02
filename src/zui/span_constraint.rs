use crate::{Rectangle, Axis, Context};

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
    ViewWidth(f32),

    /// Fixed Span: Size as a portion of the view width, ie. the width of the application window
    /// surface is 1
    ViewHeight(f32),

    /// Fixed Span: Relative size with respect to the minimum dimension of the wgpu viewport
    ViewMin(f32),

    /// Fixed Span: The length in pixels of the Span
    Pixels(f32),

    /// ALlows the widget to determine its own size
    FitContents,

    // TODO: ViewMax?

    //
    //  Parent-based Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted measure of the remaining space after other children widgets have been laid out.
    /// Will be zero pixels if the parent container is overflowing. The remaining space in the
    /// parent container is divided amongst all children with Span::ParentWeight along the parent's
    /// axis according to their weights.
    ParentWeight(f32),

    /// The size as a proportion of the parent's size
    ParentRatio(f32),
}

impl SpanConstraint {
    pub fn view_min_to_span_px(view_min: f32, viewport_dimensions_px: Dimensions<u32>) -> f32 {
        if viewport_dimensions_px.width < viewport_dimensions_px.height {
            Self::view_width_to_span_px(view_min, viewport_dimensions_px.width)
        } else {
            Self::view_height_to_span_px(view_min, viewport_dimensions_px.height)
        }
    }

    pub fn view_height_to_span_px(view_height: f32, viewport_height_px: u32) -> f32 {
        view_height as f32 * viewport_height_px as f32
    }

    pub fn view_width_to_span_px(
        view_width: f32,
        // The width of the viewport in pixels
        viewport_width_px: u32,
    ) -> f32 {
        view_width as f32 * viewport_width_px as f32
    }

    /// Converts the span into a pixel span value given context and parent widget region and axis
    pub fn to_viewport_px(
        &self,
        parent_rectangle: &Rectangle<f32>,
        parent_axis: Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        parent_span_px_available: Option<u32>,
        context: &Context,
        fit_contents_span_px: f32,
    ) -> f32 {
        match self {
            SpanConstraint::ViewWidth(vw) => {
                Self::view_width_to_span_px(*vw, context.viewport_dimensions_px.width)
            }
            SpanConstraint::ViewHeight(vh) => {
                Self::view_height_to_span_px(*vh, context.viewport_dimensions_px.height)
            }
            SpanConstraint::ViewMin(vm) => Self::view_min_to_span_px(*vm, context.viewport_dimensions_px),
            SpanConstraint::Pixels(px) => *px,
            SpanConstraint::ParentWeight(pw) => {
                if sum_of_parent_weights.is_none() || parent_span_px_available.is_none() {
                    0f32
                } else {
                    *pw / sum_of_parent_weights.unwrap() * parent_span_px_available.unwrap() as f32
                }
            }
            SpanConstraint::ParentRatio(ratio) => *ratio * parent_rectangle.span_by_axis(parent_axis) as f32,
            SpanConstraint::FitContents => fit_contents_span_px,
        }
    }

    /// Converts fixed type Spans to f32 viewport pixels
    pub fn min_span_px(
        &self,
        axis: Axis,
        region_dimensions_px: Dimensions<f32>,
        viewport_dimensions_px: Dimensions<u32>,
    ) -> Option<f32> {
        match self {
            SpanConstraint::ViewWidth(vw) => Some(Self::view_width_to_span_px(
                *vw,
                viewport_dimensions_px.width,
            )),
            SpanConstraint::ViewHeight(vh) => Some(Self::view_height_to_span_px(
                *vh,
                viewport_dimensions_px.height,
            )),
            SpanConstraint::ViewMin(vm) => Some(Self::view_min_to_span_px(*vm, viewport_dimensions_px)),
            SpanConstraint::Pixels(p) => Some(*p),
            SpanConstraint::ParentRatio(pr) => {
                let parent_span_px = region_dimensions_px.span_by_axis(axis);
                Some(pr * parent_span_px)
            }

            // This is also potentially a 'fixed' type, but can not be calculated here, so returns
            // None instead
            SpanConstraint::FitContents => None,
            SpanConstraint::ParentWeight(_) => None,
        }
    }
}

