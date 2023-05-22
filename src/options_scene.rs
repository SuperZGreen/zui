use std::ops::RangeInclusive;

use crate::{
    zui::{
        premade_widgets::{Button, FillBar},
        Axis, BaseWidget, Colour, Scene, Span, Text, TextConfiguration, TextSegment, TextSize,
        Widget,
    },
    OptionsMenuMessage, SceneIdentifier, UiMessage,
};

pub struct OptionsScene {
    master_volume: f32,
    music_volume: f32,
    sound_effects_volume: f32,
}

impl OptionsScene {
    const NORMAL_FONT_SIZE_PX: f32 = 22f32;

    pub fn new() -> Self {
        Self {
            master_volume: 50f32,
            music_volume: 50f32,
            sound_effects_volume: 50f32,
        }
    }

    fn named_slider<F, T>(
        name: &str,
        range: RangeInclusive<T>,
        value: T,
        on_change: F,
    ) -> BaseWidget<UiMessage>
    where
        F: Fn(&T) -> UiMessage + 'static,
        T: 'static + From<f32> + std::cmp::PartialEq + std::fmt::Display + Copy,
        f32: From<T>,
    {
        let text = Text::new().push_segment(TextSegment::new(name, Colour::WHITE));

        let fill_bar = FillBar::new(range, value, true, on_change);
        let bar_container: BaseWidget<UiMessage> = BaseWidget::new()
            .with_span(Span::ParentWeight(3f32))
            .push(fill_bar);

        BaseWidget::new()
            .with_span(Span::ViewHeight(0.025f32))
            .with_axis(Axis::Horizontal)
            .push(
                BaseWidget::new()
                    .with_span(Span::ParentWeight(1f32))
                    .with_text(text),
            )
            .push(bar_container)
    }
}
impl Scene for OptionsScene {
    type Message = UiMessage;

    fn handle_message(&mut self, message: Self::Message) -> (Option<Self::Message>, bool) {
        match message {
            UiMessage::OptionsMenuMessage(options_menu_message) => match options_menu_message {
                OptionsMenuMessage::BackClicked => {
                    info!("Back clicked!");
                    (
                        Some(UiMessage::GoToScene(SceneIdentifier::StartMenu)),
                        false,
                    )
                }
                OptionsMenuMessage::MasterVolumeChanged(val) => {
                    self.master_volume = val;
                    (None, false)
                }
                OptionsMenuMessage::MusicVolumeChanged(val) => {
                    self.music_volume = val;
                    (None, false)
                }
                OptionsMenuMessage::SoundEffectsVolumeChanged(val) => {
                    self.sound_effects_volume = val;
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
                                    .push_segment(TextSegment::new("Options Menu", Colour::WHITE))
                                    .with_configuration(TextConfiguration {
                                        size: TextSize::Pixels(Self::NORMAL_FONT_SIZE_PX * 2f32),
                                        ..Default::default()
                                    }),
                            ),
                    )
                    .push(
                        BaseWidget::new()
                            .with_span(Span::ParentWeight(10f32))
                            .with_text(
                                Text::new()
                                    .push_segment(TextSegment::new(
                                        "This is my text! ",
                                        Colour::WHITE,
                                    ))
                                    .push_segment(TextSegment::new(
                                        "This is another part of my text! ",
                                        Colour::rgb(0f32, 1f32, 1f32),
                                    ))
                                    .push_segment(TextSegment::new(
                                        "This is some more text ",
                                        Colour::WHITE,
                                    ))
                                    .push_segment(TextSegment::new("This is my ", Colour::WHITE))
                                    .push_segment(TextSegment::new(
                                        "FAVOURITE ",
                                        Colour::rgb(0.7f32, 1f32, 0.7f32),
                                    ))
                                    .push_segment(TextSegment::new("text!", Colour::WHITE))
                                    .push_segment(TextSegment::new(
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
                                        size: TextSize::Pixels(Self::NORMAL_FONT_SIZE_PX),
                                        ..Default::default()
                                    }),
                            ),
                    )
                    .push(Self::named_slider(
                        "Master Volume",
                        0f32..=100f32,
                        self.master_volume,
                        |val| {
                            UiMessage::OptionsMenuMessage(OptionsMenuMessage::MasterVolumeChanged(
                                *val,
                            ))
                        },
                    ))
                    .push(Self::named_slider(
                        "Music Volume",
                        0f32..=100f32,
                        self.music_volume,
                        |val| {
                            UiMessage::OptionsMenuMessage(OptionsMenuMessage::MusicVolumeChanged(
                                *val,
                            ))
                        },
                    ))
                    .push(Self::named_slider(
                        "Sound Effects",
                        0f32..=100f32,
                        self.sound_effects_volume,
                        |val| {
                            UiMessage::OptionsMenuMessage(
                                OptionsMenuMessage::SoundEffectsVolumeChanged(*val),
                            )
                        },
                    )),
                // .push(
                //     FillBar::new(0f32..=100f32, 20f32, false, |val| {
                //         UiMessage::OptionsMenuMessage(OptionsMenuMessage::BarChanged(*val))
                //     })
                //     .with_span(Span::ViewHeight(0.05f32)),
                // )
                // .push(
                //     FillBar::new(0f32..=100f32, 20f32, false, |val| {
                //         UiMessage::OptionsMenuMessage(OptionsMenuMessage::BarChanged(*val))
                //     })
                //     .with_span(Span::ViewHeight(0.05f32)),
                // )
                // .push(
                //     FillBar::new(0f32..=100f32, 20f32, false, |val| {
                //         UiMessage::OptionsMenuMessage(OptionsMenuMessage::BarChanged(*val))
                //     })
                //     .with_span(Span::ViewHeight(0.05f32)),
                // ),
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
                    Text::new().push_segment(TextSegment::new("Back to Start", Colour::WHITE)),
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
