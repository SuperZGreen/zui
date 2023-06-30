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
    pub uv_region: Rectangle<f32>,

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

    /// Adds a sprite to be built into the texture atlas
    pub fn add_sprite(&mut self, image: DynamicImage, width_px: u32, height_px: u32) -> usize {
        let index = self.unpacked_sprites.len();
        self.unpacked_sprites.push(UnpackedSprite {
            image,
            dimensions_px: glam::UVec2::new(width_px, height_px),
        });

        index
    }

    /// Prepares the list of sprites for crunch to process
    fn prepare_crunch_items(&self) -> Vec<Item<usize>> {
        let mut items_to_place = Vec::new();
        for (index, unpacked_sprite) in self.unpacked_sprites.iter().enumerate() {
            items_to_place.push(Item::new(
                index,
                (unpacked_sprite.dimensions_px.x + Self::PADDING * 2) as usize,
                (unpacked_sprite.dimensions_px.y + Self::PADDING * 2) as usize,
                Rotation::None,
            ));
        }

        items_to_place
    }

    /// Copies all of the packed items into an image
    fn image_from_packed_items(
        &self,
        packed_items: &crunch::PackedItems<usize>,
    ) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let atlas_width = packed_items.w as u32;
        let atlas_height = packed_items.h as u32;
        let mut atlas_image = image::GrayImage::new(atlas_width, atlas_height);

        // copying all packed items
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

        atlas_image
    }

    /// Creates a texture from the atlas image
    fn texture_from_atlas_image(
        &self,
        image: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Texture {
        let texture_size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
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
            view_formats: &[],
        });

        // writing the texture to GPU memory via created handle
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(1 * image.width()),
                rows_per_image: Some(image.height()),
            },
            texture_size,
        );

        texture
    }

    /// Generates the packed sprites Vector
    fn generate_packed_sprites(
        &self,
        packed_items: &crunch::PackedItems<usize>,
        atlas_width: u32,
        atlas_height: u32,
    ) -> Vec<PackedSprite> {
        let mut packed_sprites = Vec::new();
        let indices_used = self.unpacked_sprites.len();
        for index in 0..indices_used {
            let rect = packed_items
                .items
                .iter()
                .find(|item| item.data == index)
                .expect(&format!("could not find pack result with id = {}", index));

            // The uv region in rust-image pixel coordinates
            let uv_region_px = Rectangle::new(
                rect.rect.x as u32 + Self::PADDING,
                rect.rect.x as u32 + rect.rect.w as u32 - Self::PADDING,
                rect.rect.y as u32 + Self::PADDING,
                rect.rect.y as u32 + rect.rect.h as u32 - Self::PADDING,
            );

            // The uv region in wgpu UV coordinates
            let uv_region = Rectangle::new(
                uv_region_px.x_min as f32 / atlas_width as f32,
                uv_region_px.x_max as f32 / atlas_width as f32,
                uv_region_px.y_min as f32 / atlas_height as f32,
                uv_region_px.y_max as f32 / atlas_height as f32,
            );

            packed_sprites.push(PackedSprite {
                name: String::from("TODO"),
                dimensions_px: glam::UVec2::new(rect.rect.w as u32, rect.rect.h as u32),
                // uv_region: Rectangle::new(x_min, x_max, y_min, y_max),
                uv_region,
            });
        }

        // printing sprites
        // for sprite in &packed_sprites {
        //     sprite.print();
        // }

        packed_sprites
    }

    ///// Builds the atlas
    //pub fn build_atlas(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> TextureAtlas {
    //    // preparing rectangles to be packed
    //    let items_to_place = self.prepare_crunch_items();

    //    // getting the packed rectangles
    //    let packed_items = match crunch::pack_into_po2(1024 * 10, items_to_place) {
    //        Ok(res) => res,
    //        Err(_) => {
    //            error!("failed to pack items!");
    //            panic!();
    //        }
    //    };

    //    // copying sprite info images into new atlas image
    //    let atlas_image = self.packed_items_to_image(packed_items);

    //    // _ = atlas_image.save("output.png");

    //    //
    //    //  Creating texture and wgpu related structs
    //    //

    //    // creating and uploading the wgpu texture

    //    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    //    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    //        address_mode_u: wgpu::AddressMode::ClampToEdge,
    //        address_mode_v: wgpu::AddressMode::ClampToEdge,
    //        address_mode_w: wgpu::AddressMode::ClampToEdge,
    //        mag_filter: wgpu::FilterMode::Linear,
    //        min_filter: wgpu::FilterMode::Linear,
    //        mipmap_filter: wgpu::FilterMode::Linear,
    //        ..Default::default()
    //    });

    //    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    //        entries: &[
    //            wgpu::BindGroupLayoutEntry {
    //                binding: 0,
    //                visibility: wgpu::ShaderStages::FRAGMENT,
    //                ty: wgpu::BindingType::Texture {
    //                    multisampled: false,
    //                    view_dimension: wgpu::TextureViewDimension::D2,
    //                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
    //                },
    //                count: None,
    //            },
    //            wgpu::BindGroupLayoutEntry {
    //                binding: 1,
    //                visibility: wgpu::ShaderStages::FRAGMENT,
    //                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
    //                count: None,
    //            },
    //        ],
    //        label: Some("texture_atlas_bind_group_layout"),
    //    });

    //    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //        layout: &bind_group_layout,
    //        entries: &[
    //            wgpu::BindGroupEntry {
    //                binding: 0,
    //                resource: wgpu::BindingResource::TextureView(&texture_view),
    //            },
    //            wgpu::BindGroupEntry {
    //                binding: 1,
    //                resource: wgpu::BindingResource::Sampler(&sampler),
    //            },
    //        ],
    //        label: Some("texture_atlas_bind_group"),
    //    });

    //    // saving to disk
    //    // atlas_image.save_with_format("out.png", image::ImageFormat::Png).expect("failed to save!");

    //    //
    //    //  Preparing final sprite list
    //    //

    //    // adding to sprites list
    //    let mut packed_sprites = Vec::new();
    //    let indices_used = self.unpacked_sprites.len();
    //    for index in 0..indices_used {
    //        let rect = packed_items
    //            .items
    //            .iter()
    //            .find(|item| item.data == index)
    //            .expect(&format!("could not find pack result with id = {}", index));

    //        // The uv region in rust-image pixel coordinates
    //        let uv_region_px = Rectangle::new(
    //            rect.rect.x as u32 + Self::PADDING,
    //            rect.rect.x as u32 + rect.rect.w as u32 - Self::PADDING,
    //            rect.rect.y as u32 + Self::PADDING,
    //            rect.rect.y as u32 + rect.rect.h as u32 - Self::PADDING,
    //        );

    //        // The uv region in wgpu UV coordinates
    //        let uv_region = Rectangle::new(
    //            uv_region_px.x_min as f32 / atlas_width as f32,
    //            uv_region_px.x_max as f32 / atlas_width as f32,
    //            uv_region_px.y_min as f32 / atlas_height as f32,
    //            uv_region_px.y_max as f32 / atlas_height as f32,
    //        );

    //        packed_sprites.push(PackedSprite {
    //            name: String::from("TODO"),
    //            dimensions_px: glam::UVec2::new(rect.rect.w as u32, rect.rect.h as u32),
    //            // uv_region: Rectangle::new(x_min, x_max, y_min, y_max),
    //            uv_region,
    //        });
    //    }

    //    // printing sprites
    //    // for sprite in &packed_sprites {
    //    //     sprite.print();
    //    // }

    //    TextureAtlas {
    //        _texture: texture,
    //        bind_group_layout,
    //        bind_group,
    //        packed_sprites,
    //    }
    //}
}

