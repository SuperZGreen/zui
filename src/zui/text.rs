mod glyph_origin;
mod text_configuration;
mod text_line;

use glyph_origin::GlyphOrigin;
use text_line::TextLines;

use winit::dpi::PhysicalSize;

use super::{
    font::{FontStyle, SymbolInfo, SymbolKey},
    primitives::Rectangle,
    text_renderer::TextVertex,
    Axis, Colour, Typeface,
};

pub use text_configuration::{
    LineWrapping, TextAlignmentHorizontal, TextAlignmentVertical, TextConfiguration, TextSize,
};

#[derive(Clone)]
pub struct TextSegment {
    pub string: String,
    pub colour: Colour,
    pub style: FontStyle,
}

impl TextSegment {
    /// TODO
    pub fn new(string: &str, colour: Colour) -> Self {
        Self {
            string: String::from(string),
            colour,
            style: FontStyle::Regular,
        }
    }
}

impl Default for TextSegment {
    fn default() -> Self {
        Self {
            string: String::from("Test"),
            colour: Colour::WHITE,
            style: FontStyle::Regular,
        }
    }
}

#[derive(Clone)]
/// Data that is calculated before symbols are placed onto the screen
struct TextLayout {
    pub presymbols: Vec<Presymbol>,
    pub lines: TextLines,
    pub viewport_dimensions_px: glam::IVec2,
}

impl TextLayout {
    pub fn new(
        presymbols: Vec<Presymbol>,
        lines: TextLines,
        viewport_dimensions_px: glam::IVec2,
    ) -> Self {
        Self {
            presymbols,
            lines,
            viewport_dimensions_px,
        }
    }
}

#[derive(Clone)]
pub struct Text {
    /// The actual content of the string
    pub segments: Vec<TextSegment>,

    /// The per-character rendering information of the text
    pub symbols: Vec<Symbol>,

    /// Includes alignment and styling information for the text
    pub configuration: TextConfiguration,

    /// The layout of the text, including the lines, presymbols and viewport pixel dimensions
    layout: Option<TextLayout>,
}

