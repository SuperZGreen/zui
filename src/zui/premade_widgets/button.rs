use crate::zui::{
    self,
    primitives::Rectangle,
    widget::{Event, MouseEvent, Widget},
    Colour, Context, Span, Text,
};

pub struct Button<Message> {
    // configuration
    cursor_on_colour: Colour,
    cursor_off_colour: Colour,
    on_click_message: Message,

    // internal state
    cursor_is_over: bool,

    // standard widget state
    text: Option<Text>,
    span: Span,
    screen_space_span: Option<f32>,
    clip_rectangle: Option<Rectangle>,
}

impl<Message> Button<Message> {
    pub fn new(
        on_click_message: Message,
        cursor_off_colour: Colour,
        cursor_on_colour: Colour,
    ) -> Button<Message>
    where
        Message: Copy,
    {
        Self {
            cursor_on_colour,
            cursor_off_colour,
            on_click_message,
            cursor_is_over: false,
            text: None,
            span: Span::ParentWeight(1f32),
            screen_space_span: None,
            clip_rectangle: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for Button<Message>
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}

impl<Message> Widget<Message> for Button<Message>
where
    Message: Copy + Clone,
{
    fn handle_event(
        &mut self,
        event: &Event,
        _context: &Context,
    ) -> zui::widget::EventResponse<Message> {
        match event {
            Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
                self.cursor_is_over = false;
                crate::zui::widget::EventResponse::Propagate
            }
            Event::MouseEvent(MouseEvent::CursorMoved(screen_space_position)) => {
                if let Some(clip_rectangle) = self.clip_rectangle {
                    self.cursor_is_over = clip_rectangle.is_in(*screen_space_position);
                    if self.cursor_is_over {
                        crate::zui::widget::EventResponse::Consumed
                    } else {
                        crate::zui::widget::EventResponse::Consumed
                    }
                } else {
                    crate::zui::widget::EventResponse::Consumed
                }
            }

            Event::MouseEvent(MouseEvent::ButtonReleased) => {
                if self.cursor_is_over {
                    crate::zui::widget::EventResponse::Message(self.on_click_message)
                } else {
                    crate::zui::widget::EventResponse::Propagate
                }
            }

            crate::zui::widget::Event::FitRectangle((clip_rectangle, context)) => {
                self.clip_rectangle = Some(*clip_rectangle);

                if let Some(text) = &mut self.text {
                    text.update_layout(
                        context.font,
                        clip_rectangle,
                        context.aspect_ratio,
                        context.viewport_dimensions_px,
                    );
                    text.place_symbols(
                        context.font,
                        &clip_rectangle,
                        context.aspect_ratio,
                        context.viewport_dimensions_px,
                    );
                }

                crate::zui::widget::EventResponse::Propagate
            }
            _ => crate::zui::widget::EventResponse::Propagate,
        }
    }

    fn children_iter_mut(
        &mut self,
    ) -> Option<std::slice::IterMut<'_, Box<(dyn Widget<Message> + 'static)>>> {
        None
    }

    fn clip_rectangle(&self) -> Option<crate::zui::primitives::Rectangle> {
        self.clip_rectangle
    }

    fn span(&self) -> Span {
        self.span
    }

    fn screen_space_span(&self) -> Option<f32> {
        self.screen_space_span
    }

    fn update_screen_space_span(
        &mut self,
        parent_rectangle: &Rectangle,
        parent_axis: zui::Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        screen_space_span_available: Option<f32>,
        context: &Context,
    ) {
        self.screen_space_span = Some(match self.span {
            Span::FitContents => {
                if let Some(text) = &mut self.text {
                    if let Some(clip_rectangle) = &self.clip_rectangle {
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
                } else {
                    0f32
                }
            }
            span => span.to_screen_space_span(
                parent_rectangle,
                parent_axis,
                sum_of_parent_weights,
                screen_space_span_available,
                context,
                0f32,
            ),
        })
    }

    fn to_vertices(
        &self,
        viewport_dimensions_px: glam::Vec2,
    ) -> (
        Vec<crate::zui::renderer::SimpleVertex>,
        Vec<crate::zui::text_renderer::TextVertex>,
    ) {
        let mut simple_vertices = Vec::new();
        let mut text_vertices = Vec::new();
        if let Some(clip_rectangle) = self.clip_rectangle {
            let button_colour = match self.cursor_is_over {
                true => self.cursor_on_colour,
                false => self.cursor_off_colour,
            };

            simple_vertices.append(&mut clip_rectangle.to_simple_vertices(button_colour));

            if let Some(text) = &self.text {
                text_vertices.append(&mut text.to_vertices(clip_rectangle, viewport_dimensions_px))
            }
        }

        (simple_vertices, text_vertices)
    }
}
