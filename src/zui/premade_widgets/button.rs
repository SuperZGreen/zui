use std::collections::VecDeque;

use winit::dpi::PhysicalSize;

use crate::zui::{
    self,
    primitives::{Dimensions, Rectangle},
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    widget::{Event, MouseEvent, Widget, LayoutBoundaries, Layout},
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
    span_px: Option<f32>,
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
            span_px: None,
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
            Event::MouseEvent(MouseEvent::CursorMoved(cursor_position)) => {
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

            // crate::zui::widget::Event::FitRectangle(clip_rectangle) => {
            //     self.clip_rectangle = Some(*clip_rectangle);

            //     if let Some(text) = &mut self.text {
            //         text.update_layout(
            //             context.font,
            //             clip_rectangle.into(),
            //             context.viewport_dimensions_px,
            //         );
            //         text.place_symbols(
            //             context.font,
            //             &clip_rectangle,
            //         );
            //     }

            //     crate::zui::widget::EventResponse::Propagate
            // }
            _ => crate::zui::widget::EventResponse::Propagate,
        }
    }
    
    fn layout<'a>(&'a self) -> &'a Layout {
        &self.layout
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
            let button_colour = match self.cursor_is_over {
                true => self.cursor_on_colour,
                false => self.cursor_off_colour,
            };

            simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                clip_rectangle,
                button_colour,
                viewport_dimensions_px,
            ));

            if let Some(text) = &self.text {
                text_vertices.append(&mut text.to_vertices(clip_rectangle, viewport_dimensions_px))
            }
        }

        (simple_vertices, text_vertices)
    }

    fn try_update_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        _context: &Context,
    ) -> zui::primitives::Dimensions<f32> {
        Dimensions::new(64f32, 64f32)
    }

    fn try_fit_rectangle(
        &mut self,
        clip_rectangle: &Rectangle<f32>,
        context: &Context,
    ) {
        info!("fitting button: {clip_rectangle:?}");
        self.clip_rectangle = Some(*clip_rectangle);
    }
    
    
}