impl Text {
    /// A new empty Text object
    pub fn new() -> Text {
        Self {
            segments: Vec::new(),
            symbols: Vec::new(),
            configuration: TextConfiguration::default(),
            layout: None,
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

    /// Calculates the screen space dimensions of the text for a given clip rectangle, and generates
    /// presymbols
    pub fn fit_rectangle(
        &mut self,
        font: &Typeface,
        clip_rectangle: &Rectangle<f32>,
        _viewport_dimensions_px: PhysicalSize<u32>, // TODO: will be used for other text heigh calculations (?)
    ) {
        // converting the line metrics of Fontdue to screen space
        let font_metrics_px = PixelFontMetrics::new(
            &font
                .font_regular // TODO: this shouldn't just be regular, but what the TextStyle of the Text actually is
                .as_ref()
                .unwrap()
                .horizontal_line_metrics(
                    self.configuration
                        .size
                        .to_viewport_pixels(clip_rectangle.height()) as f32,
                )
                .unwrap(), // TODO
        );

        // calculating the screen space metrics of all symbols
        let presymbols = Self::generate_presymbols(
            &self,
            font,
            self.configuration
                .size
                .to_viewport_pixels(clip_rectangle.height()) as u32,
        );

        // getting lines
        let lines = TextLines::from_presymbols(
            &presymbols,
            clip_rectangle,
            &font_metrics_px,
            &self.configuration.line_wrapping,
        );

        // getting the dimensions of the lines
        let viewport_px_dimensions = lines.viewport_px_dimensions(&font_metrics_px);

        self.layout = Some(TextLayout::new(presymbols, lines, viewport_px_dimensions));
    }

    /// Updates/places/caluclates the symbol dimensions and locations from the Text's TextSegments,
    /// performing line wrapping, alignment etc
    pub fn place_symbols(&mut self, font: &Typeface, clip_rectangle: &Rectangle<f32>) {
        let layout = match &mut self.layout {
            Some(layout) => layout,
            None => return,
        };

        // converting the line metrics of Fontdue to screen space
        let font_metrics_px = PixelFontMetrics::new(
            &font
                .font_regular
                .as_ref() // TODO: this shouldn't just be regular, but what the TextStyle of the Text actually is
                .unwrap()
                .horizontal_line_metrics(
                    self.configuration
                        .size
                        .to_viewport_pixels(clip_rectangle.height()) as f32,
                )
                .unwrap(), // TODO
        );

        // placing symbols
        self.symbols.clear();

        // repositioning origin for vertical alignment
        let mut origin = match self.configuration.vertical_alignment {
            TextAlignmentVertical::Top => GlyphOrigin::at_top_left(clip_rectangle, &font_metrics_px),
            TextAlignmentVertical::Centre => {
                GlyphOrigin::at_centre_left(clip_rectangle, &font_metrics_px)
            }
            TextAlignmentVertical::Bottom => {
                GlyphOrigin::at_bottom_left(clip_rectangle, &font_metrics_px)
            }
        };

        for line in layout.lines.lines.iter() {
            // integrating text alignment
            let horizontal_offset = match self.configuration.horizontal_alignment {
                TextAlignmentHorizontal::Left => 0i32,
                TextAlignmentHorizontal::Centre => {
                    ((clip_rectangle.width() - line.viewport_px_dimensions.x as f32) / 2f32) as i32
                }
                TextAlignmentHorizontal::Right => {
                    clip_rectangle.width() as i32 - line.viewport_px_dimensions.x
                }
            };

            origin.viewport_px_position.x += horizontal_offset;
            for presymbol in &layout.presymbols[line.range.start..line.range.end] {
                self.symbols.push(origin.symbol_from_presymbol(presymbol));
                origin.increment_by_presymbol(presymbol);
            }
            origin.new_line(clip_rectangle, &font_metrics_px);
        }
    }

    /// Gives the screen space span of the text along a given axis
    pub fn span_px(&self, axis: Axis) -> Option<f32> {
        let axis_index = axis.to_index();
        if let Some(layout) = &self.layout {
            Some(layout.viewport_dimensions_px[axis_index] as f32)
        } else {
            None
        }
    }

    /// Produces a Vec of TextVertexs for the Symbols of a Text object
    pub fn to_vertices(
        &self,
        // The clipping region of the parent widget, can not render fragments outside of this rect.
        // Given in viewport pixels
        parent_clip_region: Rectangle<f32>,
        // The viewport dimensions in pixels, used to calculate the clip bounds for the text
        // fragment shader
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> Vec<TextVertex> {
        // the number of vertices produced by a symbol
        let vertices_per_symbol = 6usize;
        let mut vertices = Vec::with_capacity(self.symbols.len() * vertices_per_symbol);
        for symbol in self.symbols.iter() {
            vertices.extend_from_slice(
                &symbol.to_text_vertices(parent_clip_region, viewport_dimensions_px),
            )
        }

        vertices
    }

    fn generate_presymbols(&self, font: &Typeface, size_px: u32) -> Vec<Presymbol> {
        let mut ps = Vec::new();
        for segment in self.segments.iter() {
            let style = segment.style;

            for character in segment.string.chars() {
                let symbol_key = SymbolKey::new(character, style, size_px);

                let symbol_info = match font.get_symbol(symbol_key) {
                    Some(res) => res,
                    None => {
                        error!("could not find glyph for character: {}!", character);
                        continue;
                    }
                };

                ps.push(Presymbol {
                    character,
                    colour: segment.colour,
                    symbol_info,
                })
            }
        }

        ps
    }
}

#[derive(Copy, Clone)]
pub struct Symbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The viewport pixel region of a symbol
    pub region: Rectangle<f32>,
    /// The UV region of the symbol, using UV coordinates where (0, 0) is top left, (1, 1) is bottom
    /// right
    pub uv_region: Rectangle<f32>,
}

impl Symbol {
    /// Produces text vertices from the [`Symbol`]
    pub fn to_text_vertices(
        &self,
        parent_clip_region: Rectangle<f32>,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> [TextVertex; 6] {
        let symbol_rectangle = Rectangle::new(
            self.region.x_min as f32,
            self.region.x_max as f32,
            self.region.y_min as f32,
            self.region.y_max as f32,
        );

        let uv_top_left = glam::Vec2::new(self.uv_region.x_min, self.uv_region.y_min);
        let uv_top_right = glam::Vec2::new(self.uv_region.x_max, self.uv_region.y_min);
        let uv_bottom_left = glam::Vec2::new(self.uv_region.x_min, self.uv_region.y_max);
        let uv_bottom_right = glam::Vec2::new(self.uv_region.x_max, self.uv_region.y_max);

        let region_vertices = symbol_rectangle.vertices(viewport_dimensions_px);

        // for (index, vert) in region_vertices.iter().enumerate() {
        //     println!("{}: {}", index, vert);
        // }

        let a = TextVertex::new(
            region_vertices[0],
            uv_top_left,
            self.colour.into(),
            &parent_clip_region,
            viewport_dimensions_px,
        );
        let b = TextVertex::new(
            region_vertices[1],
            uv_top_right,
            self.colour.into(),
            &parent_clip_region,
            viewport_dimensions_px,
        );
        let c = TextVertex::new(
            region_vertices[2],
            uv_bottom_left,
            self.colour.into(),
            &parent_clip_region,
            viewport_dimensions_px,
        );
        let d = TextVertex::new(
            region_vertices[3],
            uv_bottom_right,
            self.colour.into(),
            &parent_clip_region,
            viewport_dimensions_px,
        );

        [
            a, c, b, //
            b, c, d, //
        ]
    }
}

#[derive(Clone)]
/// Holds the information required for placing a [`Symbol`], ie most things without the final position,
/// this makes it easier for text to be placed into lines, formatted etc.
pub struct Presymbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The symbol info of the presymbol, giving its metrics and uv region
    pub symbol_info: SymbolInfo,
}

/// The viewport pixel font/line metrics of a font at a certain size
pub struct PixelFontMetrics {
    /// The screen space span from ascent to descent
    pub height: i32,

    /// The ascent of the text in screen space
    pub ascent: i32,

    /// The descent of the text in screen space
    pub descent: i32,

    /// The gap between the descent of one line, and the ascent of the next, in screen space
    pub line_gap: i32,

    /// The distance between the baseline of one line, and the baseline of the next line, can be
    /// calculated as ascent - descent + line_gap TODO: this isn't actually true, new_line_size
    /// appears to be larger than that
    pub new_line_size: i32,
}

impl PixelFontMetrics {
    pub fn new(line_metrics: &fontdue::LineMetrics) -> Self {
        Self {
            height: line_metrics.ascent as i32 - line_metrics.descent as i32,
            ascent: line_metrics.ascent as i32,
            descent: line_metrics.descent as i32,
            line_gap: line_metrics.line_gap as i32,
            new_line_size: line_metrics.new_line_size as i32,
        }
    }
}
