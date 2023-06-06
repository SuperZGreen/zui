use std::ops::{Range, RangeInclusive};

use crate::zui::Rectangle;

use super::{text_configuration::LineWrapping, GlyphOrigin, PixelFontMetrics, Presymbol};

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
            // TODO: check if ceiling is the right thing to do here, in terms of reducing text
            // jitter when moving/resizing a Container
            clip_rectangle_width_px: clip_rectangle.width().ceil() as i32,
        }
    }

    /// Tries to fit a word into the text line, given an x_max value. Will return the number of
    /// characters pushed into the line, fails if the word does not fit
    pub fn try_push_word(
        &mut self,
        presymbols: &[Presymbol],
        start_index: usize,
        clip_rectangle: &Rectangle<f32>,
    ) -> Result<usize, ()> {
        todo!()
    }

    /// Tries to fit a symbol into the text line, given an x_max value. Will return Err if the
    /// symbol does not fit
    pub fn try_push_symbol(&mut self, presymbol: &Presymbol) -> Result<(), TextLineBuilderError> {
        let symbol_metrics = presymbol.symbol_info.symbol_metrics;
        let additional_width_px =
            self.previous_symbol_advance_width_px + symbol_metrics.x_shift + symbol_metrics.width;
        if self.width_px + additional_width_px < self.clip_rectangle_width_px {
            self.num_symbols += 1;
            self.width_px += additional_width_px;

            self.previous_symbol_advance_width_px =
                symbol_metrics.advance_width - (symbol_metrics.x_shift + symbol_metrics.width);

            Ok(())
        } else {
            if symbol_metrics.width > self.clip_rectangle_width_px {
                Err(TextLineBuilderError::WidthError)
            } else {
                Err(TextLineBuilderError::CreateNewLine)
            }
        }
    }

    /// Will force push the symbol to the end of the line, can cause clipping of text!
    pub fn force_push_symbol(&mut self, presymbol: &Presymbol) {
        // TODO
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
        let clip_rect = clip_rectangle.clone();

        let mut lines = Vec::new();

        if presymbols.len() > 0 {
            let mut builder = TextLineBuilder::new(0, clip_rectangle);

            for (index, presymbol) in presymbols.iter().enumerate() {
                match builder.try_push_symbol(presymbol) {
                    Ok(_) => continue,
                    Err(TextLineBuilderError::CreateNewLine) => {
                        lines.push(builder.build(font_metrics_px));
                        builder = TextLineBuilder::new(index, clip_rectangle);
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
