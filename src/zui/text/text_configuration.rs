#[derive(Clone)]
/// Holds information describing how to form the layout of the [`Text`]
pub struct TextConfiguration {
    pub line_wrapping: LineWrapping,
    pub size: TextSize,
    pub horizontal_alignment: TextAlignmentHorizontal,
    pub vertical_alignment: TextAlignmentVertical,
}

impl Default for TextConfiguration {
    fn default() -> Self {
        Self {
            line_wrapping: LineWrapping::Symbol,
            size: TextSize::Pixels(32u32),
            horizontal_alignment: TextAlignmentHorizontal::Left,
            vertical_alignment: TextAlignmentVertical::Top,
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
/// Determines the line wrapping behaviour of [`Text`]
pub enum LineWrapping {
    None,
    Symbol,
    Word,
}

#[derive(Clone)]
/// Used to derive height of the text, ascent to descent
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

#[derive(Copy, Clone)]
/// Determines the horizontal alignment of the [`Text`]
pub enum TextAlignmentHorizontal {
    Left,
    Centre,
    Right,
}

#[derive(Copy, Clone)]
/// Determinse the vertical alignment of the [`Text`]
pub enum TextAlignmentVertical {
    Top,
    Centre,
    Bottom,
}
