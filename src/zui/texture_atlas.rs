use crunch::{Item, Rotation};
use image::{DynamicImage, GenericImage};

use super::primitives::Rectangle;

struct UnpackedSprite {
    /// The image that is to be packed
    image: DynamicImage,

    /// The dimensions of the space to reserve when packing into the final atlas
    dimensions_px: glam::UVec2,
}

pub struct TextureAtlasBuilder {
    unpacked_sprites: Vec<UnpackedSprite>,
}

pub struct PackedSprite {
    /// The name of the packed sprite
    pub name: String,

    /// The UV region of the packed sprite
    pub uv_region: Rectangle,

    /// The dimensions of the final packed region, not including padding
    dimensions_px: glam::UVec2,
}

impl TextureAtlasBuilder {
    /// The number of pixels a sprite is padded by
    const PADDING: u32 = 1u32;

    pub fn new() -> Self {
        Self {
            unpacked_sprites: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, image: DynamicImage, width_px: u32, height_px: u32) {
        self.unpacked_sprites.push(UnpackedSprite {
            image,
            dimensions_px: glam::UVec2::new(width_px, height_px),
        });
    }

    pub fn build_atlas(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> TextureAtlas {
        // preparing rectangles to be packed
        let mut rects_to_place = Vec::new();
        for (index, unpacked_sprite) in self.unpacked_sprites.iter().enumerate() {
            rects_to_place.push(Item::new(
                index,
                (unpacked_sprite.dimensions_px.x + Self::PADDING * 2) as usize,
                (unpacked_sprite.dimensions_px.y + Self::PADDING * 2) as usize,
                Rotation::None,
            ));
        }

        // getting the packed rectangles
        let packed_items = match crunch::pack_into_po2(1024 * 10, rects_to_place) {
            Ok(res) => res,
            Err(_) => {
                error!("failed to pack items!");
                panic!();
            }
        };

        // copying sprite info images into new atlas image
        let atlas_width = packed_items.w as u32;
        let atlas_height = packed_items.h as u32;
        let mut atlas_image = image::GrayImage::new(atlas_width, atlas_height);
        for packed_item in packed_items.items.iter() {
            // info!("rect id: {}", rect_id);
            let index = packed_item.data;
            let rect = packed_item.rect;
            let unpacked_sprite = &self.unpacked_sprites[index];

            atlas_image
                .copy_from(
                    unpacked_sprite.image.as_luma8().unwrap(),
                    rect.x as u32 + Self::PADDING,
                    rect.y as u32 + Self::PADDING,
                )
                .expect("copy_from failed!");
        }

        // _ = atlas_image.save("output.png");

        //
        //  Creating texture and wgpu related structs
        //

        // creating and uploading the wgpu texture
        let texture = {
            let texture_size = wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
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
                    bytes_per_row: std::num::NonZeroU32::new(1 * atlas_width),
                    rows_per_image: std::num::NonZeroU32::new(atlas_height),
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
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
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
            let rect = packed_items
                .items
                .iter()
                .find(|item| item.data == index)
                .expect(&format!("could not find pack result with id = {}", index));

            // uvs
            let x_min = rect.rect.x as f32 / atlas_width as f32;
            let x_max = x_min + rect.rect.w as f32 / atlas_width as f32;
            let y_min = rect.rect.y as f32 / atlas_height as f32;
            let y_max = y_min + rect.rect.h as f32 / atlas_height as f32;

            packed_sprites.push(PackedSprite {
                name: String::from("TODO"),
                dimensions_px: glam::UVec2::new(rect.rect.w as u32, rect.rect.h as u32),
                uv_region: Rectangle::new(x_min, x_max, y_min, y_max),
            });
        }

        // printing sprites
        // for sprite in &packed_sprites {
        //     sprite.print();
        // }

        TextureAtlas {
            _texture: texture,
            bind_group_layout,
            bind_group,
            packed_sprites,
        }
    }
}

pub struct TextureAtlas {
    /// The WGPU texture handle on which the sprites reside
    _texture: wgpu::Texture,

    /// The bind group layout for the TextureAtlas, for use in other shaders
    bind_group_layout: wgpu::BindGroupLayout,

    /// The bind group of the TextureAtlas, for use in other shaders
    bind_group: wgpu::BindGroup,

    /// Collection of all sprites present on the texture atlas
    packed_sprites: Vec<PackedSprite>,
}

impl TextureAtlas {
    pub fn get<'a>(&'a self, index: usize) -> Option<&'a PackedSprite> {
        Some(self.packed_sprites.get(index)?)
    }

    /// Returns the texture atlas texture
    pub fn _texture(&self) -> &wgpu::Texture {
        &self._texture
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
