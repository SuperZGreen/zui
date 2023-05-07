use std::collections::VecDeque;

use crate::zui::{premade_widgets::Button, Axis, Colour, Scene, Span, Widget};

pub struct MainScene {
    external_messages: VecDeque<Message>
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            external_messages: VecDeque::new(),
        }
    }
    
    pub fn pop_external_message(&mut self) -> Option<Message> {
        self.external_messages.pop_front()
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
        let rebuild_required = false;

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
        
        self.external_messages.push_back(message);

        rebuild_required
    }

    fn view(&self, aspect_ratio: f32) -> crate::zui::Widget<Message> {
        // let button = Widget::new()
        //     .with_span(Span::ViewHeight(0.2f32))
        //     .with_message_clicked(Some(Message::ButtonClicked))
        //     .with_message_cursor_on(Some(Message::ButtonCursorOn))
        //     .with_message_cursor_off(Some(Message::ButtonCursorOff))
        //     .with_background(Some(self.button_colour));

        let central_content = Widget::new()
            .with_axis(Axis::Vertical)
            .push(Widget::new())
            .push(
                Widget::new()
                    .with_span(Span::ParentWeight(3f32))
                    .with_background(Some(Colour::rgb(0.2f32, 0.6f32, 0.1f32)))
                    .with_text("Hello again! :^) This is a very long line that is very, very long and will need a lot of space to render correctly which is unfortunate because text wrapping might not yet be implemented properly"),
            )
            .push(Widget::new())
            .push(
                Button::new(Message::StartClicked)
                    .with_span(Span::ParentWeight(2f32))
                    .with_text("Start"),
            )
            .push(Widget::new())
            .push(
                Button::new(Message::OptionsClicked)
                    .with_span(Span::ParentWeight(2f32))
                    .with_text("Options"),
            )
            .push(Widget::new())
            .push(
                Button::new(Message::ExitClicked)
                    .with_span(Span::ParentWeight(2f32))
                    .with_text("Exit"),
            )
            .push(Widget::new());
        
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
        Widget::new()
            .with_axis(Axis::Horizontal)
            .push(Widget::new())
            .push(central_content.with_span(central_container_span))
            .push(Widget::new())
    }
}
