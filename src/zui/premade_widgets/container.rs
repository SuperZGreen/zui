use std::collections::VecDeque;

use crate::zui::{
    primitives::Rectangle, render_layer::RenderLayer, simple_renderer::SimpleVertex,
    text_renderer::TextVertex, widget::EventResponse, Axis, Colour, Context, Event, Span, Text,
    Widget,
};

pub struct Container<Message>
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

    /// style
    pub background: Option<Colour>,

    /// calculated screen space area
    pub rectangle: Option<Rectangle<f32>>,

    /// calculated screen space span
    pub screen_space_span: Option<f32>,

    /// Flag that describes whether the container is overflowing or not
    pub overflowing: bool,

    /// Text contained within a widget's area
    pub text: Option<Text>,
}

impl<Message> Container<Message>
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
            screen_space_span: None,
            overflowing: false,
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

    pub fn update_child_rectangles(&mut self, context: &Context) {
        let self_rectangle = self.rectangle.unwrap();

        // updating the child screen spaces
        for child in &mut self.children {
            child.update_screen_space_span(&self_rectangle, self.axis, None, None, context);
            // println!(
            //     "child screen space span: {}",
            //     child.screen_space_span().unwrap_or(0f32)
            // );
        }
        // println!("");

        // getting the amount of free space within the self span for child widgets
        // let screen_space_available =
        //     Self::get_screen_space_available(&self.children, &self_rectangle, self.axis, context);

        let mut screen_space_used = 0f32;
        for child in &self.children {
            if let Some(screen_space_span) = child.screen_space_span() {
                screen_space_used += screen_space_span;
            }
        }
        let screen_space_available = self_rectangle.span_by_axis(self.axis) - screen_space_used;

        if screen_space_available < 0.0f32 {
            self.overflowing = true;
            warn!("overflowing: screen space span: {}", screen_space_available);
        } else {
            self.overflowing = false;
        }

        let sum_of_child_span_weights = Self::get_sum_of_child_span_weights(&self.children);
        // let mut top_left_accumulator = glam::Vec2::new(self_rectangle.x_min, self_rectangle.y_max);
        let mut used_screen_space_accumulator = 0f32;

        // setting the child rectangles
        for child in self.children.iter_mut() {
            // updating the child weight if not previously updated, ie for weighted spans
            match child.span() {
                Span::ParentWeight(_) => {
                    child.update_screen_space_span(
                        &self_rectangle,
                        self.axis,
                        Some(sum_of_child_span_weights),
                        Some(screen_space_available),
                        context,
                    );
                }
                _ => {}
            }

            let child_screen_space_span = child.screen_space_span().unwrap_or(0f32);

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

            child.handle_event(&Event::FitRectangle((child_rectangle, context)), context);
        }
    }

    fn get_sum_of_child_span_weights(children: &[Box<dyn Widget<Message>>]) -> f32 {
        let mut sum = 0f32;
        for child in children.iter() {
            match child.span() {
                Span::ParentWeight(weight) => sum += weight,
                _ => {}
            }
        }
        sum
    }
}

impl<Message> Widget<Message> for Container<Message>
where
    Message: Clone + Copy,
{
    fn handle_event(&mut self, event: &Event, _context: &Context) -> EventResponse<Message> {
        match event {
            Event::MouseEvent(_) => EventResponse::Propagate,
            Event::FitRectangle((rectangle, context)) => {
                self.rectangle = Some(*rectangle);
                if let Some(text) = &mut self.text {
                    text.update_layout(
                        context.font,
                        &rectangle,
                        context.aspect_ratio,
                        context.viewport_dimensions_px,
                    );
                    text.place_symbols(
                        context.font,
                        &rectangle,
                        context.aspect_ratio,
                        context.viewport_dimensions_px,
                    );
                }
                self.update_child_rectangles(context);

                EventResponse::Consumed
            }
        }
    }

    fn clip_rectangle(&self) -> Option<Rectangle<f32>> {
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
        render_layers: &mut VecDeque<RenderLayer>,
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
            let (mut sv, mut tv) = child.to_vertices(viewport_dimensions_px, render_layers);
            simple_vertices.append(&mut sv);
            text_vertices.append(&mut tv);
        }

        // creating new layer if overflowing
        if self.overflowing {
            let render_layer = RenderLayer::new(simple_vertices, text_vertices, self.rectangle);
            render_layers.push_back(render_layer);
            (Vec::new(), Vec::new())
        } else {
            (simple_vertices, text_vertices)
        }
    }

    fn update_screen_space_span(
        &mut self,
        clip_rectangle: &Rectangle<f32>,
        parent_axis: Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        screen_space_span_available: Option<f32>,
        context: &Context,
    ) {
        self.screen_space_span = Some(match self.span {
            Span::FitContents => {
                if let Some(text) = &mut self.text {
                    text.update_layout(
                        context.font,
                        clip_rectangle,
                        context.aspect_ratio,
                        context.viewport_dimensions_px,
                    );
                    text.screen_space_span(parent_axis).unwrap_or(0f32)
                } else {
                    0f32
                }
            }
            span => span.to_screen_space_span(
                clip_rectangle,
                parent_axis,
                sum_of_parent_weights,
                screen_space_span_available,
                context,
                0f32,
            ),
        })
    }

    fn screen_space_span(&self) -> Option<f32> {
        self.screen_space_span
    }

    fn span(&self) -> Span {
        self.span
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for Container<Message>
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}
