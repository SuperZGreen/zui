use std::path::PathBuf;

use image::{DynamicImage, ImageBuffer, Luma};
use rustc_hash::{FxHashMap, FxHashSet};

use super::{primitives::Rectangle, texture_atlas::TextureAtlas};
use crate::zui::texture_atlas::TextureAtlasBuilder;

/// The identifying information for a character rasterised at a specific size
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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

/// A font that is available and rasterised in the TextureAtlas
struct RasterisedFont {
    style: FontStyle,
    size_px: u32,
}

/// Contains the fonts and rasterised glyphs for a typeface, useful in rendering text to the screen
pub struct Typeface {
    /// A map of symbol keys to symbol infos, which provides the metrics and uv coordinates for a
    /// glyph
    symbols: FxHashMap<SymbolKey, SymbolInfo>,

    /// The symbols that have been requested by the user. This is where the required symbols are
    /// gathered before they are packed
    requested_symbols: FxHashSet<SymbolKey>,

    /// This will be true if there are new symbols that are requested to be rasterised, that have
    /// not previously been rasterised
    new_symbols_to_rasterise: bool,

    /// The available RasterisedFonts for the TypeFace, ie the size & style of a typeface that can
    /// be drawn
    rasterised_fonts: Vec<RasterisedFont>,

    /// The TextureAtlas that holds the rasterised glyphs for the font
    pub texture_atlas: Option<TextureAtlas>,

    /// The normal font, must exist
    pub font_regular: Option<fontdue::Font>,

    /// The bold font, optional
    pub font_bold: Option<fontdue::Font>,

