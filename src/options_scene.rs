use crate::{
    zui::{premade_widgets::Button, Axis, Colour, Scene, Span, Text, TextSegment, Widget},
    OptionsMenuMessage, SceneIdentifier, UiMessage,
};

pub struct OptionsScene {
    // TODO
}

impl OptionsScene {
    pub fn new() -> Self {
        Self {
            // TODO
        }
    }
}

impl Scene for OptionsScene {
    type Message = UiMessage;

    fn handle_message(&mut self, message: Self::Message) -> (Option<Self::Message>, bool) {
        let rebuild_required = false;

        match message {
            UiMessage::OptionsMenuMessage(OptionsMenuMessage::BackClicked) => {
                info!("Back clicked!");
                (
                    Some(UiMessage::GoToScene(SceneIdentifier::StartMenu)),
                    false,
                )
            }
            _ => (None, false),
        }
    }

    fn view(&self, aspect_ratio: f32) -> Widget<Self::Message> {
        let central_content = Widget::new()
            .with_axis(Axis::Vertical)
            .push(Widget::new())
            .push(
                Widget::new()
                    .with_span(Span::ParentWeight(10f32))
                    .with_background(Some(Colour::rgb(0.1f32, 0.1f32, 0.1f32)))
                    .with_text(
                        Text::new()
                            .with_segment(TextSegment::new("This is my text!", Colour::WHITE))
                            .with_segment(TextSegment::new(
                                " This is another part of my text!",
                                Colour::rgb(0f32, 1f32, 1f32),
                            ))
                            .with_segment(TextSegment::new(
                                "This is some more text ",
                                Colour::WHITE,
                            ))
                            .with_segment(TextSegment::new("This is my ", Colour::WHITE))
                            .with_segment(TextSegment::new(
                                "FAVOURITE ",
                                Colour::rgb(0.7f32, 1f32, 0.7f32),
                            ))
                            .with_segment(TextSegment::new("text!", Colour::WHITE)),
                    ),
            )
            .push(Widget::new())
            .push(
                Button::new(UiMessage::OptionsMenuMessage(
                    OptionsMenuMessage::BackClicked,
                ))
                .with_span(Span::ParentWeight(2f32))
                .with_text(
                    Text::new().with_segment(TextSegment::new("Back to Start", Colour::WHITE)),
                ),
            )
            .push(Widget::new());

        let central_container_span = if aspect_ratio <= 1.1f32 {
            Span::ParentWeight(20f32)
        } else {
            Span::ViewMin(1f32)
        };

        // root widget
        Widget::new()
            .with_axis(Axis::Horizontal)
            .push(Widget::new())
            .push(central_content.with_span(central_container_span))
            .push(Widget::new())
    }
}
