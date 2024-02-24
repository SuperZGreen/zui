/// Holds information describing how to form the layout of the [`Text`]
#[derive(Clone)]
pub struct TextConfiguration {
    pub line_wrapping: LineWrapping,
    pub size_px: u32,
    pub horizontal_alignment: TextAlignmentHorizontal,
    pub vertical_alignment: TextAlignmentVertical,
}

impl Default for TextConfiguration {
    fn default() -> Self {
        Self {
            line_wrapping: LineWrapping::Symbol,
            size_px: 32u32,
            horizontal_alignment: TextAlignmentHorizontal::Left,
            vertical_alignment: TextAlignmentVertical::Top,
        }
    }
}

/// Determines the line wrapping behaviour of [`Text`]
#[derive(Clone)]
#[allow(dead_code)]
pub enum LineWrapping {
    None,
    Symbol,
    Word,
}

/// Used to derive height of the text, ascent to descent
#[derive(Clone)]
pub enum TextSize {
    /// Height of the text (ascent to descent) Proportion of the parent's rectangle height
    ParentHeight(f32),

    /// Height of the text in pixels
    Pixels(u32),
}

impl TextSize {
    /// Gives the text height in terms of screen space
    pub fn to_viewport_pixels(&self, parent_rectangle_height: f32) -> i32 {
        match self {
            TextSize::ParentHeight(proportion) => (proportion * parent_rectangle_height) as i32,
            TextSize::Pixels(pixels) => *pixels as i32,
        }
    }
}

/// Determines the horizontal alignment of the [`Text`]
#[derive(Copy, Clone)]
pub enum TextAlignmentHorizontal {
    Left,
    Centre,
    Right,
}

/// Determinse the vertical alignment of the [`Text`]
#[derive(Copy, Clone)]
pub enum TextAlignmentVertical {
    Top,
    Centre,
    Bottom,
}
