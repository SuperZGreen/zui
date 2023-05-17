use crate::{
    zui::{premade_widgets::Button, Axis, Colour, Scene, Span, Text, TextSegment, BaseWidget, TextConfiguration, LineWrapping, TextSize},
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

    fn view(&self, aspect_ratio: f32) -> BaseWidget<Self::Message> {
        let central_content = BaseWidget::new()
            .with_axis(Axis::Vertical)
            .push(BaseWidget::new())
            .push(
                BaseWidget::new()
                    .with_span(Span::ParentWeight(10f32))
                    .with_background(Some(Colour::rgb(0.1f32, 0.1f32, 0.1f32)))
                    .with_text(
                        Text::new()
                            .with_segment(TextSegment::new("This is my text! ", Colour::WHITE))
                            .with_segment(TextSegment::new(
                                "This is another part of my text! ",
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
                            .with_segment(TextSegment::new("text!", Colour::WHITE))
                            .with_segment(TextSegment::new(" Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec ac sagittis nisl. Vivamus fermentum imperdiet magna eu vulputate. Phasellus vitae ex ut turpis dictum dictum vel egestas lorem. Vestibulum eu tortor eget nisl suscipit dictum ut quis dolor. Duis vitae diam eu eros mattis tincidunt a in dui. Curabitur euismod, tortor a feugiat mattis, orci libero lacinia turpis, in elementum risus erat id augue. Nulla non pharetra diam. Nullam nibh mauris, volutpat at nisl eu, scelerisque iaculis ipsum. Curabitur porta varius augue. Suspendisse id dui ante. Vivamus at lorem dictum, mollis dolor sit amet, porttitor sapien. Praesent sodales in tortor ac volutpat. Mauris in pharetra magna. Curabitur fermentum volutpat magna a vehicula.", Colour::WHITE))
                            .with_configuration(TextConfiguration {
                                size: TextSize::ParentHeight(0.05f32),
                                ..Default::default()
                            }),
                    ),
            )
            .push(BaseWidget::new())
            .push(
                Button::new(UiMessage::OptionsMenuMessage(
                    OptionsMenuMessage::BackClicked,
                ))
                .with_span(Span::ParentWeight(2f32))
                .with_text(
                    Text::new().with_segment(TextSegment::new("Back to Start", Colour::WHITE)),
                ),
            )
            .push(BaseWidget::new());

        let central_container_span = if aspect_ratio <= 1.1f32 {
            Span::ParentWeight(20f32)
        } else {
            Span::ViewMin(1f32)
        };

        // root widget
        BaseWidget::new()
            .with_axis(Axis::Horizontal)
            .push(BaseWidget::new())
            .push(central_content.with_span(central_container_span))
            .push(BaseWidget::new())
    }
}
