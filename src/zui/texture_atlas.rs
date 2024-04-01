mod packer;

use crate::typeface::coverage_image::CoverageImage;

use super::primitives::Rectangle;

/// A sprite as input to the TextureAtlasBuilder
struct UnpackedSprite {
    /// The image that is to be packed
    image: CoverageImage,
}

/// Used to build the TextureAtlas' texture, TODO: can be removed and simplified so that the
/// TextureAtlas simply takes in an array of CoverageImages for its build. This would be more
/// memory efficient as the vec could be allocated to the length of the initial glyph buffer
pub struct TextureAtlasBuilder {
    unpacked_sprites: Vec<UnpackedSprite>,
}

/// This is what is returned as a result of a 'get' for the TextureAtlas
pub struct PackedSprite {
    /// The name of the packed sprite
    pub name: String,

    /// The UV region of the packed sprite
    pub uv_region: Rectangle<f32>,
}

impl TextureAtlasBuilder {
    /// The number of pixels a sprite is padded by
    /// Note: this isn't used, as text can render correctly without it as it is pixel aligned
    const PADDING: u32 = 0u32;

    pub fn new(reserved_sprites_length: usize) -> Self {
        Self {
            unpacked_sprites: Vec::with_capacity(reserved_sprites_length),
        }
    }

    /// Adds a sprite to be built into the texture atlas
    pub fn add_sprite(&mut self, image: CoverageImage) -> usize {
        let index = self.unpacked_sprites.len();
        self.unpacked_sprites.push(UnpackedSprite { image });

        index
    }

    /// Prepares the list of sprites for crunch to process
    fn prepare_items_to_place(&self) -> Vec<packer::InputItem> {
        let mut items_to_place = Vec::new();

        for unpacked_sprite in self.unpacked_sprites.iter() {
            items_to_place.push(packer::InputItem {
                width: unpacked_sprite.image.width,
                height: unpacked_sprite.image.height,
            });
        }

        items_to_place
    }

    /// Copies all of the packed items into an image
    fn image_from_packed_items(&self, packer_output: &packer::Output) -> CoverageImage {
        let atlas_width = packer_output.span;
        let atlas_height = packer_output.span;
        let mut atlas_image = CoverageImage::new(atlas_width, atlas_height);

        // copying all packed items
        for item in packer_output.items.iter() {
            // info!("rect id: {}", rect_id);
            let unpacked_sprite = &self.unpacked_sprites[item.input_item_index];

            atlas_image
                .copy_from(
                    &unpacked_sprite.image,
                    item.rectangle.x_min as u32 + Self::PADDING,
                    item.rectangle.y_min as u32 + Self::PADDING,
                )
                .expect("failed to copy symbol coverage to atlas!");
        }

        atlas_image
    }

    /// Generates the packed sprites Vector
    fn generate_packed_sprites(
        &self,
        packer_output: &packer::Output,
        atlas_width: u32,
        atlas_height: u32,
    ) -> Vec<PackedSprite> {
        let mut packed_sprites = Vec::with_capacity(packer_output.items.len());
        for item in packer_output.items.iter() {
            // The uv region in rust-image pixel coordinates
            let uv_region_px = item.rectangle;

            // The uv region in wgpu UV coordinates
            let uv_region = Rectangle::new(
                uv_region_px.x_min as f32 / atlas_width as f32,
                uv_region_px.x_max as f32 / atlas_width as f32,
                uv_region_px.y_min as f32 / atlas_height as f32,
                uv_region_px.y_max as f32 / atlas_height as f32,
            );

            packed_sprites.push(PackedSprite {
                name: String::from("TODO"),
                uv_region,
            });
        }

        // printing sprites
        // for sprite in &packed_sprites {
        //     sprite.print();
        // }

        packed_sprites
    }
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
    image: Option<CoverageImage>,

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
    pub fn new(device: &wgpu::Device) -> Self {
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
    pub fn update_via_builder(
        &mut self,
        builder: TextureAtlasBuilder,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let input_items = builder.prepare_items_to_place();

        // packing the items
        let packer = packer::Packer::new(2048);

        let packer_output = match packer.pack(&input_items) {
            Some(output) => output,
            None => {
                // TODO: this should be fail safe
                error!("failed to pack text atlas, panicing!");
                panic!();
            }
        };

        // copying sprite info images into new atlas image
        let atlas_image = builder.image_from_packed_items(&packer_output);

        // creating and uploading the wgpu texture
        let texture = self.try_update_gpu_texture(&atlas_image, device, queue);

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
        let packed_sprites =
            builder.generate_packed_sprites(&packer_output, atlas_image.width, atlas_image.height);

        // assignments
        self.image = Some(atlas_image);
        self.texture = Some(texture);
        self.bind_group = Some(bind_group);
        self.packed_sprites = Some(packed_sprites);
    }

    /// Updates the self texture and image fields if required
    fn try_update_gpu_texture(
        &mut self,
        new_image: &CoverageImage,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Texture {
        let texture_size = wgpu::Extent3d {
            width: new_image.width,
            height: new_image.height,
            depth_or_array_layers: 1,
        };

        // creating a new texture if required, othewise just updating the old texture
        let texture = match (&self.image, self.texture.take()) {
            (Some(previous_image), Some(previous_texture))
                if previous_image.width == new_image.width
                    && previous_image.height == new_image.height =>
            {
                previous_texture
            }
            _ => {
                // creating a relevant handle to our texture
                // Note: this causes a severe memory leak if run while window is minimised. This is
                // currently prevented via early in SceneHandle::solve if view dimensions have zero area.
                device.create_texture(&wgpu::TextureDescriptor {
                    size: texture_size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::R8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    label: Some("zui_glyph_texture_atlas"),
                    view_formats: &[],
                })
            }
        };

        // writing the texture to GPU memory via created handle
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &new_image.coverage,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(1 * new_image.width),
                rows_per_image: Some(new_image.height),
            },
            texture_size,
        );

        texture
    }

    /// Saves the texture atlas image to disk, for debugging
    pub fn save_texture_to_disk(&self, path: &str) -> Result<(), ()> {
        if let Some(image) = self.image.as_ref() {
            image::GrayImage::from_raw(image.width, image.height, image.coverage.to_vec())
                .expect("failed to convert from CoverageImage to GrayImage")
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
