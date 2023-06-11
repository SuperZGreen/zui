use crate::zui::{widget::EventResponse, Event, Rectangle, Text, Widget, primitives::Dimensions};

// A Text widget that shrinks to its Text's size
pub struct TextContainer {
    text: Option<Text>,
    clip_rectangle: Option<Rectangle<f32>>,
    span_px: Option<f32>,
}
impl TextContainer {
    pub fn new() -> Self {
        Self {
            text: None,
            clip_rectangle: None,
            span_px: None,
        }
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
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
        match event {
            Event::FitRectangle(clip_rectangle) => {
                self.clip_rectangle = Some(*clip_rectangle);
                if let Some(text) = &mut self.text {
                    text.update_layout(
                        context.font,
                        clip_rectangle.into(),
                        context.viewport_dimensions_px,
                    );
                    text.place_symbols(context.font, clip_rectangle);
                }
                EventResponse::Consumed
            }
            _ => EventResponse::Consumed,
        }
    }

    fn span(&self) -> crate::zui::Span {
        todo!()
    }

    fn min_span_px(
        &mut self,
        parent_axis: crate::zui::Axis,
        region_dimensions_px: Dimensions<f32>,
        viewport_dimensions_px: winit::dpi::PhysicalSize<u32>,
    ) -> Option<f32> {
        None
    }

    fn update_viewport_span_px(
        &mut self,
        clip_rectangle: &Rectangle<f32>,
        parent_axis: crate::zui::Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        viewport_span_px_available: Option<u32>,
        context: &crate::zui::Context,
    ) {
        todo!()
    }
}
