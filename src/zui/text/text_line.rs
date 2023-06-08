use std::ops::{Range, RangeInclusive};

use crate::zui::Rectangle;

use super::{text_configuration::LineWrapping, GlyphOrigin, PixelFontMetrics, Presymbol};

/// Used to calculate the width of a word or group of Text
struct TextWidthAccumulator {
    /// The advance width NOTE: from THE EDGE of the previously pushed symbol's width
    previous_symbol_advance_width_px: i32,

    /// The current width of the symbols held in the line
    width_px: i32,
}

impl TextWidthAccumulator {
    /// Creates a new empty TextWidthAccumulator
    pub fn new() -> Self {
        Self {
            previous_symbol_advance_width_px: 0i32,
            width_px: 0i32,
        }
    }

    /// Pushes a presymbol to the TextWidthAccumulator, adds the new width
    pub fn push(&mut self, presymbol: &Presymbol) {
        let symbol_metrics = &presymbol.symbol_info.symbol_metrics;

        // The amount of horizontal pixels that the new symbol will take up past the previous
        // symbol's maximum
        let additional_width_px =
            self.previous_symbol_advance_width_px + symbol_metrics.x_shift + symbol_metrics.width;

        self.width_px += additional_width_px;
        self.previous_symbol_advance_width_px =
            symbol_metrics.advance_width - (symbol_metrics.x_shift + symbol_metrics.width);
    }

    /// Gives the current width of the text group, in pixels
    pub fn width_px(&self) -> i32 {
        self.width_px
    }
}

/// Used to build a TextLine
pub struct TextLineBuilder {
    /// The starting index of the text line presymbol
    start_index: usize,

    /// The number of symbols that have been added to the TextLineBuilder
    num_symbols: u32,

    /// The advance width NOTE: from THE EDGE of the previously pushed symbol's width
    previous_symbol_advance_width_px: i32,

    /// The current width of the symbols held in the line
    width_px: i32,

    /// The width of the parent clip rectangle container
    clip_rectangle_width_px: i32,
}

/// An error from attempting to push symbols into a TextLineBuilder
pub enum TextLineBuilderError {
    /// The symbol will not fit on the end of the current line, and a new line should be created
    CreateNewLine,

    /// The pushed symbol(s) will never fit into the clipping rectangle, as its width is larger than
    /// the width of the clip rectangle
    WidthError,
}

impl TextLineBuilder {
    /// Creates a new TextLineBuilder from the clip rectangle of the text
    pub fn new(start_index: usize, clip_rectangle: &Rectangle<f32>) -> Self {
        Self {
            start_index,
            num_symbols: 0,
            previous_symbol_advance_width_px: 0,
            width_px: 0,
            // The round here removes the jitter when fitting to a parent container, TODO: this
            // could be moved up into the calling context
            clip_rectangle_width_px: clip_rectangle.width().round() as i32,
        }
    }

    /// Tries to fit the word into the line
    pub fn try_push_slice(&mut self, word: &[Presymbol]) -> Result<(), TextLineBuilderError> {
        // Calculating the width of the word
        let mut word_width_accumulator = TextWidthAccumulator::new();
        for presymbol in word {
            word_width_accumulator.push(presymbol);
        }
        let word_width_px = word_width_accumulator.width_px();
        let additional_width = self.previous_symbol_advance_width_px + word_width_px;

        if self.width_px + additional_width >= self.clip_rectangle_width_px {
            if self.clip_rectangle_width_px > word_width_px {
                Err(TextLineBuilderError::CreateNewLine)
            } else {
                Err(TextLineBuilderError::WidthError)
            }
        } else {
            self.num_symbols += word.len() as u32;
            self.width_px += additional_width;
            self.previous_symbol_advance_width_px = match word.last() {
                Some(p) => {
                    let last_symbol_metrics = &p.symbol_info.symbol_metrics;
                    last_symbol_metrics.advance_width
                        - (last_symbol_metrics.x_shift + last_symbol_metrics.width)
                }
                None => 0,
            };
            Ok(())
        }
    }

    /// Tries to fit a symbol into the text line, given an x_max value. Will return Err if the
    /// symbol does not fit
    pub fn try_push_symbol(&mut self, presymbol: &Presymbol) -> Result<(), TextLineBuilderError> {
        let symbol_metrics = presymbol.symbol_info.symbol_metrics;

        // The amount of horizontal pixels that the new symbol will take up past the previous
        // symbol's maximum
        let additional_width_px =
            self.previous_symbol_advance_width_px + symbol_metrics.x_shift + symbol_metrics.width;

        if self.width_px + additional_width_px <= self.clip_rectangle_width_px {
            self.force_push_symbol(presymbol);

            Ok(())
        } else {
            if symbol_metrics.width + symbol_metrics.x_shift > self.clip_rectangle_width_px {
                Err(TextLineBuilderError::WidthError)
            } else {
                Err(TextLineBuilderError::CreateNewLine)
            }
        }
    }

