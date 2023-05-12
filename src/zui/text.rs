use super::{primitives::Rectangle, Colour, Font};

#[derive(Clone)]
#[allow(dead_code)]
pub enum LineWrapping {
    None,
    Letter,
    Word,
}

#[derive(Clone)]
pub struct TextConfiguration {
    pub line_wrapping: LineWrapping,
}

impl Default for TextConfiguration {
    fn default() -> Self {
        Self {
            line_wrapping: LineWrapping::Letter,
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
        let scale_factor = 0.001f32;
        let mut glyph_origin = glam::Vec2::new(
            parent_rect.x_min,
            parent_rect.y_max - font.line_metrics.ascent * scale_factor,
        );

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

                let symbol_width = symbol_metrics.width as f32 * scale_factor / aspect_ratio;
                let symbol_height = symbol_metrics.height as f32 * scale_factor;
                let symbol_x_shift = symbol_metrics.xmin as f32 * scale_factor / aspect_ratio;
                let symbol_y_shift = symbol_metrics.ymin as f32 * scale_factor;
                let symbol_advance_width =
                    symbol_metrics.advance_width * scale_factor / aspect_ratio;

                // line wrapping
                if matches!(self.configuration.line_wrapping, LineWrapping::Letter) {
                    if glyph_origin.x() + symbol_width + symbol_x_shift > parent_rect.x_max {
                        let screen_space_new_line_distance =
                            font.line_metrics.new_line_size * scale_factor;
                        glyph_origin.set_x(parent_rect.x_min);
                        glyph_origin.set_y(glyph_origin.y() - screen_space_new_line_distance)
                    }
                }

                self.symbols.push(Symbol {
                    character,
                    colour: segment.colour,
                    region: Rectangle::new(
                        glyph_origin.x() + symbol_x_shift,
                        glyph_origin.x() + symbol_x_shift + symbol_width,
                        glyph_origin.y() + symbol_y_shift,
                        glyph_origin.y() + symbol_y_shift + symbol_height,
                    ),
                    uv_top_left,
                    uv_bottom_right,
                });

                glyph_origin[0] += symbol_advance_width;
            }
        }
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

impl Symbol {
    // pub fn new(character: char, screen_space_position: glam::Vec2, font: &Font) -> Self {
    //     let (symbol_info, uv_top_left, uv_bottom_right) = match font.get_symbol(character) {
    //         Some(si) => si,
    //         None => {
    //             error!("could not retrieve symbol info for: {}", character);
    //             panic!();
    //         }
    //     };

    //     Self {
    //         character,
    //         symbol_region: None,
    //         uv_top_left,
    //         uv_bottom_right,
    //     }
    // }
}
