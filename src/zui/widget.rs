use super::{
    renderer::SimpleVertex, text::Text, text_renderer::TextVertex, Colour, Font,
    Rectangle, ScreenSpacePosition,
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

pub struct BaseWidget<Message>
where
    Message: Clone + Copy,
{
    children: Vec<Box<dyn Widget<Message>>>,

    // structure
    pub axis: Axis,
    pub span: Span,

    // internal callbacks
    pub callback_cursor_clicked: Option<fn(&mut Self) -> bool>,
    pub callback_cursor_on: Option<fn(&mut Self) -> bool>,
    pub callback_cursor_off: Option<fn(&mut Self) -> bool>,

    // message behaviour
    pub message_cursor_clicked: Option<Message>,
    pub message_cursor_on: Option<Message>,
    pub message_cursor_off: Option<Message>,

    // style
    pub background: Option<Colour>,

    // calculated screen space area
    pub rectangle: Option<Rectangle>,

    // Text contained within a widget's area
    pub text: Option<Text>,
}

impl<Message> BaseWidget<Message>
where
    Message: Clone + Copy,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            axis: Axis::Vertical,
            span: Span::ParentWeight(1f32),
            callback_cursor_clicked: None,
            callback_cursor_on: None,
            callback_cursor_off: None,
            message_cursor_clicked: None,
            message_cursor_on: None,
            message_cursor_off: None,
            background: None,
            rectangle: None,
            text: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    #[allow(dead_code)]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    #[allow(dead_code)]
    pub fn with_background(mut self, background: Option<Colour>) -> Self {
        self.background = background;
        self
    }

    #[allow(dead_code)]
    pub fn with_callback_cursor_clicked(mut self, callback: Option<fn(&mut Self) -> bool>) -> Self {
        self.callback_cursor_clicked = callback;
        self
    }

    #[allow(dead_code)]
    pub fn with_callback_cursor_on(mut self, callback: Option<fn(&mut Self) -> bool>) -> Self {
        self.callback_cursor_on = callback;
        self
    }

    #[allow(dead_code)]
    pub fn with_callback_cursor_off(mut self, callback: Option<fn(&mut Self) -> bool>) -> Self {
        self.callback_cursor_off = callback;
        self
    }

    #[allow(dead_code)]
    pub fn with_message_clicked(mut self, message: Option<Message>) -> Self {
        self.message_cursor_clicked = message;
        self
    }

    #[allow(dead_code)]
    pub fn with_message_cursor_on(mut self, message: Option<Message>) -> Self {
        self.message_cursor_on = message;
        self
    }

    #[allow(dead_code)]
    pub fn with_message_cursor_off(mut self, message: Option<Message>) -> Self {
        self.message_cursor_off = message;
        self
    }

    #[allow(dead_code)]
    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    #[allow(dead_code)]
    pub fn push(mut self, child: impl Into<Box<dyn Widget<Message>>>) -> Self {
        self.children.push(child.into());
        self
    }

    // /// Creates a new widget with padding widgets
    // #[allow(dead_code)]
    // pub fn push_padded(self, child: Self, padding_widget: Self) -> Self {
    //     let vertical_container = Self::new()
    //         .with_axis(Axis::Vertical)
    //         .push(padding_widget.clone())
    //         .push(child)
    //         .push(padding_widget.clone());
    //     let horizontal_container = Self::new()
    //         .with_axis(Axis::Horizontal)
    //         .push(padding_widget.clone())
    //         .push(vertical_container)
    //         .push(padding_widget.clone());

    //     self.push(horizontal_container)
    // }

    pub fn update_child_rectangles(&mut self, context: &Context) {
        let self_rectangle = self.rectangle.unwrap();

        // getting the amount of free space within the self span for child widgets
        let self_normalised_space_available = Self::get_parent_normalised_space_available(
            &self.children,
            self.axis,
            self_rectangle.span_by_axis(self.axis),
            context.aspect_ratio,
        );

        if self_normalised_space_available < 0.0f32 {
            warn!(
                "overflowing: screen space span: {}",
                self_normalised_space_available
            );
        }

        let sum_of_child_span_weights = Self::get_sum_of_child_span_weights(&self.children);
        // let mut top_left_accumulator = glam::Vec2::new(self_rectangle.x_min, self_rectangle.y_max);
        let mut used_screen_space_accumulator = 0f32;

        for child in self.children.iter_mut() {
            let child_screen_space_span = match child.span() {
                Span::ViewWidth(vw) => Span::view_width_to_screen_space_span(
                    vw,
                    context.aspect_ratio,
                    self.axis,
                ),
                Span::ViewHeight(vh) => Span::view_height_to_screen_space_span(
                    vh,
                    context.aspect_ratio,
                    self.axis,
                ),
                Span::ViewMin(vm) => Span::view_min_to_screen_space_span(
                    vm,
                    context.aspect_ratio,
                    self.axis,
                ),
                Span::ParentWeight(pw) => {
                    pw / sum_of_child_span_weights
                        * self_normalised_space_available
                        * self_rectangle.span_by_axis(self.axis)
                }
                Span::ParentRatio(pr) => pr * self_rectangle.span_by_axis(self.axis),
            };

            let child_rectangle = match self.axis {
                Axis::Vertical => Rectangle::new(
                    self_rectangle.x_min,
                    self_rectangle.x_max,
                    self_rectangle.y_max - used_screen_space_accumulator - child_screen_space_span,
                    self_rectangle.y_max - used_screen_space_accumulator,
                ),
                Axis::Horizontal => Rectangle::new(
                    self_rectangle.x_min + used_screen_space_accumulator,
                    self_rectangle.x_min + used_screen_space_accumulator + child_screen_space_span,
                    self_rectangle.y_min,
                    self_rectangle.y_max,
                ),
            };

            // updating accumulator
            used_screen_space_accumulator += child_screen_space_span;

            child.handle_event(&Event::FitRectangle((
                child_rectangle,
                context,
            )), context);
        }
    }

    // /// Updates the text symbols for self and child widgets if applicable
    // pub fn update_text_symbols_recursively(&mut self, font: &Font, aspect_ratio: f32) {
    //     // updating own text
    //     if let Some(text) = &mut self.text {
    //         if let Some(self_rectangle) = &self.rectangle {
    //             text.update_symbols(font, self_rectangle, aspect_ratio);
    //         }
    //     }

    //     // calling for children
    //     for child in &mut self.children {
    //         child.update_text_symbols_recursively(font, aspect_ratio);
    //     }
    // }

    // TODO: will need to change
    /// Gets the amount of space (normalised to the parent's directional span) available
    fn get_parent_normalised_space_available(
        children: &[Box<dyn Widget<Message>>],
        parent_widget_axis: Axis,
        parent_screen_space_span: f32,
        aspect_ratio: f32,
    ) -> f32 {
        let mut normalised_space_available = 1f32;
        for child in children.iter() {
            let child_space_used = match child.span() {
                Span::ViewHeight(vh) => {
                    Span::view_height_to_screen_space_span(vh, aspect_ratio, parent_widget_axis)
                        / parent_screen_space_span
                }
                Span::ViewWidth(vw) => {
                    Span::view_width_to_screen_space_span(vw, aspect_ratio, parent_widget_axis)
                        / parent_screen_space_span
                }
                Span::ViewMin(vm) => {
                    Span::view_min_to_screen_space_span(vm, aspect_ratio, parent_widget_axis)
                        / parent_screen_space_span
                }
                Span::ParentRatio(ratio) => ratio,
                Span::ParentWeight(_) => 0f32,
            };

            normalised_space_available -= child_space_used;
        }
        normalised_space_available
    }

    fn get_sum_of_child_span_weights(children: &[Box<dyn Widget<Message>>]) -> f32 {
        let mut sum = 0f32;
        for child in children.iter() {
            match child.span() {
                Span::ViewHeight(_) => {}
                Span::ViewWidth(_) => {}
                Span::ViewMin(_) => {}
                Span::ParentRatio(_) => {}
                Span::ParentWeight(weight) => sum += weight,
            }
        }
        sum
    }

    // pub fn traverse<F>(&self, f: &mut F)
    // where
    //     F: FnMut(&BaseWidget<Message>),
    // {
    //     f(self);

    //     for child in self.children.iter() {
    //         child.traverse(f);
    //     }
    // }

    // /// Traverses through widgets, adding their on_x messages to the message queue if satisfied
    // pub fn update_cursor_events_recursively(
    //     &mut self,
    //     cursor_state: &Option<CursorState>,
    //     message_queue: &mut VecDeque<Message>,
    // ) {
    //     let self_rectangle = match self.rectangle {
    //         Some(r) => r,
    //         None => return,
    //     };

    //     let mut cursor_is_over_widget = false;
    //     if let Some(cursor_state) = cursor_state {
    //         if self_rectangle.is_in(cursor_state.screen_space_position()) {
    //             cursor_is_over_widget = true;

    //             if let Some(callback_cursor_on) = self.callback_cursor_on {
    //                 callback_cursor_on(self);
    //             }

    //             if let Some(message) = self.message_cursor_on {
    //                 message_queue.push_back(message);
    //             }

    //             if cursor_state.is_clicked() {
    //                 if let Some(callback_cursor_clicked) = self.callback_cursor_clicked {
    //                     callback_cursor_clicked(self);
    //                 }

    //                 if let Some(message) = self.message_cursor_clicked {
    //                     message_queue.push_back(message);
    //                 }
    //             }
    //         } else {
    //             cursor_is_over_widget = false;
    //         }
    //     }

    //     if !cursor_is_over_widget {
    //         if let Some(callback_cursor_off) = self.callback_cursor_off {
    //             callback_cursor_off(self);
    //         }

    //         if let Some(message) = self.message_cursor_off {
    //             message_queue.push_back(message);
    //         }
    //     }

        // // TODO: always propagates to children (whether over the widget or not), add conditional
        // // propagation to children
        // for child in self.children.iter_mut() {
        //     child.update_cursor_events_recursively(cursor_state, message_queue)
        // }
    // }
}