    /// Pushes a symbol into the line without checking that it fits within the clipping rectangle,
    /// can cause clipping of text!
    pub fn force_push_symbol(&mut self, presymbol: &Presymbol) {
        let symbol_metrics = presymbol.symbol_info.symbol_metrics;
        let additional_width_px =
            self.previous_symbol_advance_width_px + symbol_metrics.x_shift + symbol_metrics.width;

        self.num_symbols += 1;
        self.width_px += additional_width_px;
        self.previous_symbol_advance_width_px =
            symbol_metrics.advance_width - (symbol_metrics.x_shift + symbol_metrics.width);
    }

    /// Creates a TextLine out of the TextLineBuilder
    pub fn build(self, font_metrics_px: &PixelFontMetrics) -> TextLine {
        TextLine {
            range: self.start_index..self.start_index + self.num_symbols as usize,
            viewport_px_dimensions: glam::IVec2::new(self.width_px, font_metrics_px.height),
        }
    }
}

#[derive(Clone)]
/// Represents a line of text as it appears on screen, can contain either Presymbols or Symbols
pub struct TextLine {
    /// The symbols that are part of the line
    pub range: Range<usize>,

    /// The width of the line
    pub viewport_px_dimensions: glam::IVec2,
}

/// A collection of text lines
#[derive(Clone)]
pub struct TextLines {
    pub lines: Vec<TextLine>,
}

impl TextLines {
    pub fn from_presymbols(
        presymbols: &[Presymbol],
        clip_rectangle: &Rectangle<f32>,
        font_metrics_px: &PixelFontMetrics,
        line_wrapping: &LineWrapping,
    ) -> Self {
        let mut lines = Vec::new();

        match line_wrapping {
            LineWrapping::Word => {
                if presymbols.len() > 0 {
                    let mut builder = TextLineBuilder::new(0, clip_rectangle);
                    let mut index = 0usize;

                    for word in Presymbol::words(presymbols).into_iter() {
                        match builder.try_push_slice(word) {
                            Ok(_) => {
                                index += word.len();
                                continue;
                            }
                            Err(TextLineBuilderError::WidthError) => {
                                // pushing the individual letters of the word
                                for presymbol in word {
                                    match builder.try_push_symbol(presymbol) {
                                        Ok(_) => {
                                            index += 1;
                                            continue;
                                        }
                                        Err(TextLineBuilderError::CreateNewLine) => {
                                            lines.push(builder.build(font_metrics_px));
                                            builder = TextLineBuilder::new(index, clip_rectangle);
                                            builder.force_push_symbol(presymbol);
                                            index += 1;
                                        }
                                        Err(TextLineBuilderError::WidthError) => {
                                            builder.force_push_symbol(presymbol);
                                            index += 1;
                                        }
                                    }
                                }
                            }
                            Err(TextLineBuilderError::CreateNewLine) => {
                                lines.push(builder.build(font_metrics_px));
                                builder = TextLineBuilder::new(index, clip_rectangle);
                                _ = builder.try_push_slice(word);
                                index += word.len();
                            }
                        }
                    }

                    // pushing final line
                    lines.push(builder.build(font_metrics_px));
                }
            }
            LineWrapping::Symbol => {
                if presymbols.len() > 0 {
                    let mut builder = TextLineBuilder::new(0, clip_rectangle);

                    for (index, presymbol) in presymbols.iter().enumerate() {
                        match builder.try_push_symbol(presymbol) {
                            Ok(_) => continue,
                            Err(TextLineBuilderError::CreateNewLine) => {
                                lines.push(builder.build(font_metrics_px));
                                builder = TextLineBuilder::new(index, clip_rectangle);

                                // can force push the symbol here, as it is proven to not be too small for
                                // the provided clip rectangle
                                builder.force_push_symbol(presymbol);
                            }
                            Err(TextLineBuilderError::WidthError) => {
                                builder.force_push_symbol(presymbol);

                                lines.push(builder.build(font_metrics_px));
                                builder = TextLineBuilder::new(index, clip_rectangle);
                            }
                        }
                    }

                    // pushing final line
                    lines.push(builder.build(font_metrics_px));
                }
            }
            LineWrapping::None => {
                let mut builder = TextLineBuilder::new(0, clip_rectangle);
                for presymbol in presymbols.iter() {
                    builder.force_push_symbol(presymbol);
                }
                lines.push(builder.build(font_metrics_px));
            }
        }

        Self { lines }
    }

    pub fn viewport_px_dimensions(&self, font_metrics_px: &PixelFontMetrics) -> glam::IVec2 {
        // lines max width
        let mut width = 0i32;
        for line in self.lines.iter() {
            if line.viewport_px_dimensions.x > width {
                width = line.viewport_px_dimensions.x;
            }
        }

        // lines height
        let mut height = 0i32;
        if let Some(first_line) = self.lines.first() {
            height += first_line.viewport_px_dimensions.y;

            for _line in &self.lines[1..self.lines.len()] {
                height += font_metrics_px.new_line_size;
            }
        }

        glam::IVec2::new(width, height)
    }
}