pub struct TextureAtlas {
    /// The bind group layout for the TextureAtlas, for use in other shaders. Is created once.
    bind_group_layout: wgpu::BindGroupLayout,

    /// The sampler for the texture. Is created once.
    sampler: wgpu::Sampler,

    /// The WGPU texture handle on which the sprites reside, will be recreated every time that the
    /// TextureAtlas is rebuilt
    texture: Option<wgpu::Texture>,

    /// the cpu side version of the texture atlas image, used for debug saving as a file
    image: Option<image::ImageBuffer<image::Luma<u8>, Vec<u8>>>,

    /// The bind group of the TextureAtlas, for use in other shaders. Will be recreated every time
    /// that the TextureAtlas is rebuilt
    bind_group: Option<wgpu::BindGroup>,

    /// Collection of all sprites present on the texture atlas. Will be recreated every time that
    /// the TextureAtlas is rebuilt
    packed_sprites: Option<Vec<PackedSprite>>,
}

impl TextureAtlas {
    /// Creates a new texture atlas, with a GPU side layout descriptor, that will be common between
    /// calls to build the texture atlas
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        // the sampler for the texture
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // the bind group layout
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

        Self {
            bind_group_layout,
            sampler,
            texture: None,
            image: None,
            bind_group: None,
            packed_sprites: None,
        }
    }

    /// Builds the atlas based on the provided TextureAtlasBuilder, uploads the texture to GPU, etc.
    pub fn build(
        &mut self,
        builder: TextureAtlasBuilder,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // getting the list of sprites to place in crunch
        let items_to_place = builder.prepare_crunch_items();

        // packing the items in crunch
        let packed_items = match crunch::pack_into_po2(1024 * 10, items_to_place) {
            Ok(pi) => pi,
            Err(e) => {
                error!("failed to pack: {e:?}");
                panic!();
            }
        };

        // copying sprite info images into new atlas image
        let atlas_image = builder.image_from_packed_items(&packed_items);

        // creating and uploading the wgpu texture
        let texture = builder.texture_from_atlas_image(&atlas_image, device, queue);

        // creating texture view
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // creating bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
            label: Some("texture_atlas_bind_group"),
        });

        // creating packed sprites list
        let packed_sprites = builder.generate_packed_sprites(
            &packed_items,
            atlas_image.width(),
            atlas_image.height(),
        );

        // assignments
        self.image = Some(atlas_image);
        self.texture = Some(texture);
        self.bind_group = Some(bind_group);
        self.packed_sprites = Some(packed_sprites);
    }

    /// Saves the texture atlas image to disk, for debugging
    pub fn save_texture_to_disk(&self, path: &str) -> Result<(), ()> {
        if let Some(image) = self.image.as_ref() {
            image
                .save_with_format(path, image::ImageFormat::Png)
                .expect("failed to save!");
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get<'a>(&'a self, index: usize) -> Option<&'a PackedSprite> {
        match self.packed_sprites.as_ref() {
            Some(ps) => Some(ps.get(index)?),
            None => None,
        }
    }

    // /// Returns the texture atlas texture
    // pub fn _texture(&self) -> &wgpu::Texture {
    //     &self._texture
    // }

    /// Returns a reference to the texture atlas' bind group layout
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    /// Returns a reference to the texture atlas' bind group, used when rendering with TextRenderer
    pub fn bind_group(&self) -> &Option<wgpu::BindGroup> {
        &self.bind_group
    }
}
