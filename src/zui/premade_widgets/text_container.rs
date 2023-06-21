use std::collections::VecDeque;

use winit::dpi::PhysicalSize;

use crate::zui::{
    primitives::Dimensions,
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Bounds, EventResponse, Layout, LayoutBoundaries},
    Colour, Context, Event, Rectangle, Span, Text, Typeface, Widget,
};

// A basic widget that contains text
pub struct TextContainer {
    text: Option<Text>,
    background_colour: Option<Colour>,
    clip_rectangle: Option<Rectangle<f32>>,
    span: Span,
    span_px: Option<f32>,
    layout: Layout,
}
impl TextContainer {
    pub fn new() -> Self {
        Self {
            text: None,
            background_colour: None,
            clip_rectangle: None,
            span: Span::FitContents,
            span_px: None,
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

impl<Message> Widget<Message> for TextContainer
where
    Message: Copy + Clone,
{
    fn handle_event(
        &mut self,
        event: &crate::zui::Event,
        context: &crate::zui::Context,
    ) -> crate::zui::widget::EventResponse<Message> {
        // responds to no input events
        EventResponse::Consumed
    }

    fn to_vertices(
        &self,
        viewport_dimensions_px: PhysicalSize<u32>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        // adding own text vertices
        let mut text_vertices = Vec::new();
        if let Some(text) = &self.text {
            if let Some(clip_rectangle) = self.clip_rectangle {
                text_vertices.append(&mut text.to_vertices(clip_rectangle, viewport_dimensions_px));
            }
        }

        // adding own rectangle/simple vertices
        let mut simple_vertices = Vec::new();
        if let Some(rectangle) = self.clip_rectangle {
            if let Some(colour) = self.background_colour {
                // simple_vertices.append(&mut rectangle.to_simple_vertices(colour));
                simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                    rectangle,
                    colour,
                    viewport_dimensions_px,
                ));
            }
        }

        (simple_vertices, text_vertices)
    }

    fn try_update_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &crate::zui::Context,
    ) -> Dimensions<f32> {
        if let Some(text) = self.text.as_mut() {
            let boundary_width = layout_boundaries.horizontal.span_px;

            text.update_layout(
                context.font,
                Bounds {
                    span: boundary_width,
                },
                context.viewport_dimensions_px,
            );

            text.dimensions_px().unwrap()
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
            text.place_symbols(context.font, clip_rectangle);
        }
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for TextContainer
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}