    /// The bold font, optional
    pub font_italic: Option<fontdue::Font>,
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
        font_regular_file_path: Option<&str>,
        font_bold_file_path: Option<&str>,
        font_italic_file_path: Option<&str>,
    ) -> Result<Self, ()> {
        // getting all fonts from files
        let font_regular =
            font_regular_file_path.map(|fp| Self::fontdue_font_from_file(fp).unwrap());
        let font_bold = font_bold_file_path.map(|fp| Self::fontdue_font_from_file(fp).unwrap());
        let font_italic = font_italic_file_path.map(|fp| Self::fontdue_font_from_file(fp).unwrap());

        Ok(Self {
            symbols: FxHashMap::default(),
            requested_symbols: FxHashSet::default(),
            new_symbols_to_rasterise: false,
            rasterised_fonts: Vec::new(),
            texture_atlas: None,
            font_regular,
            font_bold,
            font_italic,
        })
    }

    /// Queues the rasterisation of a font at a size, fonts must be rasterised before they can be
    /// used!
    pub fn queue_rasterise(&mut self, style: FontStyle, size_px: u32) {
        self.rasterised_fonts
            .push(RasterisedFont { style, size_px })
    }

    /// Gets the font from a FontStyle enum
    fn font_from_font_style<'a>(&'a self, style: FontStyle) -> &'a Option<fontdue::Font> {
        match style {
            FontStyle::Regular => &self.font_regular,
            FontStyle::Bold => &self.font_bold,
            FontStyle::Italic => &self.font_italic,
        }
    }

    /// Builds the underlying texture atlas
    pub fn rasterise(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut texture_atlas_builder = TextureAtlasBuilder::new();

        // rasterising all characters and putting into texture atlas, contains the atlas index,
        // SymbolKey and SymbolMetrics of a symbol, where the SymbolMetrics and atlas_index will
        // later be combined into a public SymbolInfo struct
        let mut rasterisation_info = Vec::new();

        // Adding all characters for all fonts in rasterised_fonts
        for rasterised_font in self.rasterised_fonts.iter() {
            let font = match self.font_from_font_style(rasterised_font.style) {
                Some(f) => f,
                None => {
                    warn!("could not find font: {:?}", rasterised_font.style);
                    continue;
                }
            };

            // reserving space for the new characters
            rasterisation_info.reserve(font.chars().len());

            // adding the font's characters to rasterisation_info and the texture atlas builder
            for (character, _) in font.chars().iter() {
                let (metrics, coverage) =
                    font.rasterize(*character, rasterised_font.size_px as f32);
                let width_px = metrics.width as u32;
                let height_px = metrics.height as u32;
                let image = Self::coverage_to_dynamic_image(coverage, width_px, height_px);

                let texture_atlas_index =
                    texture_atlas_builder.add_sprite(image, width_px, height_px);

                rasterisation_info.push((
                    texture_atlas_index,
                    SymbolKey::new(*character, rasterised_font.style, rasterised_font.size_px),
                    SymbolMetrics::new(&metrics),
                ))
            }
        }
        // building the texture atlas
        let texture_atlas = Some(texture_atlas_builder.build_atlas(device, queue));

        // getting uv coordinates from TextureAtlas for all characters, inserting into symbols
        for (texture_atlas_index, symbol_key, symbol_metrics) in rasterisation_info {
            let symbol_info = SymbolInfo {
                symbol_metrics,
                uv_region: texture_atlas
                    .as_ref()
                    .unwrap()
                    .get(texture_atlas_index)
                    .unwrap()
                    .uv_region,
            };

            self.symbols.insert(symbol_key, symbol_info);
        }

        self.texture_atlas = texture_atlas;
    }

    /// Gets the SymbolInfo and top-left, bottom-right UVs for a symbol
    pub fn get_symbol(& self, symbol_key: SymbolKey) -> Option<SymbolInfo> {
        let symbol_info = self.symbols.get(&symbol_key)?;
        Some(*symbol_info)
    }

    /// Requests a symbol to be rasterised, which will happen when try_rasterise is called
    pub fn request_rasterise_symbol(&mut self, symbol_key: SymbolKey) {
        if !self.requested_symbols.insert(symbol_key) {
            self.new_symbols_to_rasterise = true;
        }
    }

    /// Rasterises symbols if required, should be called once a frame
    pub fn try_rasterise(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.new_symbols_to_rasterise {
            let mut texture_atlas_builder = TextureAtlasBuilder::new();

            // rasterising all characters and putting into texture atlas, contains the atlas index,
            // SymbolKey and SymbolMetrics of a symbol, where the SymbolMetrics and atlas_index will
            // later be combined into a public SymbolInfo struct
            let mut rasterisation_info = Vec::new();

            // reserving space for the new characters
            rasterisation_info.reserve(self.requested_symbols.len());

            // adding the font's characters to rasterisation_info and the texture atlas builder
            for symbol_key in self.requested_symbols.iter() {
                
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

                // rasterising the symbol and getting metrics
                let (metrics, coverage) =
                    font.rasterize(symbol_key.character, symbol_key.size_px as f32);
                let width_px = metrics.width as u32;
                let height_px = metrics.height as u32;
                
                // converting to an image image
                let image = Self::coverage_to_dynamic_image(coverage, width_px, height_px);

                // queueing for packing in the texture atlas
                let texture_atlas_index =
                    texture_atlas_builder.add_sprite(image, width_px, height_px);

                // adding the rasterisation information for constructing SymbolInfo after packing
                rasterisation_info.push((
                    texture_atlas_index,
                    *symbol_key,
                    SymbolMetrics::new(&metrics),
                ))
            }

            // building the texture atlas
            let texture_atlas = Some(texture_atlas_builder.build_atlas(device, queue));
            
            // clearing self symbols as the information will be out of date with the new
            // TextureAtlas
            self.symbols.clear();

            // getting uv coordinates from TextureAtlas for all characters, inserting into symbols
            for (texture_atlas_index, symbol_key, symbol_metrics) in rasterisation_info {
                let symbol_info = SymbolInfo {
                    symbol_metrics,
                    uv_region: texture_atlas
                        .as_ref()
                        .unwrap()
                        .get(texture_atlas_index)
                        .unwrap()
                        .uv_region,
                };

                // adding to symbols
                self.symbols.insert(symbol_key, symbol_info);
            }

            self.texture_atlas = texture_atlas;
            self.new_symbols_to_rasterise = false;
        }
    }
}
