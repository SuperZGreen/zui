struct Element<Message> {
    name: String,
    message: Message,
}

pub struct SelectList<Message>
where
    Message: Clone + Copy,
{
    elements: Vec<Element>,
}

impl SelectList {
    pub fn new(text_size_px: u32) -> Self {
        Self {
            text_size_px: u32,
        }
    }

    pub fn with_selected(mut self) -> Self {

    }
}
