use crate::zui::Rectangle;

use super::{PixelFontMetrics, Presymbol, Symbol};

/// The origin accumulator that tracks where to place the next [`Symbol`]
pub struct GlyphOrigin {
    pub viewport_px_position: glam::IVec2,
}

impl GlyphOrigin {
    /// Places the origin at the top left of the rectangle
    pub fn at_top_left(clip_region: &Rectangle<i32>, font_metrics_px: &PixelFontMetrics) -> Self {
        let viewport_px_position = glam::IVec2::new(
            clip_region.x_min as i32,
            clip_region.y_max - font_metrics_px.ascent,
        );

        Self {
            viewport_px_position,
        }
    }

    /// Places the origin at the top left of the rectangle
    pub fn at_centre_left(
        clip_region: &Rectangle<i32>,
        font_metrics_px: &PixelFontMetrics,
    ) -> Self {
        let margin_y = (clip_region.height() - (font_metrics_px.ascent - font_metrics_px.descent))
            / 2i32;
        let origin_position_y = clip_region.y_min + margin_y - font_metrics_px.descent;

        let viewport_px_position =
            glam::IVec2::new(clip_region.x_min as i32, origin_position_y as i32);

        Self {
            viewport_px_position,
        }
    }

    /// Places the origin at the bottom left of the rectangle
    pub fn at_bottom_left(
        clip_region: &Rectangle<i32>,
        font_metrics_px: &PixelFontMetrics,
    ) -> Self {
        let viewport_px_position = glam::IVec2::new(
            clip_region.x_min,
            clip_region.y_min - font_metrics_px.descent,
        );

        Self {
            viewport_px_position,
        }
    }

    /// Moves the GlyphOrigin by the Presymbol, as though placing the symbol
    pub fn increment_by_presymbol(&mut self, presymbol: &Presymbol) {
        self.viewport_px_position.x += presymbol.symbol_info.symbol_metrics.advance_width;
    }

    /// Gives the symbol for a presymbol at the GlyphOrigin's location
    pub fn symbol_from_presymbol(&self, presymbol: &Presymbol) -> Symbol {
        Symbol {
            character: presymbol.character,
            colour: presymbol.colour,
            region: Rectangle::new(
                (self.viewport_px_position.x + presymbol.symbol_info.symbol_metrics.x_shift) as f32,
                (self.viewport_px_position.x
                    + presymbol.symbol_info.symbol_metrics.x_shift
                    + presymbol.symbol_info.symbol_metrics.width) as f32,
                (self.viewport_px_position.y + presymbol.symbol_info.symbol_metrics.y_shift) as f32,
                (self.viewport_px_position.y
                    + presymbol.symbol_info.symbol_metrics.y_shift
                    + presymbol.symbol_info.symbol_metrics.height) as f32,
            ),
            uv_region: presymbol.symbol_info.uv_region,
        }
    }

    /// Resets the glyph origin to the left of the parent rectangle and moves downward for a new
    /// line
    pub fn new_line(&mut self, clip_rectangle: &Rectangle<i32>, font_metrics_px: &PixelFontMetrics) {
        self.viewport_px_position.x = clip_rectangle.x_min;
        self.viewport_px_position.y -= font_metrics_px.new_line_size;
    }
}
