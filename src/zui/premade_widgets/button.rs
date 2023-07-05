use std::collections::VecDeque;

use crate::zui::{
    self,
    primitives::{Dimensions, Rectangle},
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Bounds, Event, Layout, LayoutBoundaries, MouseEvent, Widget},
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
    clip_rectangle: Option<Rectangle<f32>>,

    layout: Layout,
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
            clip_rectangle: None,
            layout: Layout::new(),
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
        context: &Context,
    ) -> zui::widget::EventResponse<Message> {
        match event {
            Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
                self.cursor_is_over = false;
                crate::zui::widget::EventResponse::Propagate
            }
            Event::MouseEvent(MouseEvent::CursorMoved) => {
                // should be Some
                let cursor_position = context.cursor_position.unwrap();

                if let Some(clip_rectangle) = self.clip_rectangle {
                    self.cursor_is_over = clip_rectangle.is_in(&cursor_position);
                }
                crate::zui::widget::EventResponse::Consumed
            }

            Event::MouseEvent(MouseEvent::ButtonReleased) => {
                if self.cursor_is_over {
                    crate::zui::widget::EventResponse::Message(self.on_click_message)
                } else {
                    crate::zui::widget::EventResponse::Propagate
                }
            }

            _ => crate::zui::widget::EventResponse::Propagate,
        }
    }

    fn layout<'a>(&'a self) -> &'a Layout {
        &self.layout
    }

    fn collect_text(&self, symbol_keys: &mut rustc_hash::FxHashSet<zui::typeface::SymbolKey>) {
        self.text.as_ref().map(|t| t.collect_symbol_keys(symbol_keys));
    }

    fn to_vertices(
        &self,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
        _render_layers: &mut VecDeque<RenderLayer>,
    ) {
        if let Some(clip_rectangle) = self.clip_rectangle {
            // getting the colour of the button
            let button_colour = match self.cursor_is_over {
                true => self.cursor_on_colour,
                false => self.cursor_off_colour,
            };

            simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                clip_rectangle,
                button_colour,
                context.viewport_dimensions_px,
            ));

            if let Some(text) = &self.text {
                text_vertices
                    .append(&mut text.to_vertices(clip_rectangle, context.viewport_dimensions_px))
            }
        }
    }

    fn try_update_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &Context,
    ) -> zui::primitives::Dimensions<f32> {
        if let Some(text) = self.text.as_mut() {
            let boundary_width = layout_boundaries.horizontal.span_px;

            text.update_layout(
                context.typeface,
                Bounds {
                    span: boundary_width,
                },
                context.viewport_dimensions_px,
            );

            let dimensions = text.dimensions_px().unwrap();

            self.layout.dimensions_px = Some(dimensions);

            dimensions
        } else {
            Dimensions::new(0f32, 0f32)
        }
    }

    fn try_fit_rectangle(&mut self, clip_rectangle: &Rectangle<f32>, context: &Context) {
        self.clip_rectangle = Some(*clip_rectangle);
        self.text.as_mut().map(|t| t.place_symbols(context.typeface, clip_rectangle));
    }
}
