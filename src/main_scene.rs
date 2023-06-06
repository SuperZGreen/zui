use crate::{
    zui::{
        premade_widgets::{Button, Container},
        Axis, Colour, Scene, Span, Text, TextAlignmentHorizontal, TextConfiguration, TextSegment, TextSize,
        Widget, text::TextAlignmentVertical,
    },
    SceneIdentifier, UiMessage,
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

    fn view(&self, aspect_ratio: f32) -> Box<dyn Widget<Self::Message>> {
        let button_off_colour = Colour::rgb(0.2f32, 0.3f32, 0.4f32);
        let button_on_colour = Colour::rgb(0.3f32, 0.4f32, 0.6f32);
        
        let button_text_configuration = TextConfiguration {
            size: TextSize::Pixels(64),
            horizontal_alignment: TextAlignmentHorizontal::Centre,
            vertical_alignment: TextAlignmentVertical::Centre,
            ..Default::default()
        };

        let central_content = Container::new()
            .with_axis(Axis::Vertical)
            .with_background(Some(Colour::rgb(1f32, 0f32, 0.5f32)))
            .push(Container::new())
            .push(
                Container::new()
                    .with_span(Span::ParentWeight(3f32))
                    .with_background(Some(Colour::rgb(0.2f32, 0.6f32, 0.1f32)))
                    .with_text(
                        Text::new()
                            .with_configuration(TextConfiguration {
                                size: TextSize::Pixels(64),
                                vertical_alignment: TextAlignmentVertical::Bottom,
                                horizontal_alignment: TextAlignmentHorizontal::Right,
                                ..Default::default()
                            })
                            .push_segment(TextSegment::new("Welcome! :^)", Colour::WHITE)),
                    ),
            )
            .push(Container::new())
            .push(
                Button::new(
                    UiMessage::GoToScene(SceneIdentifier::GameScene),
                    button_off_colour,
                    button_on_colour,
                )
                .with_span(Span::ParentWeight(2f32))
                .with_text(
                    Text::new()
                        .with_configuration(button_text_configuration.clone())
                        .push_segment(TextSegment::new("Start", Colour::WHITE)),
                ),
            )
            .push(Container::new())
            .push(
                Button::new(
                    UiMessage::GoToScene(SceneIdentifier::OptionsMenu),
                    button_off_colour,
                    button_on_colour,
                )
                .with_span(Span::ParentWeight(2f32))
                .with_text(
                    Text::new()
                        .with_configuration(button_text_configuration.clone())
                        .push_segment(TextSegment::new("Tests/Options", Colour::WHITE)),
                ),
            )
            .push(Container::new())
            .push(
                Button::new(UiMessage::Exit, button_off_colour, button_on_colour)
                    .with_span(Span::ParentWeight(2f32))
                    .with_text(
                        Text::new()
                            .with_configuration(button_text_configuration.clone())
                            .push_segment(TextSegment::new("Exit", Colour::WHITE)),
                    ),
            )
            .push(Container::new());

        // let central_container_span = if aspect_ratio <= 1.1f32 {
        //     Span::ParentWeight(20f32)
        // } else {
        //     Span::ViewMin(1f32)
        // };
        let central_container_span = Span::ViewMin(1f32);

        // let central_container = Widget::new()
        //     .with_axis(Axis::Horizontal)
        //     .with_span(Span::ViewMin(1f32))
        //     .push(Widget::new().with_span(Span::ParentWeight(1f32)))
        //     .push(central_content.with_span(central_container_span))
        //     .push(Widget::new().with_span(Span::ParentWeight(1f32)));

        // root widget
        Container::new()
            .with_background(Some(Colour::rgb(0.1f32, 0.1f32, 0.1f32)))
            .with_axis(Axis::Horizontal)
            .push(Container::new())
            .push(central_content.with_span(central_container_span))
            .push(Container::new())
            .into()
    }
}
