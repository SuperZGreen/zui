use super::{
    renderer::SimpleVertex, text_renderer::TextVertex, Rectangle,
    ScreenSpacePosition,
};

use crate::zui::Context;

#[derive(Copy, Clone)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
    pub fn _to_index(&self) -> usize {
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
    //  Flexible Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted size with respect to the parent's size. Is summed up and divided amongst other
    /// child's sizes to determine the actual screen-space size of the widget
    ParentWeight(f32),

    /// The size as a proportion of the parent's size
    ParentRatio(f32),
}

impl Span {
    pub fn view_min_to_screen_space_span(
        view_min: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
    ) -> f32 {
        if aspect_ratio < 1f32 {
            Self::view_width_to_screen_space_span(view_min, aspect_ratio, parent_widget_axis)
        } else {
            Self::view_height_to_screen_space_span(view_min, aspect_ratio, parent_widget_axis)
        }
    }

    pub fn view_height_to_screen_space_span(
        view_height: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
    ) -> f32 {
        match parent_widget_axis {
            Axis::Vertical => view_height * 2f32,
            Axis::Horizontal => view_height / aspect_ratio * 2f32,
        }
    }

    pub fn view_width_to_screen_space_span(
        view_width: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
    ) -> f32 {
        match parent_widget_axis {
            Axis::Vertical => view_width * aspect_ratio * 2f32,
            Axis::Horizontal => view_width * 2f32,
        }
    }
}

#[derive(Debug)]
pub enum MouseEvent {
    ButtonPressed,
    ButtonReleased,
    CursorEnteredWindow,
    CursorExitedWindow,
    CursorMoved(ScreenSpacePosition),
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
pub enum Event<'a> {
    /// An event that involves mouse interaction
    MouseEvent(MouseEvent),

    /// The widget is commanded to fit the provided rectangle
    // TODO: remove Context from this
    FitRectangle((Rectangle, &'a Context<'a>)),
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
    fn clip_rectangle(&self) -> Option<Rectangle> {
        None
    }

    /// The [`Span`] of the widget
    fn span(&self) -> Span {
        Span::ParentWeight(1f32)
    }

    /// The [`Axis`] of the widget
    fn axis(&self) -> Axis {
        Axis::Vertical
    }

    /// Returns the vertices necessary to render a widget
    fn to_vertices(
        &self,
        viewport_dimensions_px: glam::Vec2,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        (Vec::new(), Vec::new())
    }
}
