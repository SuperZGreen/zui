use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::{
    render_layer::RenderLayer, simple_renderer::SimpleVertex, text_renderer::TextVertex, Rectangle,
    ScreenSpacePosition,
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
    //  Absolute Sizes
    //
    /// Size as a portion of the view height, ie. the height of the application window surface is 1
    ViewWidth(f32),

    /// Size as a portion of the view width, ie. the width of the application window surface is 1
    ViewHeight(f32),

    /// Relative size with respect to the minimum dimension of the wgpu viewport
    ViewMin(f32),

    // TODO: ViewMax?

    //
    //  Parent-based Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted size with respect to the parent's size. Is summed up and divided amongst other
    /// child's sizes to determine the actual screen-space size of the widget
    ParentWeight(f32),

    /// The size as a proportion of the parent's size
    ParentRatio(f32),

    //
    //  Contents based sizes, dynamically resizes based on the size of the contents. This is text in
    //  the case of a Container.
    //
    FitContents,
}

impl Span {
    pub fn view_min_to_viewport_px(
        view_min: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> f32 {
        if aspect_ratio < 1f32 {
            Self::view_width_to_viewport_px(
                view_min,
                aspect_ratio,
                parent_widget_axis,
                viewport_dimensions_px,
            )
        } else {
            Self::view_height_to_viewport_px(
                view_min,
                aspect_ratio,
                parent_widget_axis,
                viewport_dimensions_px,
            )
        }
    }

    pub fn view_height_to_viewport_px(
        view_height: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> f32 {
        match parent_widget_axis {
            Axis::Vertical => view_height * viewport_dimensions_px.height as f32,
            Axis::Horizontal => view_height / aspect_ratio * viewport_dimensions_px.width as f32,
        }
    }

    pub fn view_width_to_viewport_px(
        view_width: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> f32 {
        match parent_widget_axis {
            Axis::Vertical => view_width * viewport_dimensions_px.height as f32 * aspect_ratio,
            Axis::Horizontal => view_width * viewport_dimensions_px.width as f32,
        }
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
            Span::ViewWidth(vw) => Self::view_width_to_viewport_px(
                *vw,
                context.aspect_ratio,
                parent_axis,
                context.viewport_dimensions_px,
            ),
            Span::ViewHeight(vh) => Self::view_height_to_viewport_px(
                *vh,
                context.aspect_ratio,
                parent_axis,
                context.viewport_dimensions_px,
            ),
            Span::ViewMin(vm) => Self::view_min_to_viewport_px(
                *vm,
                context.aspect_ratio,
                parent_axis,
                context.viewport_dimensions_px,
            ),
            Span::ParentWeight(pw) => {
                if sum_of_parent_weights.is_none() || parent_span_px_available.is_none() {
                    0f32
                } else {
                    *pw / sum_of_parent_weights.unwrap() * parent_span_px_available.unwrap() as f32
                }
            }
            Span::ParentRatio(ratio) => {
                *ratio * parent_rectangle.span_by_axis(parent_axis) as f32
            }
            Span::FitContents => fit_contents_span_px,
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

    /// A mutable iterator for all direct children, used for propagating events
    fn children_iter_mut(
        &mut self,
    ) -> Option<std::slice::IterMut<'_, Box<(dyn Widget<Message> + 'static)>>> {
        None
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

    /// The [`Axis`] of the widget
    fn axis(&self) -> Axis {
        Axis::Vertical
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
