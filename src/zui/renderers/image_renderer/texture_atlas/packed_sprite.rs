use crate::zui::primitives::{Rectangle, Dimensions};

/// Contains post-packing sprite info, including the name and sprite coordinates in the atlas
/// texture
#[derive(Clone, Debug)]
pub struct PackedSprite {
    /// The name of the Sprite, used for lookup
    name: String,

    /// The region on the texture atlas that the sprite takes up in uv coordinates
    region: Rectangle<f32>,

    /// The pixel dimensions of the sprite. This is used for sprite scaling when using the billboard
    /// renderer
    dimensions_px: Dimensions<i32>,
}

impl PackedSprite {
    /// Creates a new Sprite
    pub fn new(name: &str, region: Rectangle<f32>, dimensions_px: Dimensions<i32>) -> PackedSprite {
        PackedSprite {
            name: String::from(name),
            region,
            dimensions_px,
        }
    }

    /// Gives the dimensions of the sprite in pixels
    pub fn dimensions_px(&self) -> Dimensions<i32> {
        self.dimensions_px
    }

    /// Gives the texture region of the sprite
    pub fn region(&self) -> Rectangle<f32> {
        self.region
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Converts the region into UVs for rendering. Using y-down:
    ///
    ///   0 -----> 1
    ///          /
    ///        /
    ///      /
    ///    /
    ///   2 -----> 3
    pub fn uvs(&self) -> [glam::Vec2; 4] {
        [
            glam::Vec2::new(self.region.x_min, self.region.y_min),
            glam::Vec2::new(self.region.x_max, self.region.y_min),
            glam::Vec2::new(self.region.x_min, self.region.y_max),
            glam::Vec2::new(self.region.x_max, self.region.y_max),
        ]
    }

    /// Converts the region into UVs for rendering, fills in an array rather than creating it, for
    /// optimisation purposes
    pub fn populate_uvs(&self, uvs: &mut [glam::Vec2; 4]) {
        uvs[0] = glam::Vec2::new(self.region.x_min, self.region.y_min);
        uvs[1] = glam::Vec2::new(self.region.x_max, self.region.y_min);
        uvs[2] = glam::Vec2::new(self.region.x_min, self.region.y_max);
        uvs[3] = glam::Vec2::new(self.region.x_max, self.region.y_max);
    }
}
