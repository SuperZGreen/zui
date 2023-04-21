use std::path::PathBuf;

use fontdue::Metrics;
use image::{DynamicImage, ImageBuffer, Luma};
use rand::Rng;
use rustc_hash::FxHashMap;

use crate::zui::texture_atlas::TextureAtlasBuilder;
use super::texture_atlas::TextureAtlas;

pub struct Font {
    symbols: FxHashMap<char, ()>,
    pub texture_atlas: TextureAtlas,
}

struct SymbolInfo {
    metrics: Metrics,
    atlas_index: usize,
}

impl Font {
    fn coverage_to_dynamic_image(coverage: Vec<u8>, width_px: u32, height_px: u32) -> DynamicImage {
        // let temp_buffer: Vec<u8> =
        //     vec![rand::thread_rng().gen(); width_px as usize * height_px as usize];

        let buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
            match ImageBuffer::from_raw(width_px, height_px, coverage) {
            // match ImageBuffer::from_raw(width_px, height_px, temp_buffer) {
                Some(res) => res,
                None => todo!(),
            };

        DynamicImage::ImageLuma8(buffer)
    }

    pub fn new(
        file_path: &str,
        _size_px: u32,
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
        for (character, _) in font.chars() {
            let (metrics, coverage) = font.rasterize(*character, 40f32);
            let width_px = metrics.width as u32;
            let height_px = metrics.height as u32;
            let image = Self::coverage_to_dynamic_image(coverage, width_px, height_px);
            texture_atlas_builder.add_sprite(image, width_px, height_px);
        }

        let texture_atlas = texture_atlas_builder.build_atlas(device, queue);

        Ok(Self {
            symbols: FxHashMap::default(),
            texture_atlas,
        })
    }
}
