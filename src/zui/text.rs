use super::{primitives::Rectangle, Colour, Font};

#[derive(Clone)]
#[allow(dead_code)]
pub enum LineWrapping {
    None,
    Symbol,
    Word,
}

#[derive(Clone)]
pub enum TextSize {
    /// Height of the text (ascent to descent) Proportion of the parent's rectangle height
    ParentHeight(f32),

    /// Height of the text in pixels
    Pixels(f32),
    // /// Height of the text in screen space units
    // ScreenSpace(f32),
}

impl TextSize {
    /// Gives the text height in terms of screen space
    pub fn to_screen_space_span(
        &self,
        parent_rectangle_height: f32,
        viewport_height_px: u32,
    ) -> f32 {
        match self {
            TextSize::ParentHeight(proportion) => proportion * parent_rectangle_height,
            TextSize::Pixels(pixels) => pixels / viewport_height_px as f32 / 2f32,
            // TextSize::ScreenSpace() => todo!(),
        }
    }
}

#[derive(Clone)]
pub struct TextConfiguration {
    pub line_wrapping: LineWrapping,
    pub size: TextSize,
}

impl Default for TextConfiguration {
    fn default() -> Self {
        Self {
            line_wrapping: LineWrapping::Symbol,
            size: TextSize::ParentHeight(0.5f32),
        }
    }
}

#[derive(Clone)]
pub struct TextSegment {
    pub string: String,
    pub colour: Colour,
}

impl TextSegment {
    pub fn new(string: &str, colour: Colour) -> Self {
        Self {
            string: String::from(string),
            colour,
        }
    }
}

#[derive(Clone)]
pub struct Text {
    /// The actual content of the string
    pub segments: Vec<TextSegment>,

    /// The per-character rendering information of the text
    pub symbols: Vec<Symbol>,

    /// Includes layout and styling information for the text
    pub configuration: TextConfiguration,
}

impl Text {
    /// A new empty Text object
    pub fn new() -> Text {
        Self {
            segments: Vec::new(),
            symbols: Vec::new(),
            configuration: TextConfiguration::default(),
        }
    }

    /// Adds the text segment to the Text object
    pub fn with_segment(mut self, segment: TextSegment) -> Self {
        self.segments.push(segment);
        self
    }

    #[allow(dead_code)]
    pub fn with_configuration(mut self, configuration: TextConfiguration) -> Self {
        self.configuration = configuration;
        self
    }

    pub fn update_symbols(&mut self, font: &Font, parent_rect: &Rectangle, aspect_ratio: f32) {
        // the screen space height of a symbol
        // TODO, remove fixed pixels size
        let text_height_screen_space = self
            .configuration
            .size
            .to_screen_space_span(parent_rect.height(), 100u32);

        // the span of the original rasterised symbol
        let rasterised_font_height_px = font.line_metrics.ascent - font.line_metrics.descent;
        let rasterised_font_width_px = rasterised_font_height_px * aspect_ratio;

        let new_line_screen_space_span =
            font.line_metrics.new_line_size / rasterised_font_height_px * text_height_screen_space;

        let mut glyph_origin = glam::Vec2::new(
            parent_rect.x_min,
            parent_rect.y_max
                - font.line_metrics.ascent / rasterised_font_height_px * text_height_screen_space,
        );

        // calculating the screen space metrics of all symbols
        let presymbols = {
            let mut ps = Vec::new();
            for segment in self.segments.iter() {
                for character in segment.string.chars() {
                    let (info, uv_top_left, uv_bottom_right) = match font.get_symbol(character) {
                        Some(res) => res,
                        None => {
                            error!("could not find glyph for character: {}!", character);
                            continue;
                        }
                    };
                    let symbol_metrics = &info.metrics;

                    let normalised_symbol_width =
                        symbol_metrics.width as f32 / rasterised_font_width_px;
                    let normalised_symbol_height =
                        symbol_metrics.height as f32 / rasterised_font_height_px;
                    let normalised_symbol_x_shift =
                        symbol_metrics.xmin as f32 / rasterised_font_width_px;
                    let normalised_symbol_y_shift =
                        symbol_metrics.ymin as f32 / rasterised_font_height_px;
                    let normalised_symbol_advance_width =
                        symbol_metrics.advance_width / rasterised_font_width_px;

                    let symbol_metrics = SymbolMetrics {
                        width: normalised_symbol_width * text_height_screen_space,
                        height: normalised_symbol_height * text_height_screen_space,
                        x_shift: normalised_symbol_x_shift * text_height_screen_space,
                        y_shift: normalised_symbol_y_shift * text_height_screen_space,
                        advance_width: normalised_symbol_advance_width * text_height_screen_space,
                    };

                    ps.push(Presymbol {
                        character,
                        colour: segment.colour,
                        uv_top_left,
                        uv_bottom_right,
                        symbol_metrics,
                    })
                }
            }

            ps
        };

        // placing the symbols
        self.symbols = {
            let mut symbols = Vec::with_capacity(presymbols.len());
            for presymbol in presymbols {
                // line wrapping
                if matches!(self.configuration.line_wrapping, LineWrapping::Symbol) {
                    if glyph_origin.x()
                        + presymbol.symbol_metrics.width
                        + presymbol.symbol_metrics.x_shift
                        > parent_rect.x_max
                    {
                        glyph_origin.set_x(parent_rect.x_min);
                        glyph_origin.set_y(glyph_origin.y() - new_line_screen_space_span)
                    }
                }

                // adding symbol
                symbols.push(Symbol {
                    character: presymbol.character,
                    colour: presymbol.colour,
                    region: Rectangle::new(
                        glyph_origin.x() + presymbol.symbol_metrics.x_shift,
                        glyph_origin.x()
                            + presymbol.symbol_metrics.x_shift
                            + presymbol.symbol_metrics.width,
                        glyph_origin.y() + presymbol.symbol_metrics.y_shift,
                        glyph_origin.y()
                            + presymbol.symbol_metrics.y_shift
                            + presymbol.symbol_metrics.height,
                    ),
                    uv_top_left: presymbol.uv_top_left,
                    uv_bottom_right: presymbol.uv_bottom_right,
                });

                // moving the origin accumulator
                glyph_origin[0] += presymbol.symbol_metrics.advance_width;
            }
            symbols
        };
    }
}

#[derive(Copy, Clone)]
pub struct Symbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The screen space region of a symbol
    pub region: Rectangle,
    /// The top left UV coordinate of the symbol
    pub uv_top_left: glam::Vec2,
    /// The bottom right UV coordinate of the symbol
    pub uv_bottom_right: glam::Vec2,
}

struct Presymbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The top left UV coordinate of the symbol
    pub uv_top_left: glam::Vec2,
    /// The bottom right UV coordinate of the symbol
    pub uv_bottom_right: glam::Vec2,
    /// The screen space dimension/attributes of the symbol
    pub symbol_metrics: SymbolMetrics,
}

/// The screen space metrics of a symbol
struct SymbolMetrics {
    pub width: f32,
    pub height: f32,
    pub x_shift: f32,
    pub y_shift: f32,
    pub advance_width: f32,
}
