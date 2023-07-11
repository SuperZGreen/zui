pub mod named;

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colour {
    pub const WHITE: Self = Self {r: 1f32, g: 1f32, b: 1f32, a: 1f32};
    pub const LIGHT_GREY: Self = Self {r: 0.75f32, g:0.75f32, b: 0.75f32, a: 1f32};
    pub const GREY: Self = Self {r: 0.5f32, g: 0.5f32, b: 0.5f32, a: 1f32};
    pub const DARK_GREY: Self = Self {r: 0.25f32, g:0.25f32, b: 0.25f32, a: 1f32};
    pub const BLACK: Self = Self {r: 0f32, g: 0f32, b: 0f32, a: 1f32};
    
    pub const LIGHT_RED: Self = Self {r: 1f32, g: 0.5f32, b: 0.5f32, a: 1f32};
    pub const RED: Self = Self {r: 1f32, g: 0f32, b: 0f32, a: 1f32};
    pub const DARK_RED: Self = Self {r: 0.5f32, g: 0f32, b: 0f32, a: 1f32};

    pub const LIGHT_YELLOW: Self = Self {r: 1f32, g: 1f32, b: 0.5f32, a: 1f32};
    pub const YELLOW: Self = Self {r: 1f32, g: 1f32, b: 0f32, a: 1f32};
    pub const DARK_YELLOW: Self = Self {r: 0.5f32, g: 0.5f32, b: 0f32, a: 1f32};

    pub const LIGHT_ORANGE: Self = Self {r: 1f32, g: 0.75f32, b: 0.5f32, a: 1f32};
    pub const ORANGE: Self = Self {r: 1f32, g: 0.5f32, b: 0f32, a: 1f32};
    pub const DARK_ORANGE: Self = Self {r: 0.5f32, g: 0.25f32, b: 0f32, a: 1f32};

    pub const LIGHT_GREEN: Self = Self {r: 0.5f32, g: 1f32, b: 0.5f32, a: 1f32};
    pub const GREEN: Self = Self {r: 0f32, g: 1f32, b: 0f32, a: 1f32};
    pub const DARK_GREEN: Self = Self {r: 0f32, g: 0.5f32, b: 0f32, a: 1f32};

    pub const LIGHT_CYAN: Self = Self {r: 0.5f32, g: 1f32, b: 1f32, a: 1f32};
    pub const CYAN: Self = Self {r: 0f32, g: 1f32, b: 1f32, a: 1f32};
    pub const DARK_CYAN: Self = Self {r: 0f32, g: 0.5f32, b: 0.5f32, a: 1f32};

    pub const LIGHT_BLUE: Self = Self {r: 0.5f32, g: 0.5f32, b: 1f32, a: 1f32};
    pub const BLUE: Self = Self {r: 0f32, g: 0f32, b: 1f32, a: 1f32};
    pub const DARK_BLUE: Self = Self {r: 0f32, g: 0f32, b: 0.5f32, a: 1f32};

    pub const LIGHT_MAGENTA: Self = Self {r: 1f32, g: 0.5f32, b: 1f32, a: 1f32};
    pub const MAGENTA: Self = Self {r: 1f32, g: 0f32, b: 1f32, a: 1f32};
    pub const DARK_MAGENTA: Self = Self {r: 0.5f32, g: 0f32, b: 0.5f32, a: 1f32};

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
