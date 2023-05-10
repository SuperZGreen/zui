use crate::{
    zui::{premade_widgets::Button, Axis, Colour, Scene, Span, Widget},
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
                    .with_text("This is the Options Menu Text. As you can see this is a very good options menu with lots of options that you can click on and enjoy. I hope that you enjoy your time in the options menu as I obviously spent quite a bit of time doing it up and making it look nice and everything."),
            )
            .push(Widget::new())
            .push(
                Button::new(UiMessage::OptionsMenuMessage(OptionsMenuMessage::BackClicked))
                    .with_span(Span::ParentWeight(2f32))
                    .with_text("Back to Start Menu"),
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
