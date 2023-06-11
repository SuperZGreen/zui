use rustc_hash::FxHashSet;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::{
    font::SymbolKey, render_layer::RenderLayer, simple_renderer::SimpleVertex,
    text_renderer::TextVertex, Rectangle, primitives::Dimensions,
};
use std::collections::VecDeque;

use crate::zui::Context;

#[derive(Copy, Clone)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
    pub fn to_index(&self) -> usize {
        match self {
            Axis::Horizontal => 0usize,
            Axis::Vertical => 1usize,
        }
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Span {
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

    //
    //  Contents based sizes, dynamically resizes based on the size of the contents. This is text in
    //  the case of a Container.
    //
    FitContents,

    // TODO: ViewMax?

    //
    //  Parent-based Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted size with respect to the parent's size. Is summed up and divided amongst other
    /// child's sizes to determine the actual screen-space size of the widget
    ParentWeight(f32),

    /// The size as a proportion of the parent's size
    ParentRatio(f32),
}

impl Span {
    pub fn view_min_to_span_px(view_min: f32, viewport_dimensions_px: PhysicalSize<u32>) -> f32 {
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
            Span::ViewWidth(vw) => {
                Self::view_width_to_span_px(*vw, context.viewport_dimensions_px.width)
            }
            Span::ViewHeight(vh) => {
                Self::view_height_to_span_px(*vh, context.viewport_dimensions_px.height)
            }
            Span::ViewMin(vm) => Self::view_min_to_span_px(*vm, context.viewport_dimensions_px),
            Span::Pixels(px) => *px,
            Span::ParentWeight(pw) => {
                if sum_of_parent_weights.is_none() || parent_span_px_available.is_none() {
                    0f32
                } else {
                    *pw / sum_of_parent_weights.unwrap() * parent_span_px_available.unwrap() as f32
                }
            }
            Span::ParentRatio(ratio) => *ratio * parent_rectangle.span_by_axis(parent_axis) as f32,
            Span::FitContents => fit_contents_span_px,
        }
    }

    /// Converts fixed type Spans to f32 viewport pixels
    pub fn fixed_span_to_span_px(&self, viewport_dimensions_px: PhysicalSize<u32>) -> Option<f32> {
        match self {
            Span::ViewWidth(vw) => Some(Self::view_width_to_span_px(
                *vw,
                viewport_dimensions_px.width,
            )),
            Span::ViewHeight(vh) => Some(Self::view_height_to_span_px(
                *vh,
                viewport_dimensions_px.height,
            )),
            // This is also potentially a 'fixed' type, but can not be calculated here, so returns
            // None instead
            Span::FitContents => None,

            Span::ViewMin(vm) => Some(Self::view_min_to_span_px(*vm, viewport_dimensions_px)),
            Span::Pixels(p) => Some(*p),
            Span::ParentWeight(_) => None,
            Span::ParentRatio(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum MouseEvent {
    ButtonPressed,
    ButtonReleased,
    CursorEnteredWindow,
    CursorExitedWindow,
    CursorMoved(PhysicalPosition<f64>),
}

#[derive(Debug)]
pub enum WindowEvent {
    // Resized {
    //     /// Width in pixels
    //     width: u32,
    //     /// Height in pixels
    //     height: u32,
    // },
}

#[derive(Debug)]
pub enum Event {
    /// An event that involves mouse interaction
    MouseEvent(MouseEvent),

    /// The widget is commanded to fit the provided rectangle
    FitRectangle(Rectangle<f32>),
}

pub enum EventResponse<Message> {
    /// The [`Event`] is consumed by the widget and will not be propagated to its children
    Consumed,
    /// The [`Event`] is consumed by the widget and a message is produced
    Message(Message),
    /// The [`Event`] is not consumed by the widget and will be propagated to its children
    Propagate,
}

#[allow(unused_variables)]
pub trait Widget<Message> {
    /// Handles interaction events, returning the EventResponse that determines whether events
    /// should be propagated to children
    fn handle_event(&mut self, event: &Event, context: &Context) -> EventResponse<Message> {
        EventResponse::Propagate
    }

    /// Gives the children of the [`Widget`] as a slice
    fn children(&self) -> &[Box<(dyn Widget<Message>)>] {
        &[]
    }

    /// Gives the children of the [`Widget`] as a mutable slice
    fn children_mut(&mut self) -> &mut [Box<(dyn Widget<Message>)>] {
        &mut []
    }

    /// Returns the clipping rectangle of the widget
    fn clip_rectangle(&self) -> Option<Rectangle<f32>> {
        None
    }

    /// The [`Span`] of the widget in viewport pixels, returns None if is a weighted value and must be
    /// set by the parent
    fn span_px(&self) -> Option<f32> {
        None
    }

    /// Returns the [`Span`] of a widget
    fn span(&self) -> Span;

    /// Updates the screen space span of the [`Widget`], giving it everything it might need to
    /// calculate the span
    fn update_viewport_span_px(
        &mut self,
        clip_rectangle: &Rectangle<f32>,
        parent_axis: Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        viewport_span_px_available: Option<u32>,
        context: &Context,
    );

    /// Updates span_px to its minimum possible value, this is the sum of its children's 'fixed'
    /// type Spans, if the Widget is of type FitContents, it'll iterate through that Widget's
    /// children to determine what the minimum size of the container will be.
    /// Is a subset of span(). Basically returns the span for all except for Span::ParentWeight
    fn min_span_px(
        &mut self,
        axis: Axis,
        region_dimensions_px: Dimensions<f32>,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> Option<f32> {
        let parent_span_px = region_dimensions_px.span_by_axis(axis);

        match self.span() {
            Span::ViewWidth(vw) => Some(Span::view_width_to_span_px(
                vw,
                viewport_dimensions_px.width,
            )),
            Span::ViewHeight(vh) => Some(Span::view_height_to_span_px(
                vh,
                viewport_dimensions_px.height,
            )),
            Span::ViewMin(vm) => Some(Span::view_min_to_span_px(vm, viewport_dimensions_px)),
            Span::Pixels(p) => Some(p),

            // getting the size of all the children's min spans
            Span::FitContents => {
                let mut cumulative_min_span_px = 0f32;
                for child in self.children_mut().iter_mut() {
                    if let Some(span_px) =
                        child.min_span_px(axis, region_dimensions_px, viewport_dimensions_px)
                    {
                        cumulative_min_span_px += span_px
                    }
                }

                if cumulative_min_span_px != 0f32 {
                    Some(cumulative_min_span_px)
                } else {
                    None
                }
            }
            Span::ParentRatio(pr) => Some(pr * parent_span_px),
            Span::ParentWeight(_) => None,
        }
    }

    /// The [`Axis`] of the widget
    fn axis(&self) -> Axis {
        Axis::Vertical
    }

    /// Widgets insert all characters (SymbolKeys) that they will need to render their Text, and
    /// propogates the call to their children. This is to ensure that the font system has the
    /// required characters rasterised and ready to render when text Presymbols are generated
    fn collect_text(&self, symbol_keys: &mut FxHashSet<SymbolKey>) {
        // Note: not collecting own text by default!

        for child in self.children().iter() {
            child.collect_text(symbol_keys);
        }
    }

    /// Returns the vertices necessary to render a widget, will append a new RenderLayer to
    /// render_layers to allow for a new clipping region when overflowing. This is not normally
    /// required by widgets that do not have nested children/do not have grand-children that may
    /// overflow
    fn to_vertices(
        &self,
        viewport_dimensions_px: PhysicalSize<u32>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        (Vec::new(), Vec::new())
    }
}
