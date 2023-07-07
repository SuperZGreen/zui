use zui::{
    premade_widgets::{Button, Container, TextContainer},
    typeface::FontStyle,
    Axis, Colour, Scene, Span, Text, TextConfiguration, TextSegment, Widget, StateStore,
};

use crate::UiMessage;

pub struct MainScene {
    count: u64,
}

impl MainScene {
    pub fn new() -> Self {
        Self { count: 0u64 }
    }
}

#[derive(PartialEq, Eq, std::hash::Hash, std::fmt::Debug, Copy, Clone)]
pub enum Identifier {
    ResetCounterButton,
    HelloTextContainer,
    H2TextContainer,
    Th1TextContainer,
    Th2TextContainer,
    Th3TextContainer,
    Ex2_2TextContainer,
    FishTextContainer,
    MoneyTextContainer,
    PeopleTextContainer,
}

impl Scene for MainScene {
    type Message = UiMessage;
    type StateIdentifier = Identifier;

    fn handle_message(&mut self, message: Self::Message) -> (Option<Self::Message>, bool) {
        match message {
            UiMessage::SetCounter(count) => {
                self.count = count;
                (None, true)
            }
            UiMessage::IncrementCounter(increment) => {
                self.count += increment;
                (None, true)
            }
            _ => (Some(message), false),
        }
    }

    fn view(&self, state_store: &mut StateStore<Self::StateIdentifier>, _aspect_ratio: f32) -> Box<dyn Widget<Self::Message, Self::StateIdentifier>> {
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
        let v4_text = TextContainer::new(Identifier::HelloTextContainer)
            .with_text(Text::new().push_segment(TextSegment::new("Hello!", Colour::WHITE)))
            .with_background_colour(Some(Colour::DARK_CYAN));

        let h1 = Container::new()
            .with_name("h1")
            .with_span(Span::Pixels(64f32))
            .with_background(Some(Colour::CYAN));
        let h2_text = TextContainer::new(Identifier::H2TextContainer).with_text(
            Text::new().push_segment(TextSegment::new(
                "This is the h2 text, it goes and goes and keeps on going on and on and on",
                // "This is the h2 text",
                Colour::ORANGE,
            )),
        );
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

        let th1 = TextContainer::new(Identifier::Th1TextContainer)
            .with_text(
                Text::new().push_segment(TextSegment::new(" This is text 1!  ", Colour::BLACK)),
            )
            .with_background_colour(Some(Colour::LIGHT_RED));
        let th2 = TextContainer::new(Identifier::Th2TextContainer)
            .with_text(
                Text::new()
                    .with_configuration(TextConfiguration {
                        size_px: 18,
                        ..Default::default()
                    })
                    .push_segment(TextSegment {
                        string: String::from(" This is the second text!  "),
                        colour: Colour::BLACK,
                        style: FontStyle::Bold,
                    }),
            )
            .with_background_colour(Some(Colour::LIGHT_GREEN));
        let th3 = TextContainer::new(Identifier::Th3TextContainer)
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
        let expandable_2_2_text =
            TextContainer::new(Identifier::Ex2_2TextContainer)
                .with_text(Text::new().push_segment(TextSegment::new(
                "This is my long test text for the expandable_2_2, I want to make it pretty long \
                so that I can see it wrap around for a bit. I think that might be pretty useful to \
                check out my formatting.",
                Colour::LIGHT_BLUE,
            ))
            .push_segment(TextSegment::new(" \u{f023a}", Colour::CYAN))
            .push_segment(TextSegment::new(&format!(" {}", self.count), Colour::WHITE))
            .with_configuration(TextConfiguration {
                line_wrapping: zui::LineWrapping::Word,
                ..Default::default()
            })
        );
        let fish_text = TextContainer::new(Identifier::FishTextContainer).with_text(
            Text::new()
                .push_segment(TextSegment::new(" \u{f023a}", Colour::CYAN))
                .push_segment(TextSegment::new(&format!(" {}", self.count), Colour::WHITE)),
        );
        let money_text = TextContainer::new(Identifier::MoneyTextContainer).with_text(
            Text::new()
                .push_segment(TextSegment::new(" \u{f0d6}", Colour::YELLOW))
                .push_segment(TextSegment::new(&format!(" {}", self.count), Colour::WHITE)),
        );
        let people_text = TextContainer::new(Identifier::PeopleTextContainer).with_text(
            Text::new()
                .push_segment(TextSegment::new(" \u{f4fd}", Colour::ORANGE))
                .push_segment(TextSegment::new(&format!(" {}", self.count), Colour::WHITE)),
        );
        let expandable_2_2 = Container::new()
            .with_name("expandable_2_2")
            .with_span(Span::ParentWeight(2f32))
            .with_background(Some(Colour::BLACK))
            .push(expandable_2_2_text)
            .push(fish_text)
            .push(money_text)
            .push(people_text);

        let expandable_2_3_button = Button::new(
            UiMessage::SetCounter(0),
            Colour::DARK_BLUE,
            Colour::DARK_CYAN,
            state_store,
            Identifier::ResetCounterButton,
        )
        .with_text(Text::new().push_segment(TextSegment::new("Reset counter", Colour::WHITE)));

        let expandable_2_3 = Container::new()
            .with_name("expandable_2_3")
            .with_span(Span::ParentWeight(1f32))
            .with_background(Some(Colour::GREY))
            .push(expandable_2_3_button);

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
}
