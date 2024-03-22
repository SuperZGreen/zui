use std::collections::VecDeque;

use crate::{
    text::TextDescriptor, zui::{
        primitives::Dimensions,
        render_layer::RenderLayer,
        renderers::{SimpleVertex, TextVertex},
        widget::{Bounds, LayoutBoundaries},
        widget_store::EntryDefaultDescriptor,
        Colour, Context, Rectangle, Text, Widget,
    }, Event, PaddingWeights, PositionConstraint, SpanConstraint, TextSegment
};

/// A widget that contains and displays a block of text.
pub struct TextContainer {
    /// The text that the text container contains.
    text: Option<Text>,

    /// The background colour of the text container.
    background_colour: Option<Colour>,
}

#[allow(dead_code)]
impl TextContainer {
    pub fn new() -> Self {
        Self {
            text: None,
            background_colour: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_background_colour(mut self, colour: Option<Colour>) -> Self {
        self.background_colour = colour;
        self
    }

    pub fn set_text(&mut self, text: Text) {
        self.text = Some(text);
    }
}

impl Default for TextContainer {
    fn default() -> Self {
        Self {
            text: None,
            background_colour: None,
        }
    }
}

impl<Message> Widget<Message> for TextContainer
where
    Message: Clone,
{
    fn handle_event(
        &mut self,
        event: &Event,
        _region: &Rectangle<i32>,
        _context: &Context,
    ) -> Option<Message> {
        match event {
            _ => {}
        }

        None
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

        // adding own rectangle/simple vertices
        if let Some(colour) = self.background_colour {
            simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                region,
                colour,
                context.viewport_dimensions_px,
            ));
        }
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

    fn place(&mut self, region: &Rectangle<i32>, context: &Context) {
        if let Some(text) = self.text.as_mut() {
            text.place_symbols(context.typeface, region);
        }
    }

    fn collect_text(
        &self,
        symbol_keys: &mut rustc_hash::FxHashSet<crate::zui::typeface::SymbolKey>,
    ) {
        let text = match self.text.as_ref() {
            Some(text) => text,
            None => return,
        };

        text.collect_symbol_keys(symbol_keys);
    }

    fn entry_default_descriptor(&self) -> crate::zui::widget_store::EntryDefaultDescriptor {
        EntryDefaultDescriptor {
            children: None,
            width_constraint: SpanConstraint::FitContents,
            height_constraint: SpanConstraint::FitContents,
            position_constraint: PositionConstraint::ParentDetermined(PaddingWeights::NONE),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for TextContainer
where
    Message: Clone + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}

impl From<&str> for TextContainer {
    fn from(value: &str) -> Self {
        Self {
            text: Some(Text::from(TextDescriptor {
                segments: vec![TextSegment::from(value)],
                ..Default::default()
            })),
            ..Default::default()
        }
    }
}


/// A descriptor for a TextContainer
pub struct TextContainerDescriptor {
    /// The text that the text container contains
    pub text: Option<Text>,

    /// The background colour of the text container.
    pub background_colour: Option<Colour>,
}

impl Default for TextContainerDescriptor {
    fn default() -> Self {
        Self {
            text: None,
            background_colour: None,
        }
    }
}

impl From<TextContainerDescriptor> for TextContainer {
    fn from(descriptor: TextContainerDescriptor) -> Self {
        Self {
            text: descriptor.text,
            background_colour: descriptor.background_colour,
        }
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for TextContainerDescriptor
where
    Message: Clone + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(TextContainer::from(self))
    }
}
