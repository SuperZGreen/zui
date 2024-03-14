//! Note that this texture atlas is taken straight from the green_game project.
//! This TextureAtlas is used for images and takes longer to build, and is more efficiently packed
//! than the text glyph TextureAtlas.

pub mod packed_sprite;

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use packed_sprite::PackedSprite;
use std::path::{Path, PathBuf};

use crate::zui::primitives::{Dimensions, Rectangle};

/// Id used to retrieve sprite information from the TextureAtlas
#[derive(Copy, Clone, PartialEq)]
pub struct SpriteId(usize);

impl Default for SpriteId {
    /// The Id of the fallback sprite. Should present as a question mark
    fn default() -> Self {
        SpriteId(0usize)
    }
}

/// Descriptor used to add a sprite to the TextureAtlas
pub struct SpriteDescriptor {
    /// Path to the sprite's file
    file_path: PathBuf,
}

/// Builder to create a texture atlas
pub struct TextureAtlasBuilder {
    /// Vector of sprite descriptors, which will be packed into the final textureatlas
    sprite_descriptors: Vec<SpriteDescriptor>,

    /// The number of pixels of padding each sprite will have
    padding: u32,
}

pub struct TextureAtlas {
    /// The WGPU texture handle on which the sprites reside
    texture: wgpu::Texture,

    /// The bind group layout for the TextureAtlas, for use in other shaders
    bind_group_layout: wgpu::BindGroupLayout,

    /// The bind group of the TextureAtlas, for use in other shaders
    bind_group: wgpu::BindGroup,

    /// Collection of all sprites present on the texture atlas
    sprites: Vec<PackedSprite>,
}

impl TextureAtlasBuilder {
    /// Creates a new empty TextureAtlasBuilder
    pub fn new<P: AsRef<Path>>(padding: u32, fallback_sprite_path: P) -> Self
    where
        PathBuf: From<P>,
    {
        // a fallback sprite that will have SpriteId(0)/SpriteId::default() to use if a sprite can
        // not be found.
        let fallback_sprite_descriptor = SpriteDescriptor {
            file_path: PathBuf::from(fallback_sprite_path),
        };
        Self {
            sprite_descriptors: vec![fallback_sprite_descriptor],
            padding,
        }
    }

    /// Adds a sprite to the TextureAtlasBuilder and returns its SpriteId.
    /// TODO: make it such that if a path already exists in the sprite descriptors, that sprite
    /// is returned instead of adding a whole new duplicate texture to the texture atlas
    pub fn add_sprite<P: AsRef<Path>>(&mut self, sprite_path: P) -> SpriteId
    where
        PathBuf: From<P>,
    {
        let id = SpriteId(self.sprite_descriptors.len());
        self.sprite_descriptors.push(SpriteDescriptor {
            file_path: PathBuf::from(sprite_path),
        });

        id
    }

    /// Builds the TextureAtlas. Max span gives the maximum edge dimension of the final texture
    /// before failure. Returns Ok(TextureAtlas) if the atlas was built successfully, returns
    /// Err(()) if the packed size of the atlas exceeded max_span.
    pub fn build(
        self,
        max_span: usize,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<TextureAtlas, ()> {
        // getting all of the sprite paths
        let sprite_paths: Vec<PathBuf> = self
            .sprite_descriptors
            .iter()
            .map(|sd| sd.file_path.clone())
            .collect();

        // creating SpritePackingInfos
        let sprite_packing_infos = Self::sprite_packing_infos_from_paths(sprite_paths);

        // creating crunch Items
        let crunch_items = self.items_from_sprite_packing_infos(&sprite_packing_infos);

        // attempting pack via crunch
        let packed_items = match crunch::pack_into_po2(max_span, crunch_items) {
            Ok(pi) => pi,
            Err(e) => {
                error!("failed to pack items, error: {e:?}");
                return Err(());
            }
        };

        // copying onto new image
        let (atlas_image, packed_sprites) = self.blit_sprites(sprite_packing_infos, packed_items);

        // saving the image
        atlas_image
            .save_with_format("out.png", image::ImageFormat::Png)
            .expect("failed to save!");

        // uploading image and creating texture
        let texture = texture_from_rgba_image(device, queue, &atlas_image)
            .expect("failed to create texture from image");

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

        Ok(TextureAtlas {
            texture,
            bind_group_layout,
            bind_group,
            sprites: packed_sprites,
        })
    }

    /// Loads sprites from the provided paths
    fn sprite_packing_infos_from_paths(sprite_paths: Vec<PathBuf>) -> Vec<SpritePackingInfo> {
        let mut sprite_packing_infos = Vec::with_capacity(sprite_paths.len());
        for (index, path) in sprite_paths.into_iter().enumerate() {
            let packing_info = match SpritePackingInfo::load_from_path(&path, SpriteId(index)) {
                Ok(spi) => spi,
                Err(e) => {
                    error!("failed to create SpritePackingInfo! error: {e:?}");
                    continue;
                }
            };

            sprite_packing_infos.push(packing_info);
        }

        sprite_packing_infos
    }

    /// Creates a vec of crunch items from the SpritePackingInfos
    fn items_from_sprite_packing_infos(
        &self,
        sprite_packing_infos: &[SpritePackingInfo],
    ) -> Vec<crunch::Item<SpriteId>> {
        let mut items = Vec::with_capacity(sprite_packing_infos.len());

        let padding_pixels = self.padding as usize * 2usize;

        for info in sprite_packing_infos {
            items.push(crunch::Item::new(
                info.id,
                info.image.width() as usize + padding_pixels,
                info.image.height() as usize + padding_pixels,
                crunch::Rotation::None,
            ));
        }

        items
    }

    /// Creates a new image to back the atlas and blits the provided sprites onto this image using
    /// the packing results provided by crunch's PackedItems struct. Returns the atlas image and a
    /// list of packed sprites
    fn blit_sprites(
        &self,
        infos: Vec<SpritePackingInfo>,
        packed_items: crunch::PackedItems<SpriteId>,
    ) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, Vec<PackedSprite>) {
        let mut atlas_image = image::RgbaImage::new(packed_items.w as u32, packed_items.h as u32);
        let mut packed_sprites = Vec::new();

        let padding_px = self.padding as usize;

        for info in infos {
            let item = packed_items
                .items
                .iter()
                .find(|i| i.data == info.id)
                .unwrap_or_else(|| panic!("could not find sprite: {}", info.name));

            let sprite_region_px = Rectangle {
                x_min: item.rect.x + padding_px,
                x_max: item.rect.x + item.rect.w - padding_px,
                y_min: item.rect.y + padding_px,
                y_max: item.rect.y + item.rect.h - padding_px,
            };

            // copying main image
            atlas_image
                .copy_from(
                    &info.image,
                    sprite_region_px.x_min as u32,
                    sprite_region_px.y_min as u32,
                )
                .expect("copy_from failed!");

            // copying boundary/padding margin
            Self::blit_padding(
                &info.image,
                &mut atlas_image,
                sprite_region_px,
                padding_px as u32,
            );

            let sprite_region_uv = Rectangle {
                x_min: sprite_region_px.x_min as f32 / packed_items.w as f32,
                x_max: sprite_region_px.x_max as f32 / packed_items.w as f32,
                y_min: sprite_region_px.y_min as f32 / packed_items.h as f32,
                y_max: sprite_region_px.y_max as f32 / packed_items.h as f32,
            };

            packed_sprites.push(PackedSprite::new(
                &info.name,
                sprite_region_uv,
                Dimensions {
                    width: info.image.width() as i32,
                    height: info.image.height() as i32,
                },
            ));
        }

        (atlas_image, packed_sprites)
    }

