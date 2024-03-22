use std::collections::VecDeque;

use crate::{
    zui::{
        primitives::{Dimensions, Rectangle},
        render_layer::RenderLayer,
        renderers::{SimpleVertex, TextVertex},
        widget::{Bounds, Event, LayoutBoundaries, MouseEvent, Widget},
        widget_store::EntryDefaultDescriptor,
        Colour, Context,
    },
    PaddingWeights, PositionConstraint, SpanConstraint, Text,
};

pub struct Button<Message> {
    cursor_on_colour: Colour,
    cursor_off_colour: Colour,
    on_click_message: Message,
    cursor_is_over: bool,
    text: Option<Text>,
}

impl<Message> Button<Message> {
    pub fn new(
        on_click_message: Message,
        cursor_off_colour: Colour,
        cursor_on_colour: Colour,
    ) -> Button<Message>
    where
        Message: Clone,
    {
        Self {
            cursor_on_colour,
            cursor_off_colour,
            on_click_message,
            cursor_is_over: false,
            text: None,
        }
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for Button<Message>
where
    Message: Clone + 'static,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}

impl<Message> Widget<Message> for Button<Message>
where
    Message: Clone + 'static,
{
    fn handle_event(
        &mut self,
        event: &Event,
        region: &Rectangle<i32>,
        context: &Context,
    ) -> Option<Message> {
        match event {
            Event::MouseEvent(MouseEvent::CursorMoved) => {
                if let Some(cursor_position) = context.cursor_position {
                    if region.is_in(&cursor_position) {
                        self.cursor_is_over = true;
                    } else {
                        self.cursor_is_over = false;
                    }
                }

                None
            }

            Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
                self.cursor_is_over = false;

                None
            }

            Event::MouseEvent(MouseEvent::ButtonReleased) => {
                if self.cursor_is_over {
                    Some(self.on_click_message.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn to_vertices(
        &self,
        region: Rectangle<i32>,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
    ) {
        // adding own text vertices
        if let Some(text) = &self.text {
            text_vertices.append(&mut text.to_vertices(region, context.viewport_dimensions_px));
        }

        let colour = if self.cursor_is_over {
            self.cursor_on_colour
        } else {
            self.cursor_off_colour
        };

        // adding own rectangle/simple vertices
        simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
            region,
            colour,
            context.viewport_dimensions_px,
        ));
    }

    fn calculate_minimum_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &Context,
    ) -> Dimensions<i32> {
        if let Some(text) = self.text.as_mut() {
            let boundary_width = layout_boundaries.horizontal.span_px;

            text.update_layout(
                context.typeface,
                Bounds {
                    span: boundary_width as f32, // TODO
                },
                context.viewport_dimensions_px,
            );

            let dimensions = text.dimensions_px().unwrap();

            // self.layout.dimensions_px = Some(dimensions);

            dimensions
        } else {
            Dimensions::new(0i32, 0i32)
        }
    }

    fn entry_default_descriptor(&self) -> crate::zui::widget_store::EntryDefaultDescriptor {
        EntryDefaultDescriptor {
            children: None,
            width_constraint: SpanConstraint::FitContents,
            height_constraint: SpanConstraint::FitContents,
            position_constraint: PositionConstraint::ParentDetermined(PaddingWeights::NONE),
        }
    }

    fn place(&mut self, region: &Rectangle<i32>, context: &Context) {
        if let Some(text) = self.text.as_mut() {
            text.place_symbols(context.typeface, region);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn collect_text(&self, symbol_keys: &mut rustc_hash::FxHashSet<crate::typeface::SymbolKey>) {
        let text = match self.text.as_ref() {
            Some(text) => text,
            None => return,
        };

        text.collect_symbol_keys(symbol_keys);
    }
}
