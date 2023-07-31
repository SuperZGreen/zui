use std::collections::VecDeque;

use crate::{zui::{
    self,
    primitives::{Dimensions, Rectangle},
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Bounds, Event, LayoutBoundaries, MouseEvent, Widget, DimensionsError},
    Colour, Context,
}, SpanConstraint};

pub struct Button<Message> {
    cursor_on_colour: Colour,
    cursor_off_colour: Colour,
    on_click_message: Message,
    cursor_is_over: bool,
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
        }
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
    Message: Copy + Clone + 'static,
{

    fn handle_event(&mut self, event: &Event, _context: &Context) {
        match event {
            Event::MouseEvent(MouseEvent::CursorMoved) => {
                // Do nothing
            }
            _ => {}
        }
    }

    fn to_vertices(
        &self,
        region: Rectangle<i32>,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        _text_vertices: &mut Vec<TextVertex>,
        _render_layers: &mut VecDeque<RenderLayer>,
    ) {
    }

    fn calculate_dimensions(
        &self,
        layout_boundaries: &LayoutBoundaries,
        width_contraint: SpanConstraint,
        height_contraint: SpanConstraint,
        _context: &Context,
    ) -> Result<Dimensions<i32>, DimensionsError> {
        let width_px = match width_contraint {
            SpanConstraint::Pixels(px) => px as i32,
            SpanConstraint::ParentRatio(pr) => {
                (layout_boundaries.horizontal.span_px as f32 * pr) as i32
            }
            _ => 64i32,
        };

        let height_px = match height_contraint {
            SpanConstraint::Pixels(px) => px as i32,
            SpanConstraint::ParentRatio(pr) => {
                (layout_boundaries.horizontal.span_px as f32 * pr) as i32
            }
            _ => 64i32,
        };

        Ok(Dimensions {
            width: width_px,
            height: height_px,
        })
    }

}
