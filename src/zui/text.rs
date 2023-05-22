use winit::dpi::PhysicalSize;

use super::{primitives::Rectangle, text_renderer::TextVertex, Colour, Font};

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
    /// Height of the text in screen space units
    ScreenSpace(f32),
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
            TextSize::Pixels(pixels) => pixels / viewport_height_px as f32 * 2f32,
            TextSize::ScreenSpace(ss) => *ss,
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
            size: TextSize::ParentHeight(1f32),
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
    pub fn push_segment(mut self, segment: TextSegment) -> Self {
        self.segments.push(segment);
        self
    }

    pub fn with_configuration(mut self, configuration: TextConfiguration) -> Self {
        self.configuration = configuration;
        self
    }

    /// Updates/places/caluclates the symbol dimensions and locations from the Text's TextSegments,
    /// performing line wrapping, alignment etc
    pub fn place_symbols(
        &mut self,
        font: &Font,
        parent_rect: &Rectangle,
        aspect_ratio: f32,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) {
        // the screen space height of a symbol
        // TODO, remove fixed pixels size
        let text_height_screen_space = self
            .configuration
            .size
            .to_screen_space_span(parent_rect.height(), viewport_dimensions_px.height);
        // println!("viewport height: {}", viewport_dimensions_px.height);

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
                    let (info, uv_region) = match font.get_symbol(character) {
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
                        uv_region,
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
                    if glyph_origin.x
                        + presymbol.symbol_metrics.width
                        + presymbol.symbol_metrics.x_shift
                        > parent_rect.x_max
                    {
                        glyph_origin.x = parent_rect.x_min;
                        glyph_origin.y = glyph_origin.y - new_line_screen_space_span;
                    }
                }

                // adding symbol
                symbols.push(Symbol {
                    character: presymbol.character,
                    colour: presymbol.colour,
                    region: Rectangle::new(
                        glyph_origin.x + presymbol.symbol_metrics.x_shift,
                        glyph_origin.x
                            + presymbol.symbol_metrics.x_shift
                            + presymbol.symbol_metrics.width,
                        glyph_origin.y + presymbol.symbol_metrics.y_shift,
                        glyph_origin.y
                            + presymbol.symbol_metrics.y_shift
                            + presymbol.symbol_metrics.height,
                    ),
                    uv_region: presymbol.uv_region,
                });

                // moving the origin accumulator
                glyph_origin[0] += presymbol.symbol_metrics.advance_width;
            }
            symbols
        };
    }

    /// Produces a Vec of TextVertexs for the Symbols of a Text object
    pub fn to_vertices(
        &self,
        // The clipping region of the parent widget, can not render fragments outside of this rect. Given in NDC/screen space
        parent_clip_region: Rectangle,
        // The viewport dimensions in pixels, used to calculate the clip bounds for the text fragment shader
        viewport_dimensions_px: glam::Vec2,
    ) -> Vec<TextVertex> {
        //the number of vertices produced by a symbol
        let vertices_per_symbol = 6usize;
        let mut vertices = Vec::with_capacity(self.symbols.len() * vertices_per_symbol);
        for symbol in self.symbols.iter() {
            let region_vertices = symbol.region.vertices();

            // println!("rect: {:?}", symbol.region);

            let uv_top_left = glam::Vec2::new(symbol.uv_region.x_min, symbol.uv_region.y_min);
            let uv_top_right = glam::Vec2::new(symbol.uv_region.x_max, symbol.uv_region.y_min);
            let uv_bottom_left = glam::Vec2::new(symbol.uv_region.x_min, symbol.uv_region.y_max);
            let uv_bottom_right = glam::Vec2::new(symbol.uv_region.x_max, symbol.uv_region.y_max);

            let a = TextVertex::new(
                region_vertices[0],
                uv_top_left,
                symbol.colour.into(),
                &parent_clip_region,
                viewport_dimensions_px,
            );
            let b = TextVertex::new(
                region_vertices[1],
                uv_top_right,
                symbol.colour.into(),
                &parent_clip_region,
                viewport_dimensions_px,
            );
            let c = TextVertex::new(
                region_vertices[2],
                uv_bottom_left,
                symbol.colour.into(),
                &parent_clip_region,
                viewport_dimensions_px,
            );
            let d = TextVertex::new(
                region_vertices[3],
                uv_bottom_right,
                symbol.colour.into(),
                &parent_clip_region,
                viewport_dimensions_px,
            );

            vertices.push(a);
            vertices.push(c);
            vertices.push(b);

            vertices.push(b);
            vertices.push(c);
            vertices.push(d);
        }

        vertices
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
    /// The UV region of the symbol, using UV coordinates where (0, 0) is top left, (1, 1) is bottom
    /// right
    pub uv_region: Rectangle,
}

struct Presymbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The UV region of the symbol, using UV coordinates where (0, 0) is top left, (1, 1) is bottom
    /// right
    pub uv_region: Rectangle,
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