impl<Message> Widget<Message> for BaseWidget<Message>
where
    Message: Clone + Copy,
{
    fn handle_event(&mut self, event: &Event, _context: &Context) -> EventResponse<Message> {
        match event {
            Event::MouseEvent(_) => EventResponse::Propagate,
            Event::FitRectangle((rectangle, context)) => {
                self.rectangle = Some(*rectangle);
                if let Some(text) = &mut self.text {
                    text.update_symbols(
                        context.font,
                        &rectangle,
                        context.aspect_ratio,
                    );
                }
                self.update_child_rectangles(context);

                EventResponse::Consumed
            }
        }
    }

    fn clip_rectangle(&self) -> Option<Rectangle> {
        self.rectangle
    }

    fn children_iter_mut(
        &mut self,
    ) -> Option<std::slice::IterMut<'_, Box<(dyn Widget<Message> + 'static)>>> {
        Some(self.children.iter_mut())
    }

    fn to_vertices(
        &self,
        viewport_dimensions_px: glam::Vec2,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        // adding own text vertices
        let mut text_vertices = Vec::new();
        if let Some(text) = &self.text {
            if let Some(clip_rectangle) = self.rectangle {
                text_vertices.append(&mut text.to_vertices(clip_rectangle, viewport_dimensions_px));
            }
        }

        // adding own rectangle/simple vertices
        let mut simple_vertices = Vec::new();
        if let Some(rectangle) = self.rectangle {
            if let Some(colour) = self.background {
                simple_vertices.append(&mut rectangle.to_simple_vertices(colour));
            }
        }

        // adding children's vertices
        for child in self.children.iter() {
            let (mut sv, mut tv) = child.to_vertices(viewport_dimensions_px);
            simple_vertices.append(&mut sv);
            text_vertices.append(&mut tv);
        }

        (simple_vertices, text_vertices)
    }

    fn span(&self) -> Span {
        self.span
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for BaseWidget<Message>
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
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
        _viewport_dimensions_px: glam::Vec2,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        (Vec::new(), Vec::new())
    }
}
