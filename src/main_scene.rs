use crate::zui::{premade_widgets::Button, Axis, Colour, Scene, Span, Widget};

pub struct MainScene {
    cursor_over_button: bool,
    button_colour: Colour,
}

impl MainScene {
    const BUTTON_OFF_COLOUR: Colour = Colour {
        r: 0.1f32,
        g: 0.1f32,
        b: 0.6f32,
        a: 1f32,
    };
    const BUTTON_ON_COLOUR: Colour = Colour {
        r: 0.3f32,
        g: 0.6f32,
        b: 0.3f32,
        a: 1f32,
    };

    pub fn new() -> Self {
        Self {
            cursor_over_button: false,
            button_colour: Self::BUTTON_OFF_COLOUR,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Message {
    StartClicked,
    OptionsClicked,
    ExitClicked,
}

impl Scene for MainScene {
    type Message = Message;

    fn handle_message(&mut self, message: Self::Message) -> bool {
        let mut rebuild_required = false;

        match message {
            Message::StartClicked => {
                info!("Start clicked!");
            }
            Message::OptionsClicked => {
                info!("Options clicked!");
            }
            Message::ExitClicked => {
                info!("Exit clicked!");
            }
        }

        rebuild_required
    }

    fn view(&self) -> crate::zui::Widget<Message> {
        // let button = Widget::new()
        //     .with_span(Span::ViewHeight(0.2f32))
        //     .with_message_clicked(Some(Message::ButtonClicked))
        //     .with_message_cursor_on(Some(Message::ButtonCursorOn))
        //     .with_message_cursor_off(Some(Message::ButtonCursorOff))
        //     .with_background(Some(self.button_colour));

        Widget::new()
            .with_axis(Axis::Horizontal)
            .push(Widget::new())
            .push(
                Widget::new()
                    .with_axis(Axis::Vertical)
                    .with_span(Span::ViewMin(1f32))
                    .push(Widget::new())
                    .push(
                        Widget::new()
                            .with_span(Span::ParentWeight(3f32))
                            .with_background(Some(Colour::rgb(0.2f32, 0.6f32, 0.1f32))),
                    )
                    .push(Widget::new())
                    .push(Button::new(Message::StartClicked).with_span(Span::ParentWeight(2f32)))
                    .push(Widget::new())
                    .push(Button::new(Message::OptionsClicked).with_span(Span::ParentWeight(2f32)))
                    .push(Widget::new())
                    .push(Button::new(Message::ExitClicked).with_span(Span::ParentWeight(2f32)))
                    .push(Widget::new()),
            )
            .push(Widget::new())
    }
}
