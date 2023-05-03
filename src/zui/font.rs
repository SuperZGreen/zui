use std::path::PathBuf;

use fontdue::{LineMetrics, Metrics};
use image::{DynamicImage, ImageBuffer, Luma};
use rustc_hash::FxHashMap;

use super::texture_atlas::TextureAtlas;
use crate::zui::texture_atlas::TextureAtlasBuilder;

pub struct Font {
    symbols: FxHashMap<char, SymbolInfo>,
    pub line_metrics: LineMetrics,
    pub texture_atlas: TextureAtlas,
}

pub struct SymbolInfo {
    pub metrics: Metrics,
    atlas_index: usize,
}

impl Font {
    fn coverage_to_dynamic_image(coverage: Vec<u8>, width_px: u32, height_px: u32) -> DynamicImage {
        let buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
            match ImageBuffer::from_raw(width_px, height_px, coverage) {
                Some(res) => res,
                None => todo!(),
            };

        DynamicImage::ImageLuma8(buffer)
    }

    pub fn new(
        file_path: &str,
        size_px: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, ()> {
        let font_path = PathBuf::from(file_path);

        let font_bytes = match std::fs::read(&font_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                error!("Could not load font file: {}", font_path.display());
                return Err(());
            }
        };

        let font = fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default()).unwrap();

        let mut texture_atlas_builder = TextureAtlasBuilder::new();
        let mut symbols = FxHashMap::default();
        for (index, (character, _)) in font.chars().iter().enumerate() {
            let (metrics, coverage) = font.rasterize(*character, size_px as f32);
            let width_px = metrics.width as u32;
            let height_px = metrics.height as u32;
            let image = Self::coverage_to_dynamic_image(coverage, width_px, height_px);
            texture_atlas_builder.add_sprite(image, width_px, height_px);
            symbols.insert(
                *character,
                SymbolInfo {
                    metrics,
                    atlas_index: index,
                },
            );
        }

        let texture_atlas = texture_atlas_builder.build_atlas(device, queue);

        Ok(Self {
            symbols,
            line_metrics: font.horizontal_line_metrics(size_px as f32).unwrap(),
            texture_atlas,
        })
    }

    /// Gets the SymbolInfo and top-left, bottom-right UVs for a symbol
    pub fn get_symbol(&self, character: char) -> Option<(&SymbolInfo, glam::Vec2, glam::Vec2)> {
        let symbol_info = self.symbols.get(&character)?;
        let packed_sprite = self.texture_atlas.get(symbol_info.atlas_index)?;

        Some((
            symbol_info,
            packed_sprite.top_left,
            packed_sprite.bottom_right,
        ))
    }
}
