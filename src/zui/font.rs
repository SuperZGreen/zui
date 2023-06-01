use std::path::PathBuf;

use fontdue::LineMetrics;
use image::{DynamicImage, ImageBuffer, Luma};
use rustc_hash::FxHashMap;

use super::{primitives::Rectangle, texture_atlas::TextureAtlas};
use crate::zui::texture_atlas::TextureAtlasBuilder;

/// The identifying information for a character rasterised at a specific size
#[derive(Hash, Eq, PartialEq)]
pub struct SymbolKey {
    /// The character that the symbol represents
    pub character: char,
    /// The font style of the character
    pub font_style: FontStyle,
    /// The size of the font in pixels
    pub size_px: u32,
}

impl SymbolKey {
    pub fn new(character: char, font_style: FontStyle, size_px: u32) -> Self {
        Self {
            character,
            font_style,
            size_px,
        }
    }
}

#[derive(Copy, Clone)]
/// The information used externally to draw the Symbol to the screen
pub struct SymbolInfo {
    /// The pixel metrics of the symbol
    pub symbol_metrics: SymbolMetrics,
    /// The region that the symbol uses on the Typeface's texture atlas, using UV coordinates where
    /// (0, 0) is top left, (1, 1) is bottom right
    pub uv_region: Rectangle<f32>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
/// Describes the style of a typeface
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
}

#[derive(Copy, Clone)]
/// The metrics of a symbol in pixels
pub struct SymbolMetrics {
    /// The width of the symbol
    pub width: i32,
    /// The height of the symbol
    pub height: i32,
    /// The x translation of the symbol from its origin
    pub x_shift: i32,
    /// The y translation of the symbol from its origin
    pub y_shift: i32,
    /// The x movement of the origin after placing the symbol
    pub advance_width: i32,
}

impl SymbolMetrics {
    /// Generates the screen space metrics for a symbol given its pixel metrics generated from
    /// fontdue.
    pub fn new(metrics: &fontdue::Metrics) -> Self {
        Self {
            width: metrics.width as i32,
            height: metrics.height as i32,
            x_shift: metrics.xmin as i32,
            y_shift: metrics.ymin as i32,
            advance_width: metrics.advance_width as i32,
        }
    }
}

/// Contains the fonts and rasterised glyphs for a typeface, useful in rendering text to the screen
pub struct Typeface {
    symbols: FxHashMap<SymbolKey, SymbolInfo>,
    pub line_metrics: LineMetrics,

    /// The TextureAtlas that holds the rasterised glyphs for the font
    pub texture_atlas: TextureAtlas,

    /// The normal font, must exist
    pub font_regular: fontdue::Font,

    /// The bold font, optional
    pub _font_bold: Option<fontdue::Font>,

    /// The bold font, optional
    pub _font_italic: Option<fontdue::Font>,
}

impl Typeface {
    /// Converts fontue's rasterised coverage to a dynamic image containing a Luma (greyscale) value
    fn coverage_to_dynamic_image(coverage: Vec<u8>, width_px: u32, height_px: u32) -> DynamicImage {
        let buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
            match ImageBuffer::from_raw(width_px, height_px, coverage) {
                Some(res) => res,
                None => todo!(),
            };

        DynamicImage::ImageLuma8(buffer)
    }

    /// Loads a fontdue::Font from a file path &str
    fn fontdue_font_from_file(path: &str) -> Option<fontdue::Font> {
        let font_path = PathBuf::from(path);

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

    /// Creates a new Typeface
    pub fn new(
        font_regular_file_path: &str,
        font_bold_file_path: Option<&str>,
        font_italic_file_path: Option<&str>,
        size_px: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, ()> {
        // getting all fonts from files
        let font_regular = Self::fontdue_font_from_file(font_regular_file_path).unwrap();
        let font_bold = font_bold_file_path.map(|fp| Self::fontdue_font_from_file(fp).unwrap());
        let font_italic = font_italic_file_path.map(|fp| Self::fontdue_font_from_file(fp).unwrap());

        let mut texture_atlas_builder = TextureAtlasBuilder::new();
        let mut symbols = FxHashMap::default();

        // rasterising all characters and putting into texture atlas
        let mut rasterisation_info = Vec::with_capacity(font_regular.chars().len());
        for (character, _) in font_regular.chars().iter() {
            let (metrics, coverage) = font_regular.rasterize(*character, size_px as f32);
            let width_px = metrics.width as u32;
            let height_px = metrics.height as u32;
            let image = Self::coverage_to_dynamic_image(coverage, width_px, height_px);

            let texture_atlas_index = texture_atlas_builder.add_sprite(image, width_px, height_px);
            rasterisation_info.push((
                texture_atlas_index,
                *character,
                SymbolMetrics::new(&metrics),
            ))
        }

        let texture_atlas = texture_atlas_builder.build_atlas(device, queue);

        // getting uv coordinates from TextureAtlas for all characters, inserting into symbols
        for (texture_atlas_index, character, symbol_metrics) in rasterisation_info {
            let symbol_key = SymbolKey::new(character, FontStyle::Regular, size_px);
            let symbol_info = SymbolInfo {
                symbol_metrics,
                uv_region: texture_atlas.get(texture_atlas_index).unwrap().uv_region,
            };

            symbols.insert(symbol_key, symbol_info);
        }

        Ok(Self {
            symbols,
            line_metrics: font_regular
                .horizontal_line_metrics(size_px as f32)
                .unwrap(),
            texture_atlas,
            font_regular,
            _font_bold: font_bold,
            _font_italic: font_italic,
        })
    }

    /// Gets the SymbolInfo and top-left, bottom-right UVs for a symbol
    pub fn get_symbol(&self, symbol_key: SymbolKey) -> Option<SymbolInfo> {
        let symbol_info = self.symbols.get(&symbol_key)?;
        Some(*symbol_info)
    }
}
