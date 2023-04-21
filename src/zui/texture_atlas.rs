use std::fmt::Debug;
use std::hash::Hash;

use image::{DynamicImage, GenericImage};
// use image::{GenericImage, GenericImageView};
use rectangle_pack::{GroupedRectsToPlace, RectToInsert, RectanglePackOk, TargetBin};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

struct UnpackedSprite {
    image: DynamicImage,
    width_px: u32,
    height_px: u32,
}

pub struct TextureAtlasBuilder {
    unpacked_sprites: Vec<UnpackedSprite>,
}

pub struct PackedSprite {
    name: String,
    top_left: glam::Vec2,
    bottom_right: glam::Vec2,
    width_px: u32,
    height_px: u32,
}

impl TextureAtlasBuilder {
    pub fn new() -> Self {
        Self {
            unpacked_sprites: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, image: DynamicImage, width_px: u32, height_px: u32) {
        self.unpacked_sprites.push(UnpackedSprite {
            image,
            width_px,
            height_px,
        });
    }

    fn attempt_pack_expanding_until_fits<RectToPlaceId, GroupId>(
        initial_span: u32,
        grouped_rects_to_pack: &GroupedRectsToPlace<RectToPlaceId, GroupId>,
    ) -> Result<(RectanglePackOk<RectToPlaceId, i32>, u32), ()>
    where
        RectToPlaceId: Debug + Hash + Eq + Ord + PartialOrd + Clone + Copy,
        GroupId: Debug + Hash + Eq + Ord + PartialOrd + Clone + Copy,
    {
        let mut span = initial_span;
        let packed_results = loop {
            // adding target bins
            let mut target_bins = BTreeMap::new();
            target_bins.insert(0, TargetBin::new(span, span, 1));

            // attempting pack
            let results = match rectangle_pack::pack_rects(
                grouped_rects_to_pack,
                &mut target_bins,
                &rectangle_pack::volume_heuristic,
                &rectangle_pack::contains_smallest_box,
            ) {
                Ok(placements) => placements,
                Err(_e) => {
                    span += 256;
                    continue;
                }
            };

            // returns results if successful
            break (results, span);
        };

        Ok(packed_results)
    }

    pub fn build_atlas(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> TextureAtlas {
        // preparing rectangles to be packed
        let mut rects_to_place = GroupedRectsToPlace::new();
        for (index, unpacked_sprite) in self.unpacked_sprites.iter().enumerate() {
            rects_to_place.push_rect(
                index,
                Some(vec![0]),
                RectToInsert::new(unpacked_sprite.width_px, unpacked_sprite.height_px, 1),
            );
        }

        // getting the packed rectangles
        let (packing_results, atlas_span) =
            match Self::attempt_pack_expanding_until_fits(256, &rects_to_place) {
                Ok(results) => results,
                Err(_) => {
                    error!("failed to pack font!");
                    todo!()
                }
            };

        // copying sprite info images into new atlas image
        let mut atlas_image = image::GrayImage::new(atlas_span, atlas_span);
        for (rect_id, (_, location)) in packing_results.packed_locations() {
            // info!("rect id: {}", rect_id);
            let unpacked_sprite = &self.unpacked_sprites[*rect_id];

            atlas_image
                .copy_from(
                    unpacked_sprite.image.as_luma8().unwrap(),
                    location.x(),
                    location.y(),
                )
                .expect("copy_from failed!");
        }
        
        _ = atlas_image.save("output.png");

        //
        //  Creating texture and wgpu related structs
        //

        // creating and uploading the wgpu texture
        let texture = {
            let texture_size = wgpu::Extent3d {
                width: atlas_span,
                height: atlas_span,
                depth_or_array_layers: 1,
            };

            // creating a relevant handle to our texture
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("texture_curses"),
            });

            // writing the texture to GPU memory via created handle
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &atlas_image,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(1 * atlas_span),
                    rows_per_image: std::num::NonZeroU32::new(atlas_span),
                },
                texture_size,
            );

            texture
        };

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_atlas_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_atlas_bind_group"),
        });
        // saving to disk
        // atlas_image.save_with_format("out.png", image::ImageFormat::Png).expect("failed to save!");

        //
        //  Preparing final sprite list
        //

        // adding to sprites list
        let mut packed_sprites = Vec::new();
        let indices_used = self.unpacked_sprites.len();
        for index in 0..indices_used {
            let (_, (_, location)) = packing_results
                .packed_locations()
                .iter()
                .find(|(bin_id, _)| **bin_id == index)
                .expect(&format!("could not find pack result with id = {}", index));

            let top_left = glam::Vec2::new(
                location.x() as f32 / atlas_span as f32,
                location.y() as f32 / atlas_span as f32,
            );
            let bottom_right = top_left
                + glam::Vec2::new(
                    location.width() as f32 / atlas_span as f32,
                    location.height() as f32 / atlas_span as f32,
                );

            packed_sprites.push(PackedSprite {
                name: String::from("TODO"),
                top_left,
                bottom_right,
                width_px: location.width(),
                height_px: location.height(),
            });
        }

        // printing sprites
        // for sprite in &packed_sprites {
        //     sprite.print();
        // }

        TextureAtlas {
            texture,
            bind_group_layout,
            bind_group,
            packed_sprites,
        }
    }
}

pub struct TextureAtlas {
    /// The WGPU texture handle on which the sprites reside
    texture: wgpu::Texture,

    /// The bind group layout for the TextureAtlas, for use in other shaders
    bind_group_layout: wgpu::BindGroupLayout,

    /// The bind group of the TextureAtlas, for use in other shaders
    bind_group: wgpu::BindGroup,

    /// Collection of all sprites present on the texture atlas
    packed_sprites: Vec<PackedSprite>,
}

impl TextureAtlas {
    /// Gives the id for a provided file stem/name
    // pub fn sprite_id_from_name(&self, name: &str) -> Option<usize> {
    //     self.sprites.iter().position(|s| s.name() == name)
    // }

    /// Gives the id for a provided file stem/name, or returns the question mark texture
    // pub fn sprite_id_from_name_or_question_mark(&self, name: &str) -> Option<usize> {
    //     match self.sprites.iter().position(|s| s.name() == name) {
    //         Some(index) => {
    //             return Some(index);
    //         }
    //         None => {
    //             warn!(
    //                 "could not find sprite: '{}', attempting to return question mark!",
    //                 name
    //             );
    //             match self.sprite_id_from_name("question_mark") {
    //                 Some(question_mark_index) => Some(question_mark_index),
    //                 None => {
    //                     warn!("could not find question mark texture!");
    //                     return None;
    //                 }
    //             }
    //         }
    //     }
    // }

    /// Returns the sprite for a given sprite id
    // pub fn sprite_from_id(&self, id: usize) -> &Sprite {
    //     &self.sprites[id]
    // }

    /// Returns the texture atlas texture
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// Returns a reference to the texture atlas' bind group layout
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    /// Returns a reference to the texture atlas' bind group
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
