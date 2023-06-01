use crate::{
    zui::{
        premade_widgets::Container, Axis, Colour, Scene, Span, Text, TextConfiguration,
        TextSegment, TextSize, Widget,
    },
    UiMessage,
};

use std::default::{self, Default};

pub struct GameScene {
    // TODO
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            // TODO
        }
    }
}

impl Scene for GameScene {
    type Message = UiMessage;

    fn handle_message(&mut self, message: Self::Message) -> (Option<Self::Message>, bool) {
        (Some(message), false)
    }

    fn view(&self, _aspect_ratio: f32) -> Box<dyn Widget<Self::Message>> {
        let text = Text::new()
            .with_configuration(TextConfiguration {
                size: TextSize::Pixels(32),
                ..Default::default()
            })
            .push_segment(TextSegment::new("Hello", Colour::WHITE));

        let left_container = Container::new()
            .with_background(Some(Colour::rgb(1f32, 0f32, 0.5f32)))
            .push(Container::new().with_background(Some(Colour::WHITE)))
            .push(Container::new().with_background(Some(Colour::WHITE)))
            .push(Container::new().with_background(Some(Colour::WHITE)));

        let right_container = Container::new()
            .with_background(Some(Colour::rgb(0f32, 0.5f32, 1f32)))
            .push(
                Container::new()
                    .with_background(Some(Colour::DARK_GREEN))
                    .with_span(Span::ParentRatio(0.2f32))
                    .with_text(text.clone()),
            )
            .push(
                Container::new()
                    .with_background(Some(Colour::GREEN))
                    .with_span(Span::ParentRatio(0.2f32))
                    .with_text(text.clone()),
            )
            .push(
                Container::new()
                    .with_background(Some(Colour::LIGHT_GREEN))
                    .with_span(Span::ParentRatio(0.2f32))
                    .with_text(text.clone()),
            );
        Container::new()
            .with_axis(Axis::Horizontal)
            .push(left_container)
            .push(right_container)
            .into()
    }
}
