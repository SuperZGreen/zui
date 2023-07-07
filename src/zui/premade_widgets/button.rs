use std::collections::VecDeque;

use crate::{
    zui::{
        self,
        primitives::{Dimensions, Rectangle},
        render_layer::RenderLayer,
        simple_renderer::SimpleVertex,
        text_renderer::TextVertex,
        widget::{Bounds, Event, Layout, LayoutBoundaries, MouseEvent, Widget},
        Colour, Context, Span, Text,
    },
    StateStore,
};

/// The internal state of the Button
pub struct ButtonState {
    /// Describes whether the cursor is over the button or not, used for highlighting
    cursor_is_over: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self {
            cursor_is_over: false
        }
    }
}

pub struct Button<Message, StateIdentifier> {
    // configuration
    cursor_on_colour: Colour,
    cursor_off_colour: Colour,
    on_click_message: Message,

    state_identifier: StateIdentifier,

    // standard widget state
    text: Option<Text>,
    span: Span,
    clip_rectangle: Option<Rectangle<f32>>,

    layout: Layout,
}

impl<Message, StateIdentifier> Button<Message, StateIdentifier>
where
    StateIdentifier: std::hash::Hash + std::fmt::Debug + Eq + Copy,
{
    pub fn new(
        on_click_message: Message,
        cursor_off_colour: Colour,
        cursor_on_colour: Colour,
        state_store: &mut StateStore<StateIdentifier>,
        state_identifier: StateIdentifier,
    ) -> Button<Message, StateIdentifier>
    where
        Message: Copy,
    {
        state_store.try_insert(state_identifier, Box::new(ButtonState::default()));

        Self {
            cursor_on_colour,
            cursor_off_colour,
            on_click_message,
            state_identifier,
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

impl<'a, Message, StateIdentifier> Into<Box<dyn Widget<Message, StateIdentifier> + 'a>>
    for Button<Message, StateIdentifier>
where
    Message: Clone + Copy + 'a,
    StateIdentifier: std::hash::Hash + Eq + std::fmt::Debug + 'a,
{
    fn into(self) -> Box<dyn Widget<Message, StateIdentifier> + 'a> {
        Box::new(self)
    }
}

impl<Message, StateIdentifier> Widget<Message, StateIdentifier> for Button<Message, StateIdentifier>
where
    Message: Copy + Clone,
    StateIdentifier: std::hash::Hash + Eq + std::fmt::Debug,
{
    fn handle_event(
        &mut self,
        event: &Event,
        context: &Context,
        state_store: &mut StateStore<StateIdentifier>,
    ) -> zui::widget::EventResponse<Message> {
        match event {
            Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
                let state = state_store
                    .get_as_mut::<ButtonState>(&self.state_identifier)
                    .unwrap();
                state.cursor_is_over = false;
                crate::zui::widget::EventResponse::Propagate
            }
            Event::MouseEvent(MouseEvent::CursorMoved) => {
                // should be Some
                let cursor_position = context.cursor_position.unwrap();

                if let Some(clip_rectangle) = self.clip_rectangle {
                    let state = state_store
                        .get_as_mut::<ButtonState>(&self.state_identifier)
                        .unwrap();
                    state.cursor_is_over = clip_rectangle.is_in(&cursor_position);
                }
                crate::zui::widget::EventResponse::Consumed
            }

            Event::MouseEvent(MouseEvent::ButtonReleased) => {
                let state = state_store
                    .get_as::<ButtonState>(&self.state_identifier)
                    .unwrap();
                if state.cursor_is_over {
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
        self.text
            .as_ref()
            .map(|t| t.collect_symbol_keys(symbol_keys));
    }

    fn to_vertices(
        &self,
        context: &Context,
        state_store: &StateStore<StateIdentifier>,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
        _render_layers: &mut VecDeque<RenderLayer>,
    ) {
        if let Some(clip_rectangle) = self.clip_rectangle {
            // getting the colour of the button
            let state = state_store
                .get_as::<ButtonState>(&self.state_identifier)
                .unwrap();
            let button_colour = match state.cursor_is_over {
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
        self.text
            .as_mut()
            .map(|t| t.place_symbols(context.typeface, clip_rectangle));
    }
}
