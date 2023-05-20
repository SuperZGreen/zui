use crate::{
    zui::{
        premade_widgets::{Button, FillBar},
        Axis, BaseWidget, Colour, LineWrapping, Scene, Span, Text, TextConfiguration, TextSegment,
        TextSize, Widget,
    },
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
            UiMessage::OptionsMenuMessage(options_menu_message) => match options_menu_message {
                OptionsMenuMessage::BackClicked => {
                    info!("Back clicked!");
                    (
                        Some(UiMessage::GoToScene(SceneIdentifier::StartMenu)),
                        false,
                    )
                }
                OptionsMenuMessage::BarChanged(val) => {
                    // println!("bar changed to: {val}");
                    (None, false)
                }
            },
            _ => (None, false),
        }
    }

    fn view(&self, aspect_ratio: f32) -> Box<dyn Widget<Self::Message>> {
        let button_off_colour = Colour::rgb(0.2f32, 0.3f32, 0.4f32);
        let button_on_colour = Colour::rgb(0.3f32, 0.4f32, 0.6f32);

        let central_content = BaseWidget::new()
            .with_axis(Axis::Vertical)
            .push(BaseWidget::new())
            .push(
                BaseWidget::new()
                    .with_span(Span::ParentWeight(10f32))
                    .with_background(Some(Colour::rgb(0.1f32, 0.1f32, 0.1f32)))
                    .push(
                        BaseWidget::new()
                            .with_span(Span::ViewHeight(0.05f32))
                            .with_text(
                                Text::new()
                                    .with_segment(TextSegment::new("Options Menu", Colour::WHITE))
                                    .with_configuration(TextConfiguration {
                                        size: TextSize::ParentHeight(1f32),
                                        ..Default::default()
                                    }),
                            ),
                    )
                    .push(
                        BaseWidget::new()
                            .with_span(Span::ParentWeight(10f32))
                            .with_text(
                                Text::new()
                                    .with_segment(TextSegment::new(
                                        "This is my text! ",
                                        Colour::WHITE,
                                    ))
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
                                    .with_segment(TextSegment::new(
                                        " Lorem ipsum dolor sit amet, conse\
                                ctetur adipiscing elit. Donec ac sagittis nisl. Vivamus fermentum i\
                                mperdiet magna eu vulputate. Phasellus vitae ex ut turpis dictum di\
                                ctum vel egestas lorem. Vestibulum eu tortor eget nisl suscipit dic\
                                tum ut quis dolor. Duis vitae diam eu eros mattis tincidunt a in du\
                                i. Curabitur euismod, tortor a feugiat mattis, orci libero lacinia \
                                turpis, in elementum risus erat id augue. Nulla non pharetra diam. \
                                Nullam nibh mauris, volutpat at nisl eu, scelerisque iaculis ipsum.\
                                Curabitur porta varius augue. Suspendisse id dui ante. Vivamus at l\
                                orem dictum, mollis dolor sit amet, porttitor sapien. Praesent soda\
                                les in tortor ac volutpat. Mauris in pharetra magna. Curabitur ferm\
                                entum volutpat magna a vehicula.",
                                        Colour::WHITE,
                                    ))
                                    .with_configuration(TextConfiguration {
                                        // size: TextSize::ParentHeight(0.05f32),
                                        size: TextSize::Pixels(16f32),
                                        ..Default::default()
                                    }),
                            ),
                    )
                    .push(
                        FillBar::new(0f32..=100f32, 20f32, false, |val| {
                            UiMessage::OptionsMenuMessage(OptionsMenuMessage::BarChanged(*val))
                        })
                        .with_span(Span::ViewHeight(0.05f32)),
                    )
                    .push(
                        FillBar::new(0f32..=100f32, 20f32, false, |val| {
                            UiMessage::OptionsMenuMessage(OptionsMenuMessage::BarChanged(*val))
                        })
                        .with_span(Span::ViewHeight(0.05f32)),
                    )
                    .push(
                        FillBar::new(0f32..=100f32, 20f32, false, |val| {
                            UiMessage::OptionsMenuMessage(OptionsMenuMessage::BarChanged(*val))
                        })
                        .with_span(Span::ViewHeight(0.05f32)),
                    ),
            )
            .push(BaseWidget::new())
            .push(
                Button::new(
                    UiMessage::OptionsMenuMessage(OptionsMenuMessage::BackClicked),
                    button_off_colour,
                    button_on_colour,
                )
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
            .into()
    }
}
