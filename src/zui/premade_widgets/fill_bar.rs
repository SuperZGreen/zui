use std::ops::RangeInclusive;

use crate::zui::{
    primitives::Rectangle, widget::EventResponse, Colour, Context, LineWrapping, MouseEvent,
    ScreenSpacePosition, Span, Text, TextConfiguration, TextSegment, TextSize, Widget,
};

pub struct FillBar<'a, T, Message> {
    front_colour: Colour,
    back_colour: Colour,

    /// The range of the bar
    range: RangeInclusive<T>,
    /// The current value of the bar
    value: T,

    /// Can be dragged by the user
    _interactable: bool,

    /// Message when being dragged by user
    on_change: Box<dyn Fn(&T) -> Message + 'a>,

    cursor_is_over: bool,
    is_grabbed: bool,

    clip_rectangle: Option<Rectangle>,
    bar_background_rectangle: Option<Rectangle>,
    bar_foreground_rectangle: Option<Rectangle>,

    span: Span,
    text: Text,
}

impl<'a, T, Message> FillBar<'a, T, Message>
where
    T: 'a + From<f32> + std::cmp::PartialEq + std::fmt::Display + Copy,
    f32: From<T>,
{
    pub fn new<F>(range: RangeInclusive<T>, value: T, interactable: bool, on_change: F) -> Self
    where
        F: Fn(&T) -> Message + 'a,
    {
        let text = Self::format_text(&value, range.end());
        Self {
            front_colour: Colour::rgb(0.3f32, 0.3f32, 0.3f32),
            back_colour: Colour::rgb(0.05f32, 0.05f32, 0.05f32),
            range,
            value,
            _interactable: interactable,
            on_change: Box::new(on_change),
            cursor_is_over: false,
            is_grabbed: false,
            clip_rectangle: None,
            bar_background_rectangle: None,
            bar_foreground_rectangle: None,
            span: Span::ParentWeight(1f32),
            text,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    fn format_text(value: &T, max: &T) -> Text {
        Text::new()
            .with_segment(TextSegment::new(
                &format!("{:.2}/{:.2}", value, max),
                Colour::WHITE,
            ))
            .with_configuration(TextConfiguration {
                size: TextSize::ParentHeight(1f32),
                line_wrapping: LineWrapping::None,
            })
    }

    fn determine_value(bar_rectangle: Rectangle, cursor_position: ScreenSpacePosition) -> T {
        let bar_screen_space_length = bar_rectangle.x_max - bar_rectangle.x_min;
        let mut bar_normalised_value =
            (cursor_position.x - bar_rectangle.x_min) / bar_screen_space_length;
        if bar_normalised_value < 0f32 {
            bar_normalised_value = 0f32;
        } else if bar_normalised_value > 1f32 {
            bar_normalised_value = 1f32;
        }

        (bar_normalised_value * 100f32).into()
    }

    /// Sets the value if not already equal to current value, returns true if updated
    fn try_update_value(&mut self, value: T, context: &Context) -> bool {
        if value == self.value {
            false
        } else {
            self.text = Self::format_text(&value, self.range.end());
            if let Some(background_rectangle) = self.bar_background_rectangle {
                self.text
                    .update_symbols(context.font, &background_rectangle, context.aspect_ratio);

                if let Some(foreground_rectangle) = &mut self.bar_foreground_rectangle {
                    let bar_position = Self::screen_space_position_from_value(
                        &self.range,
                        background_rectangle,
                        &value,
                    );
                    foreground_rectangle.x_max = bar_position;
                }
            }
            self.value = value;
            true
        }
    }

    /// Gets the screen space position of
    fn screen_space_position_from_value(
        range: &RangeInclusive<T>,
        bar_rectangle: Rectangle,
        value: &T,
    ) -> f32 {
        let value: f32 = (*value).into();
        let max: f32 = (*range.end()).into();
        // let min: f32 = (*range.start()).into();
        let bar_normalised_value = value / max;

        bar_normalised_value * bar_rectangle.width() + bar_rectangle.x_min
    }
}

impl<'a, T, Message> Into<Box<dyn Widget<Message> + 'a>> for FillBar<'a, T, Message>
where
    Message: Clone + Copy + 'a,
    T: 'a + std::convert::From<f32> + std::cmp::PartialEq + std::fmt::Display + Copy,
    f32: From<T>,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}

impl<'a, T, Message> Widget<Message> for FillBar<'a, T, Message>
where
    Message: Copy + Clone,
    T: std::convert::From<f32> + std::cmp::PartialEq + std::fmt::Display + 'a + Copy,
    f32: From<T>,
{
    fn handle_event(
        &mut self,
        event: &crate::zui::Event,
        context: &Context,
    ) -> crate::zui::widget::EventResponse<Message> {
        match event {
            crate::zui::Event::MouseEvent(MouseEvent::CursorMoved(screen_space_position)) => {
                if let Some(bar_rectangle) = self.bar_background_rectangle {
                    if self.is_grabbed {
                        let value = Self::determine_value(bar_rectangle, *screen_space_position);
                        return if self.try_update_value(value, context) {
                            EventResponse::Message((self.on_change)(&self.value))
                        } else {
                            EventResponse::Consumed
                        };
                    }

                    self.cursor_is_over = bar_rectangle.is_in(*screen_space_position);
                }
                crate::zui::widget::EventResponse::Consumed
            }

            crate::zui::Event::MouseEvent(MouseEvent::ButtonPressed) => {
                if let Some(bar_rectangle) = self.bar_background_rectangle {
                    if self.cursor_is_over {
                        self.is_grabbed = true;

                        if let Some(cursor_position) = context.cursor_position {
                            let value = Self::determine_value(bar_rectangle, cursor_position);

                            return if self.try_update_value(value, context) {
                                EventResponse::Message((self.on_change)(&self.value))
                            } else {
                                EventResponse::Consumed
                            };
                        }
                    }
                }
                crate::zui::widget::EventResponse::Consumed
            }

            crate::zui::Event::MouseEvent(MouseEvent::ButtonReleased) => {
                self.is_grabbed = false;
                crate::zui::widget::EventResponse::Consumed
            }

            crate::zui::Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
                self.cursor_is_over = false;
                crate::zui::widget::EventResponse::Consumed
            }

            crate::zui::Event::FitRectangle((clip_rectangle, context)) => {
                self.clip_rectangle = Some(*clip_rectangle);
                self.bar_background_rectangle = Some(*clip_rectangle);
                let mut bar_foreground_rectangle = *clip_rectangle;
                bar_foreground_rectangle.x_max =
                    (bar_foreground_rectangle.x_min + bar_foreground_rectangle.x_max) / 2f32;
                self.bar_foreground_rectangle = Some(bar_foreground_rectangle);
                self.text
                    .update_symbols(context.font, &clip_rectangle, context.aspect_ratio);

                crate::zui::widget::EventResponse::Consumed
            }

            _ => EventResponse::Consumed,
        }
    }

    fn span(&self) -> Span {
        self.span
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

        if let Some(background_rectangle) = self.bar_background_rectangle {
            simple_vertices.append(&mut background_rectangle.to_simple_vertices(self.back_colour));
            let mut text_verts = self
                .text
                .to_vertices(background_rectangle, viewport_dimensions_px);
            text_vertices.append(&mut text_verts);
        }

        if let Some(foreground_rectangle) = self.bar_foreground_rectangle {
            simple_vertices.append(&mut foreground_rectangle.to_simple_vertices(self.front_colour));
        }

        (simple_vertices, text_vertices)
    }
}
