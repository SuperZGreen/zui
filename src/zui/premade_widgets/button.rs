use crate::zui::{Colour, BaseWidget};

pub struct Button {}

impl Button {
    
    const COLOUR_CURSOR_OFF: Colour = Colour { r: 0.2f32, g: 0.3f32, b: 0.3f32, a: 1f32 };
    const COLOUR_CURSOR_ON: Colour = Colour { r: 0.2f32, g: 0.6f32, b: 0.6f32, a: 1f32 };
    
    pub fn new<Message>(message: Message) -> BaseWidget<Message>
    where
        Message: Copy,
    {
        BaseWidget::new()
            .with_callback_cursor_on(Some(cursor_on_behaviour))
            .with_callback_cursor_off(Some(cursor_off_behaviour))
            .with_background(Some(Self::COLOUR_CURSOR_OFF))
            .with_message_clicked(Some(message))
    }
}

fn cursor_on_behaviour<Message>(widget: &mut BaseWidget<Message>) -> bool
where
    Message: Copy,
{
    widget.background = Some(Button::COLOUR_CURSOR_ON);
    true
}

fn cursor_off_behaviour<Message>(widget: &mut BaseWidget<Message>) -> bool
where
    Message: Copy,
{
    widget.background = Some(Button::COLOUR_CURSOR_OFF);
    true
}