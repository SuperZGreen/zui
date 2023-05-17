use crate::{
    zui::{
        premade_widgets::Button, Axis, Colour, Scene, Span, Text, TextConfiguration, TextSegment,
        TextSize, BaseWidget,
    },
    SceneIdentifier, StartMenuMessage, UiMessage,
};

pub struct MainScene {
    // TODO
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            // TODO
        }
    }
}

impl Scene for MainScene {
    type Message = UiMessage;

    fn handle_message(&mut self, message: Self::Message) -> (Option<Self::Message>, bool) {
        (Some(message), false)
    }

    fn view(&self, aspect_ratio: f32) -> crate::zui::BaseWidget<Self::Message> {
        let central_content = BaseWidget::new()
            .with_axis(Axis::Vertical)
            .push(BaseWidget::new())
            .push(
                BaseWidget::new()
                    .with_span(Span::ParentWeight(3f32))
                    .with_background(Some(Colour::rgb(0.2f32, 0.6f32, 0.1f32)))
                    .with_text(
                        Text::new()
                            .with_configuration(TextConfiguration {
                                size: TextSize::ParentHeight(0.8f32),
                                ..Default::default()
                            })
                            .with_segment(TextSegment::new("Welcome! :^)", Colour::WHITE)),
                    ),
            )
            .push(BaseWidget::new())
            .push(
                Button::new(UiMessage::StartMenuMessage(StartMenuMessage::StartClicked))
                    .with_span(Span::ParentWeight(2f32))
                    .with_text(
                        Text::new()
                            .with_configuration(TextConfiguration {
                                size: TextSize::ParentHeight(0.5f32),
                                ..Default::default()
                            })
                            .with_segment(TextSegment::new("Start", Colour::WHITE)),
                    ),
            )
            .push(BaseWidget::new())
            .push(
                Button::new(UiMessage::GoToScene(SceneIdentifier::OptionsMenu))
                    .with_span(Span::ParentWeight(2f32))
                    .with_text(
                        Text::new()
                            .with_configuration(TextConfiguration {
                                size: TextSize::ParentHeight(0.5f32),
                                ..Default::default()
                            })
                            .with_segment(TextSegment::new("Tests/Options", Colour::WHITE)),
                    ),
            )
            .push(BaseWidget::new())
            .push(
                Button::new(UiMessage::Exit)
                    .with_span(Span::ParentWeight(2f32))
                    .with_text(
                        Text::new()
                            .with_configuration(TextConfiguration {
                                size: TextSize::ParentHeight(0.5f32),
                                ..Default::default()
                            })
                            .with_segment(TextSegment::new("Exit", Colour::WHITE)),
                    ),
            )
            .push(BaseWidget::new());

        let central_container_span = if aspect_ratio <= 1.1f32 {
            Span::ParentWeight(20f32)
        } else {
            Span::ViewMin(1f32)
        };

        // let central_container = Widget::new()
        //     .with_axis(Axis::Horizontal)
        //     .with_span(Span::ViewMin(1f32))
        //     .push(Widget::new().with_span(Span::ParentWeight(1f32)))
        //     .push(central_content.with_span(central_container_span))
        //     .push(Widget::new().with_span(Span::ParentWeight(1f32)));

        // root widget
        BaseWidget::new()
            .with_axis(Axis::Horizontal)
            .push(BaseWidget::new())
            .push(central_content.with_span(central_container_span))
            .push(BaseWidget::new())
    }
}
