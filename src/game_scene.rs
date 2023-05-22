use crate::{
    zui::{
        Axis, BaseWidget, Colour, Scene, Text, TextConfiguration, TextSegment, TextSize, Widget,
    },
    UiMessage,
};

use std::default::Default;

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
        BaseWidget::new()
            .with_axis(Axis::Horizontal)
            .push(
                BaseWidget::new().with_text(
                    Text::new()
                        .push_segment(TextSegment::new("p", Colour::WHITE))
                        .with_configuration(TextConfiguration {
                            size: TextSize::Pixels(1387f32),
                            ..Default::default()
                        }),
                ),
            )
            .push(
                BaseWidget::new().with_text(
                    Text::new()
                        .push_segment(TextSegment::new("p", Colour::WHITE))
                        .with_configuration(TextConfiguration {
                            size: TextSize::ParentHeight(1f32),
                            ..Default::default()
                        }),
                ),
            )
            .into()
    }
}
