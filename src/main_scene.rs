use crate::{
    zui::{
        premade_widgets::{Container, TextContainer},
        Axis, Colour, Scene, Span, Text, TextSegment, Widget,
    },
    UiMessage,
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

    fn view(&self, _aspect_ratio: f32) -> Box<dyn Widget<Self::Message>> {
        let v1 = Container::new()
            .with_name("v1")
            .with_span(Span::Pixels(64f32))
            .with_background(Some(Colour::RED));
        let v2 = Container::new()
            .with_name("v2")
            .with_span(Span::Pixels(128f32))
            .with_background(Some(Colour::YELLOW));
        let v3 = Container::new()
            .with_name("v3")
            .with_span(Span::Pixels(192f32))
            .with_background(Some(Colour::GREEN));
        let v4_text = TextContainer::new()
            .with_text(Text::new().push_segment(TextSegment::new("Hello!", Colour::WHITE)))
            .with_background_colour(Some(Colour::DARK_CYAN));

        let h1 = Container::new()
            .with_name("h1")
            .with_span(Span::Pixels(64f32))
            .with_background(Some(Colour::CYAN));
        let h2_text = TextContainer::new()
            .with_text(
                Text::new().push_segment(TextSegment::new(
                    "This is the h2 text, it goes and goes and keeps on going on and on and on",
                    // "This is the h2 text",
                    Colour::ORANGE
            )));
        let h2 = Container::new()
            .with_name("h2")
            .with_span(Span::Pixels(128f32))
            .with_background(Some(Colour::BLUE))
            .push(h2_text);
        let h3 = Container::new()
            .with_name("h3")
            .with_span(Span::Pixels(192f32))
            .with_background(Some(Colour::MAGENTA));
        let h4 = Container::new()
            .with_name("h4")
            .with_span(Span::Pixels(64f32))
            .with_background(Some(Colour::CYAN));
        let h5 = Container::new()
            .with_name("h5")
            .with_span(Span::Pixels(128f32))
            .with_background(Some(Colour::BLUE));

        let row = Container::new()
            .with_name("row")
            .with_background(Some(Colour::LIGHT_GREY))
            .with_axis(Axis::Horizontal)
            .with_span(Span::FitContents)
            .push(h1)
            .push(h2)
            .push(h3)
            .push(h4)
            .push(h5);

        let th1 = TextContainer::new()
            .with_text(
                Text::new().push_segment(TextSegment::new(" This is text 1!  ", Colour::BLACK)),
            )
            .with_background_colour(Some(Colour::LIGHT_RED));
        let th2 = TextContainer::new()
            .with_text(Text::new().push_segment(TextSegment::new(
                " This is the second text!  ",
                Colour::BLACK,
            )))
            .with_background_colour(Some(Colour::LIGHT_GREEN));
        let th3 = TextContainer::new()
            .with_text(Text::new().push_segment(TextSegment::new(
                " This is the third text!   ",
                Colour::BLACK,
            )))
            .with_background_colour(Some(Colour::LIGHT_BLUE));

        let text_row = Container::new()
            .with_name("text_row")
            .with_background(Some(Colour::LIGHT_GREY))
            .with_axis(Axis::Horizontal)
            .with_span(Span::FitContents)
            .push(th1)
            .push(th2)
            .push(th3);

        let expandable_child_1 = Container::new()
            .with_name("expandable_child_1")
            .with_span(Span::Pixels(128f32))
            .with_background(Some(Colour::DARK_RED));
        let expandable_child_2 = Container::new()
            .with_name("expandable_child_2")
            .with_span(Span::Pixels(256f32))
            .with_background(Some(Colour::DARK_GREEN));
        let expandable_child_3 = Container::new()
            .with_name("expandable_child_3")
            .with_span(Span::Pixels(64f32))
            .with_background(Some(Colour::DARK_BLUE));

        let expandable_1 = Container::new()
            .with_name("expandable_1")
            .with_span(Span::ParentWeight(1f32))
            .with_background(Some(Colour::YELLOW))
            .push(expandable_child_1)
            .push(expandable_child_2)
            .push(expandable_child_3);

        let expandable_2_1 = Container::new()
            .with_name("expandable_2_1")
            .with_span(Span::ParentWeight(1f32))
            .with_background(Some(Colour::WHITE));
        let expandable_2_2_text = TextContainer::new()
            .with_text(Text::new().push_segment(TextSegment::new(
                "This is my long test text for the expandable_2_2, I want to make it pretty long \
                so that I can see it wrap around for a bit. I think that might be pretty useful to \
                check out my formatting.",
                Colour::LIGHT_BLUE,
            )));
        let expandable_2_2 = Container::new()
            .with_name("expandable_2_2")
            .with_span(Span::ParentWeight(2f32))
            .with_background(Some(Colour::BLACK))
            .push(expandable_2_2_text);
        let expandable_2_3 = Container::new()
            .with_name("expandable_2_3")
            .with_span(Span::ParentWeight(1f32))
            .with_background(Some(Colour::GREY));

        let expandable_2 = Container::new()
            .with_name("expandable_2")
            .with_axis(Axis::Horizontal)
            .with_span(Span::ParentWeight(1f32))
            .with_background(Some(Colour::MAGENTA))
            .push(expandable_2_1)
            .push(expandable_2_2)
            .push(expandable_2_3);


        Container::new()
            .with_name("root_widget")
            .with_span(Span::ParentRatio(1f32))
            .with_background(Some(Colour::DARK_GREY))
            .push(v1)
            .push(v2)
            .push(row)
            .push(v3)
            .push(v4_text)
            .push(text_row)
            .push(expandable_1)
            .push(expandable_2)
            .into()
    }

    // fn view(&self, aspect_ratio: f32) -> Box<dyn Widget<Self::Message>> {
    //     let button_off_colour = Colour::rgb(0.2f32, 0.3f32, 0.4f32);
    //     let button_on_colour = Colour::rgb(0.3f32, 0.4f32, 0.6f32);

    //     let button_text_configuration = TextConfiguration {
    //         size_px: 64,
    //         horizontal_alignment: TextAlignmentHorizontal::Centre,
    //         vertical_alignment: TextAlignmentVertical::Centre,
    //         ..Default::default()
    //     };

    //     let central_content = Container::new()
    //         .with_axis(Axis::Vertical)
    //         .with_background(Some(Colour::rgb(1f32, 0f32, 0.5f32)))
    //         .push(Container::new())
    //         .push(
    //             Container::new()
    //                 .with_span(Span::ParentWeight(3f32))
    //                 .with_background(Some(Colour::rgb(0.2f32, 0.6f32, 0.1f32)))
    //                 .with_text(
    //                     Text::new()
    //                         .with_configuration(TextConfiguration {
    //                             size_px: 64,
    //                             vertical_alignment: TextAlignmentVertical::Bottom,
    //                             horizontal_alignment: TextAlignmentHorizontal::Right,
    //                             ..Default::default()
    //                         })
    //                         .push_segment(TextSegment::new("Welcome! :^)", Colour::WHITE)),
    //                 ),
    //         )
    //         .push(Container::new())
    //         .push(
    //             Button::new(
    //                 UiMessage::GoToScene(SceneIdentifier::GameScene),
    //                 button_off_colour,
    //                 button_on_colour,
    //             )
    //             .with_span(Span::ParentWeight(2f32))
    //             .with_text(
    //                 Text::new()
    //                     .with_configuration(button_text_configuration.clone())
    //                     .push_segment(TextSegment::new("Start", Colour::WHITE)),
    //             ),
    //         )
    //         .push(Container::new())
    //         .push(
    //             Button::new(
    //                 UiMessage::GoToScene(SceneIdentifier::OptionsMenu),
    //                 button_off_colour,
    //                 button_on_colour,
    //             )
    //             .with_span(Span::ParentWeight(2f32))
    //             .with_text(
    //                 Text::new()
    //                     .with_configuration(button_text_configuration.clone())
    //                     .push_segment(TextSegment::new("Tests/Options", Colour::WHITE)),
    //             ),
    //         )
    //         .push(Container::new())
    //         .push(
    //             Button::new(UiMessage::Exit, button_off_colour, button_on_colour)
    //                 .with_span(Span::ParentWeight(2f32))
    //                 .with_text(
    //                     Text::new()
    //                         .with_configuration(button_text_configuration.clone())
    //                         .push_segment(TextSegment::new("Exit", Colour::WHITE)),
    //                 ),
    //         )
    //         .push(Container::new());

    //     // let central_container_span = if aspect_ratio <= 1.1f32 {
    //     //     Span::ParentWeight(20f32)
    //     // } else {
    //     //     Span::ViewMin(1f32)
    //     // };
    //     let central_container_span = Span::ViewMin(1f32);

    //     // let central_container = Widget::new()
    //     //     .with_axis(Axis::Horizontal)
    //     //     .with_span(Span::ViewMin(1f32))
    //     //     .push(Widget::new().with_span(Span::ParentWeight(1f32)))
    //     //     .push(central_content.with_span(central_container_span))
    //     //     .push(Widget::new().with_span(Span::ParentWeight(1f32)));

    //     // root widget
    //     Container::new()
    //         .with_background(Some(Colour::rgb(0.1f32, 0.1f32, 0.1f32)))
    //         .with_axis(Axis::Horizontal)
    //         .push(Container::new())
    //         .push(central_content.with_span(central_container_span))
    //         .push(Container::new())
    //         .into()
    // }
}
