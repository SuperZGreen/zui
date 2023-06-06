use std::ops::RangeInclusive;

use crate::zui::Rectangle;

use super::{text_configuration::LineWrapping, GlyphOrigin, PixelFontMetrics, Presymbol};

#[derive(Clone)]
/// Represents a line of text as it appears on screen, can contain either Presymbols or Symbols
pub struct TextLine {
    /// The symbols that are part of the line
    pub range: RangeInclusive<usize>,

    /// The width of the line
    pub viewport_px_dimensions: glam::IVec2,
}

impl TextLine {
    fn new(range: RangeInclusive<usize>, viewport_px_dimensions: glam::IVec2) -> Self {
        Self {
            range,
            viewport_px_dimensions,
        }
    }

    /// Tries to fit a word into the text line, given an x_max value. Will return the index of the
    /// next word that should be pushed, therefore if the return value is equal to start_index, the
    /// word could not be pushed into the line.
    pub fn try_push_word(
        &mut self,
        presymbols: &[Presymbol],
        start_index: usize,
        clip_rectangle: &Rectangle<f32>,
    ) -> usize {
        todo!()
    }

    /// Tries to fit a symbol into the text line, given an x_max value
    pub fn try_push_symbol(&mut self, presymbols: &[Presymbol], start_index: usize) -> usize {
        todo!()
    }
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
            let mut origin = GlyphOrigin::at_top_left(&clip_rect, font_metrics_px);
            let mut line_start_index = 0usize;
            let mut line_min_x = origin.viewport_px_position.x;
            let mut line_max_x = origin.viewport_px_position.x;

            for (index, presymbol) in presymbols.iter().enumerate() {
                if origin.presymbol_fits_in_rect(&clip_rect, presymbol) {
                    // tracking the new max width of the line
                    line_max_x = origin.viewport_px_position.x
                        + presymbol.symbol_info.symbol_metrics.x_shift
                        + presymbol.symbol_info.symbol_metrics.width;
                } else {
                    // resetting the origin to the start of the line
                    origin = GlyphOrigin::at_top_left(&clip_rect, font_metrics_px);

                    // calculating the new line x min
                    line_min_x = origin.viewport_px_position.x;

                    // pushing the line
                    let line_end_index = index.checked_sub(1).unwrap_or(0);
                    lines.push(TextLine::new(
                        line_start_index..=line_end_index,
                        glam::IVec2::new(line_max_x - line_min_x, font_metrics_px.height),
                    ));
                    line_start_index = index;
                }

                origin.increment_by_presymbol(presymbol);
            }

            // pushing final line
            lines.push(TextLine::new(
                line_start_index..=presymbols.len() - 1,
                glam::IVec2::new(line_max_x - line_min_x, font_metrics_px.height),
            ));
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
