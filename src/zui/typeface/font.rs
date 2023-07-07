use crate::text::PixelFontMetrics;

use super::{coverage_image::CoverageImage, SymbolMetrics};

/// Represents a single font/ttf file. So seperate Fonts will exist for Regular, Italic and Bold.
pub struct Font {
    /// The underlying fontdue font
    fontdue_font: fontdue::Font,
}

impl Font {
    /// Loads a new font from a file path
    pub fn load(path: &str) -> Option<Self> {
        let fontdue_font = match Self::fontdue_font_from_file(path) {
            Some(ff) => ff,
            None => {
                warn!("failed to load font: {path}");
                return None;
            }
        };

        Some(Self { fontdue_font })
    }

    /// Rasterises the provided character, providing the SymbolMetrics and CoverageImage for the
    /// symbol, if the symbol is not present, returns the information of the font's default
    /// character
    pub fn rasterise(&self, character: char, size_px: u32) -> (SymbolMetrics, CoverageImage) {
        let (metrics, coverage) = self.fontdue_font.rasterize(character, size_px as f32);

        let symbol_coverage = CoverageImage {
            coverage: coverage.into_boxed_slice(),
            width: metrics.width as u32,
            height: metrics.height as u32,
        };

        let symbol_metrics = SymbolMetrics::new(&metrics);

        (symbol_metrics, symbol_coverage)
    }

    /// Loads a fontdue::Font from a file path &str
    fn fontdue_font_from_file(path: &str) -> Option<fontdue::Font> {
        let font_path = std::path::PathBuf::from(path);

        // reading in font file data
        let font_bytes = match std::fs::read(&font_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                error!("Could not load font file: {}", font_path.display());
                return None;
            }
        };

        // decoding font file data to fontdue font
        match fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default()) {
            Ok(f) => Some(f),
            Err(_) => {
                error!("fontdue::Font::from_bytes failed!");
                None
            }
        }
    }

    /// Calculates the metrics for the font at a provided size
    pub fn calculate_metrics(&self, size_px: u32) -> PixelFontMetrics {
        PixelFontMetrics::new(
            &self
                .fontdue_font
                .horizontal_line_metrics(size_px as f32)
                .unwrap(),
        )
    }
}