    /// Blits the padding of a source image to destination given the inner pixel region. NOTE: this
    /// does not do corners.
    fn blit_padding(
        img_src: &DynamicImage,
        img_dst: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        inner_region_dst: Rectangle<usize>,
        padding_px: u32,
    ) {
        for y in 0..img_src.height() {
            for p in 0..padding_px {
                // left side
                let pixel_src = img_src.get_pixel(0, y);
                let pixel_dst = img_dst.get_pixel_mut(
                    inner_region_dst.x_min as u32 - 1 - p,
                    y + inner_region_dst.y_min as u32,
                );
                *pixel_dst = pixel_src;

                // right side
                let pixel_src = img_src.get_pixel(img_src.width() - 1, y);
                let pixel_dst = img_dst.get_pixel_mut(
                    inner_region_dst.x_max as u32 + p,
                    y + inner_region_dst.y_min as u32,
                );
                *pixel_dst = pixel_src;
            }
        }

        for x in 0..img_src.width() {
            for p in 0..padding_px {
                // top
                let pixel_src = img_src.get_pixel(x, 0);
                let pixel_dst = img_dst.get_pixel_mut(
                    x + inner_region_dst.x_min as u32,
                    inner_region_dst.y_min as u32 - 1 - p,
                );
                *pixel_dst = pixel_src;

                // bottom
                let pixel_src = img_src.get_pixel(x, img_src.height() - 1);
                let pixel_dst = img_dst.get_pixel_mut(
                    x + inner_region_dst.x_min as u32,
                    inner_region_dst.y_max as u32 + p,
                );
                *pixel_dst = pixel_src;
            }
        }
    }
}

/// Sprite information required to pack a sprite
struct SpritePackingInfo {
    /// the name of the sprite, derived from its path. TODO: may be removed in the future
    name: String,

    /// The Id of the sprite, ie, it's location in the TextureAtlas's vec of sprites
    id: SpriteId,

    /// The image that the sprite contains
    image: DynamicImage,
}

impl SpritePackingInfo {
    /// Attemts to load a sprite from disk based on the provided path. Returns Err(()) if failed to
    /// load, or get path file stem.
    fn load_from_path(path: &Path, id: SpriteId) -> Result<Self, ()> {
        let image_reader = match image::io::Reader::open(path) {
            Ok(ir) => ir,
            Err(_) => {
                error!("failed to open requested file at: {}", path.display());
                return Err(());
            }
        };

        let image = match image_reader.decode() {
            Ok(di) => di,
            Err(e) => {
                error!("failed to decode image, error: {}", e);
                return Err(());
            }
        };

        let name = match path.file_stem() {
            Some(n) => match n.to_str() {
                Some(name_string) => String::from(name_string),
                None => {
                    error!("failed to convert from OsStr to str!");
                    return Err(());
                }
            },
            None => {
                error!("failed to get file: {path:?} stem!");
                return Err(());
            }
        };

        Ok(Self { name, image, id })
    }
}

impl TextureAtlas {
    pub const PIXELS_PER_METER: f32 = 512f32;

    /// Gets a PackedSprite from a SpriteId
    pub fn get(&self, id: SpriteId) -> &PackedSprite {
        &self.sprites[id.0]
    }

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

pub fn texture_from_rgba_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &image::RgbaImage,
) -> Result<wgpu::Texture, String> {

    // getting the image dimensions
    let image_dimensions = image.dimensions();

    // converting the image dimensions to a wgpu relevant format
    let texture_size = wgpu::Extent3d {
        width: image_dimensions.0,
        height: image_dimensions.1,
        depth_or_array_layers: 1,
    };

    // creating a relevant handle to our texture
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
        image,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * image_dimensions.0),
            rows_per_image: Some(image_dimensions.1),
        },
        texture_size,
    );

    Ok(texture)
}

