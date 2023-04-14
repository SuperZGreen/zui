use crate::zui::{Axis, Colour, Scene, Span, Widget};

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
    ButtonClicked,
    ButtonCursorOn,
    ButtonCursorOff,
}

impl Scene for MainScene {
    type Message = Message;

    fn handle_message(&mut self, message: Self::Message) -> bool {
        let mut rebuild_required = false;

        match message {
            Message::ButtonClicked => info!("button was clicked!"),
            Message::ButtonCursorOn => {
                self.cursor_over_button = true;
                self.button_colour = Self::BUTTON_ON_COLOUR;
                rebuild_required = true;
                info!("mouse is over button!");
            }
            Message::ButtonCursorOff => {
                self.cursor_over_button = false;
                self.button_colour = Self::BUTTON_OFF_COLOUR;
                rebuild_required = true;
                info!("mouse is not over button!");
            }
        }

        rebuild_required
    }

    fn view(&self) -> crate::zui::Widget<Message> {
        let button = Widget::new()
            .with_span(Span::ViewHeight(0.2f32))
            .with_on_click(Some(Message::ButtonClicked))
            .with_on_cursor_on(Some(Message::ButtonCursorOn))
            .with_on_cursor_off(Some(Message::ButtonCursorOff))
            .with_background(Some(self.button_colour));

        Widget::new()
            .with_axis(Axis::Horizontal)
            .push(Widget::new())
            .push(
                Widget::new()
                    .with_axis(Axis::Vertical)
                    .with_span(Span::ViewMin(1f32))
                    .push(Widget::new())
                    .push(button)
                    .push(Widget::new()),
            )
            .push(Widget::new())
    }
}
