use crate::{Text, TextConfiguration, TextSegment};

/// Used to create a Text struct via Text::from<TextDescriptor>.
pub struct TextDescriptor {
    /// The TextSegments that make up the larger text block.
    pub segments: Vec<TextSegment>,

    /// The configuration settings for the Text.
    pub configuration: TextConfiguration,
}

impl Default for TextDescriptor {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            configuration: TextConfiguration::default(),
        }
    }
}

impl From<TextDescriptor> for Text {
    fn from(text_descriptor: TextDescriptor) -> Self {
        Self  {
            segments: text_descriptor.segments,
            symbols: Vec::new(),
            configuration: text_descriptor.configuration,
            layout: None,
        }
    }
}
