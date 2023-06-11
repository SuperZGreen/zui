use std::{collections::VecDeque, ops::RangeInclusive};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::zui::{
    self,
    primitives::Rectangle,
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text::{TextAlignmentHorizontal, TextAlignmentVertical},
    widget::EventResponse, Colour, Context, LineWrapping, MouseEvent, Span, Text,
    TextConfiguration, TextSegment, TextSize, Widget, Event,
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

    /// Acts as both the clip rectangle and the background rectangle of the FillBar
    clip_rectangle: Option<Rectangle<f32>>,
    
    /// The foreground of the FillBar, used to render the foreground colour of the bar
    bar_foreground_rectangle: Option<Rectangle<f32>>,

    span: Span,
    span_px: Option<f32>,

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
            bar_foreground_rectangle: None,
            span: Span::ParentWeight(1f32),
            span_px: None,
            text,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    fn format_text(value: &T, max: &T) -> Text {
        Text::new()
            .push_segment(TextSegment::new(
                &format!("{:.2}/{:.2}", value, max),
                Colour::WHITE,
            ))
            .with_configuration(TextConfiguration {
                // TODO: once dynamic text sizing is readded, do this
                // size: TextSize::ParentHeight(1f32),
                size: TextSize::Pixels(32),
                line_wrapping: LineWrapping::None,
                horizontal_alignment: TextAlignmentHorizontal::Right,
                vertical_alignment: TextAlignmentVertical::Centre,
                ..Default::default()
            })
    }

    /// Determines the new 'value' of the FillBar based on the cursor position
    fn determine_value(bar_rectangle: Rectangle<f32>, cursor_position: PhysicalPosition<f64>) -> T {
        let bar_length_px = bar_rectangle.x_max - bar_rectangle.x_min;
        let mut bar_normalised_value =
            (cursor_position.x as f32 - bar_rectangle.x_min) / bar_length_px;
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
            if let Some(clip_rectangle) = &self.clip_rectangle {
                self.text.update_layout(
                    context.font,
                    clip_rectangle.into(),
                    context.viewport_dimensions_px,
                );
                self.text.place_symbols(
                    context.font,
                    &clip_rectangle,
                );

                if let Some(foreground_rectangle) = &mut self.bar_foreground_rectangle {
                    let bar_position = Self::viewport_px_position_from_value(
                        &self.range,
                        *clip_rectangle,
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
    fn viewport_px_position_from_value(
        range: &RangeInclusive<T>,
        bar_rectangle: Rectangle<f32>,
        value: &T,
    ) -> f32 {
        let value: f32 = (*value).into();
        let max: f32 = (*range.end()).into();
        // let min: f32 = (*range.start()).into();
        let bar_normalised_value = value / max;

        bar_normalised_value * bar_rectangle.width() as f32 + bar_rectangle.x_min
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
        event: &Event,
        context: &Context,
    ) -> crate::zui::widget::EventResponse<Message> {
        match event {
            crate::zui::Event::MouseEvent(MouseEvent::CursorMoved(cursor_position_px)) => {
                if let Some(clip_rectangle) = self.clip_rectangle {
                    if self.is_grabbed {
                        let value = Self::determine_value(clip_rectangle, *cursor_position_px);
                        return if self.try_update_value(value, context) {
                            EventResponse::Message((self.on_change)(&self.value))
                        } else {
                            EventResponse::Consumed
                        };
                    }

                    self.cursor_is_over = clip_rectangle.is_in(&cursor_position_px);
                }
                crate::zui::widget::EventResponse::Consumed
            }

            crate::zui::Event::MouseEvent(MouseEvent::ButtonPressed) => {
                if let Some(clip_rectangle) = self.clip_rectangle {
                    if self.cursor_is_over {
                        self.is_grabbed = true;

                        if let Some(cursor_position) = context.cursor_position {
                            let value = Self::determine_value(clip_rectangle, cursor_position);

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

            crate::zui::Event::FitRectangle(clip_rectangle) => {
                self.clip_rectangle = Some(*clip_rectangle);

                // sizing the foreground bar
                let mut bar_foreground_rectangle = *clip_rectangle;
                bar_foreground_rectangle.x_max = Self::viewport_px_position_from_value(
                    &self.range,
                    *clip_rectangle,
                    &self.value,
                );
                self.bar_foreground_rectangle = Some(bar_foreground_rectangle);

                // regenerating text symbols
                self.text.update_layout(
                    context.font,
                    clip_rectangle.into(),
                    context.viewport_dimensions_px,
                );
                self.text.place_symbols(
                    context.font,
                    &clip_rectangle,
                );

                crate::zui::widget::EventResponse::Consumed
            }

            _ => EventResponse::Consumed,
        }
    }

    fn span(&self) -> Span {
        self.span
    }

    fn span_px(&self) -> Option<f32> {
        self.span_px
    }

    fn update_viewport_span_px(
        &mut self,
        parent_rectangle: &Rectangle<f32>,
        parent_axis: zui::Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        parent_span_px_available: Option<u32>,
        context: &Context,
    ) {
        self.span_px = Some(match self.span {
            Span::FitContents => {
                if let Some(clip_rectangle) = &self.clip_rectangle {
                    self.text.update_layout(
                        context.font,
                        clip_rectangle.into(),
                        context.viewport_dimensions_px,
                    );
                    self.text.span_px(parent_axis).unwrap_or(0f32)
                } else {
                    0f32
                }
            }
            span => span.to_viewport_px(
                parent_rectangle,
                parent_axis,
                sum_of_parent_weights,
                parent_span_px_available,
                context,
                0f32,
            ),
        })
    }

    fn to_vertices(
        &self,
        viewport_dimensions_px: PhysicalSize<u32>,
        _render_layers: &mut VecDeque<RenderLayer>,
    ) -> (
        Vec<crate::zui::simple_renderer::SimpleVertex>,
        Vec<crate::zui::text_renderer::TextVertex>,
    ) {
        let mut simple_vertices = Vec::new();
        let mut text_vertices = Vec::new();

        if let Some(clip_rectangle) = self.clip_rectangle {
            simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                clip_rectangle,
                self.back_colour,
                viewport_dimensions_px,
            ));

            let mut text_verts = self
                .text
                .to_vertices(clip_rectangle, viewport_dimensions_px);
            text_vertices.append(&mut text_verts);
        }

        if let Some(foreground_rectangle) = self.bar_foreground_rectangle {
            simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                foreground_rectangle,
                self.front_colour,
                viewport_dimensions_px,
            ));
        }

        (simple_vertices, text_vertices)
    }
}
