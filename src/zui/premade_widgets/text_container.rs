use std::collections::VecDeque;

use crate::{
    zui::{
        primitives::Dimensions,
        render_layer::RenderLayer,
        simple_renderer::SimpleVertex,
        text_renderer::TextVertex,
        widget::{Bounds, EventResponse, Layout, LayoutBoundaries},
        Colour, Context, Rectangle, Text, Widget,
    },
    StateStore,
};

// A basic widget that contains text
pub struct TextContainer<StateIdentifier> {
    text: Option<Text>,
    _state_identifier: StateIdentifier,
    background_colour: Option<Colour>,
    clip_rectangle: Option<Rectangle<f32>>,
    layout: Layout,
}
impl<StateIdentifier> TextContainer<StateIdentifier> {
    pub fn new(state_identifier: StateIdentifier) -> Self {
        Self {
            text: None,
            _state_identifier: state_identifier,
            background_colour: None,
            clip_rectangle: None,
            layout: Layout::new(),
        }
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_background_colour(mut self, colour: Option<Colour>) -> Self {
        self.background_colour = colour;
        self
    }
}

impl<Message, StateIdentifier> Widget<Message, StateIdentifier> for TextContainer<StateIdentifier>
where
    Message: Copy + Clone,
    StateIdentifier: std::fmt::Debug + Eq + std::hash::Hash,
{
    fn handle_event(
        &mut self,
        _event: &crate::zui::Event,
        _context: &crate::zui::Context,
        _state_store: &mut StateStore<StateIdentifier>,
    ) -> crate::zui::widget::EventResponse<Message> {
        // responds to no input events
        EventResponse::Consumed
    }

    fn to_vertices(
        &self,
        context: &Context,
        _state_store: &StateStore<StateIdentifier>,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
        _render_layers: &mut VecDeque<RenderLayer>,
    ) {
        // adding own text vertices
        if let Some(text) = &self.text {
            if let Some(clip_rectangle) = self.clip_rectangle {
                text_vertices
                    .append(&mut text.to_vertices(clip_rectangle, context.viewport_dimensions_px));
            }
        }

        // adding own rectangle/simple vertices
        if let Some(rectangle) = self.clip_rectangle {
            if let Some(colour) = self.background_colour {
                simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                    rectangle,
                    colour,
                    context.viewport_dimensions_px,
                ));
            }
        }
    }

    fn try_update_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &crate::zui::Context,
    ) -> Dimensions<f32> {
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

    fn layout<'a>(&'a self) -> &'a Layout {
        &self.layout
    }

    fn try_fit_rectangle(&mut self, clip_rectangle: &Rectangle<f32>, context: &Context) {
        self.clip_rectangle = Some(*clip_rectangle);
        if let Some(text) = self.text.as_mut() {
            text.place_symbols(context.typeface, clip_rectangle);
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
}

impl<'a, Message, StateIdentifier> Into<Box<dyn Widget<Message, StateIdentifier> + 'a>>
    for TextContainer<StateIdentifier>
where
    Message: Clone + Copy + 'a,
    StateIdentifier: std::fmt::Debug + Eq + std::hash::Hash + 'a,
{
    fn into(self) -> Box<dyn Widget<Message, StateIdentifier> + 'a> {
        Box::new(self)
    }
}
