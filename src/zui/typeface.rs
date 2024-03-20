pub mod coverage_image;
mod font;

use font::Font;

use rustc_hash::{FxHashMap, FxHashSet};

use self::coverage_image::CoverageImage;

use super::{primitives::Rectangle, texture_atlas::TextureAtlas};
use crate::zui::texture_atlas::TextureAtlasBuilder;

/// The identifying information for a character rasterised at a specific size
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

/// The information used externally to draw the Symbol to the screen
#[derive(Clone)]
pub struct SymbolInfo {
    /// The pixel metrics of the symbol
    pub symbol_metrics: SymbolMetrics,

    /// The coverage image of the symbol, to be reusued when re-rasterising symbols
    pub coverage_image: CoverageImage,

    /// The region that the symbol uses on the Typeface's texture atlas, using UV coordinates where
    /// (0, 0) is top left, (1, 1) is bottom right
    pub uv_region: Rectangle<f32>,
}

/// Describes the style of a typeface
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[allow(dead_code)]
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
    /// Generates the viewport pixel metrics for a symbol given its pixel metrics generated from
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
    /// A map of symbol keys to symbol infos, which provides the metrics and uv coordinates for a
    /// glyph
    symbols: FxHashMap<SymbolKey, SymbolInfo>,

    /// The TextureAtlas that holds the rasterised glyphs for the font
    pub texture_atlas: TextureAtlas,

    /// The normal font, must exist
    pub font_regular: Option<Font>,

    /// The bold font, optional
    pub font_bold: Option<Font>,

    /// The bold font, optional
    pub font_italic: Option<Font>,
}

impl Typeface {
    /// Creates a new Typeface
    pub fn new(
        font_regular_file_path: Option<&str>,
        font_bold_file_path: Option<&str>,
        font_italic_file_path: Option<&str>,
        device: &wgpu::Device,
    ) -> Result<Self, ()> {
        // loading all fonts from files
        let font_regular = font_regular_file_path.and_then(|fp| Font::load(fp));
        let font_bold = font_bold_file_path.and_then(|fp| Font::load(fp));
        let font_italic = font_italic_file_path.and_then(|fp| Font::load(fp));

        let texture_atlas = TextureAtlas::new(device);

        Ok(Self {
            symbols: FxHashMap::default(),
            texture_atlas,
            font_regular,
            font_bold,
            font_italic,
        })
    }

    /// Gets the font from a FontStyle enum
    fn font_from_font_style<'a>(&'a self, style: FontStyle) -> &'a Option<Font> {
        match style {
            FontStyle::Regular => &self.font_regular,
            FontStyle::Bold => &self.font_bold,
            FontStyle::Italic => &self.font_italic,
        }
    }

    /// Gets the SymbolInfo for a symbol
    pub fn get_symbol<'a>(&'a self, symbol_key: &SymbolKey) -> Option<&'a SymbolInfo> {
        let symbol_info = self.symbols.get(symbol_key)?;
        Some(symbol_info)
    }

    /// Rasterises symbols in the provided hashset, and discards those that aren't in symbol_keys
    pub fn rasterise_symbol_keys(
        &mut self,
        symbol_keys: FxHashSet<SymbolKey>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // checking if needs to be rebuilt
        let mut requires_rebuild = false;
        for symbol_key in symbol_keys.iter() {
            if !self.symbols.contains_key(symbol_key) {
                requires_rebuild = true;
                break;
            }
        }

        // early exit if does not need to be rebuilt
        if !requires_rebuild {
            return;
        }

        let mut texture_atlas_builder = TextureAtlasBuilder::new(symbol_keys.len());

        // rasterising all characters and putting into texture atlas, contains the atlas index,
        // SymbolKey and SymbolMetrics of a symbol, where the SymbolMetrics and atlas_index will
        // later be combined into a public SymbolInfo struct
        let mut rasterisation_info = Vec::with_capacity(symbol_keys.len());

        // adding the font's characters to rasterisation_info and the texture atlas builder
        for symbol_key in symbol_keys.iter() {
            // getting the font specified in the SymbolKey
            let font = match self.font_from_font_style(symbol_key.font_style) {
                Some(f) => f,
                None => {
                    error!(
                        "cannot rasterise character: {}, as font style: {:?} does not exist!",
                        symbol_key.character, symbol_key.font_style
                    );
                    continue;
                }
            };

            // Preventing rasterisation if the symbol already exists, reusing symbol metrics and
            // info instead
            let (symbol_metrics, symbol_coverage) = match self.symbols.get(symbol_key) {
                Some(si) => (si.symbol_metrics, si.coverage_image.clone()),
                None => {
                    // rasterising the symbol and getting metrics
                    font.rasterise(symbol_key.character, symbol_key.size_px)
                }
            };

            // queueing for packing in the texture atlas
            let texture_atlas_index = texture_atlas_builder.add_sprite(symbol_coverage.clone());

            // adding the rasterisation information for constructing SymbolInfo after packing
            rasterisation_info.push((
                texture_atlas_index,
                *symbol_key,
                symbol_metrics,
                symbol_coverage,
            ))
        }

        // let build_stopwatch = Stopwatch::start();
        // rebuilding the texture atlas and uploading texture
        self.texture_atlas
            .update_via_builder(texture_atlas_builder, device, queue);
        // trace!("building atlas took: {:.2}ms", build_stopwatch.elapsed() * 1000f32);

        // clearing self symbols as the information will be out of date with the new TextureAtlas
        self.symbols.clear();

        // getting uv coordinates from TextureAtlas for all characters, inserting into symbols
        for (texture_atlas_index, symbol_key, symbol_metrics, symbol_coverage) in rasterisation_info
        {
            let symbol_info = SymbolInfo {
                symbol_metrics,
                // uv_region: Rectangle::new(
                //     0f32, 1f32,
                //     0f32, 1f32,
                // ),
                uv_region: self
                    .texture_atlas
                    .get(texture_atlas_index)
                    .unwrap()
                    .uv_region,
                coverage_image: symbol_coverage,
            };

            // adding to symbols
            self.symbols.insert(symbol_key, symbol_info);
        }
    }
}
