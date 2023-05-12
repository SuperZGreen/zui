#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colour {
    pub const WHITE: Self = Self {r: 1f32, g: 1f32, b: 1f32, a: 1f32};
    pub const BLACK: Self = Self {r: 0f32, g: 0f32, b: 0f32, a: 1f32};

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1f32 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Colour> for glam::Vec4 {
    fn from(colour: Colour) -> Self {
        Self::new(colour.r, colour.g, colour.b, colour.a)
    }
}
