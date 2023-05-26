use std::ops::RangeInclusive;

use winit::dpi::PhysicalSize;

use super::{primitives::Rectangle, text_renderer::TextVertex, Axis, Colour, Font};

#[derive(Clone)]
#[allow(dead_code)]
pub enum LineWrapping {
    None,
    Symbol,
    Word,
}

#[derive(Clone)]
/// Used to derive height of the text, ascent to descent
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

#[derive(Copy, Clone)]
pub enum TextAlignmentHorizontal {
    Left,
    Centre,
    Right,
}

#[derive(Copy, Clone)]
pub enum TextAlignmentVertical {
    Top,
    Centre,
    Bottom,
}

#[derive(Clone)]
pub struct TextConfiguration {
    pub line_wrapping: LineWrapping,
    pub size: TextSize,
    pub horizontal_alignment: TextAlignmentHorizontal,
    pub vertical_alignment: TextAlignmentVertical,
}

impl Default for TextConfiguration {
    fn default() -> Self {
        Self {
            line_wrapping: LineWrapping::Symbol,
            size: TextSize::ParentHeight(1f32),
            horizontal_alignment: TextAlignmentHorizontal::Left,
            vertical_alignment: TextAlignmentVertical::Top,
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
struct TextLines {
    pub lines: Vec<TextLine>,
}

impl TextLines {
    pub fn from_presymbols(
        presymbols: &[Presymbol],
        clip_rectangle: &Rectangle<f32>,
        font_metrics_ss: &ScreenSpaceFontMetrics,
    ) -> Self {
        let mut lines = Vec::new();

        if presymbols.len() > 0 {
            let mut origin = GlyphOrigin::at_top_left(clip_rectangle, font_metrics_ss);
            let mut line_start_index = 0usize;
            let mut line_min_x = origin.screen_space_position.x;
            let mut line_max_x = origin.screen_space_position.x;

            for (index, presymbol) in presymbols.iter().enumerate() {
                if origin.presymbol_fits_in_rect(clip_rectangle, presymbol) {
                    // tracking the new max width of the line
                    line_max_x = origin.screen_space_position.x
                        + presymbol.symbol_metrics.x_shift
                        + presymbol.symbol_metrics.width;
                } else {
                    // resetting the origin to the start of the line
                    origin = GlyphOrigin::at_top_left(clip_rectangle, font_metrics_ss);

                    // calculating the new line x min
                    line_min_x = origin.screen_space_position.x;

                    // pushing the line
                    let line_end_index = index.checked_sub(1).unwrap_or(0);
                    lines.push(TextLine::new(
                        line_start_index..=line_end_index,
                        glam::Vec2::new(line_max_x - line_min_x, font_metrics_ss.height),
                    ));
                    line_start_index = index;
                }

                origin.increment_by_presymbol(presymbol);
            }

            // pushing final line
            lines.push(TextLine::new(
                line_start_index..=presymbols.len() - 1,
                glam::Vec2::new(line_max_x - line_min_x, font_metrics_ss.height),
            ));
        }

        Self { lines }
    }

    pub fn screen_space_dimensions(&self, font_metrics_ss: &ScreenSpaceFontMetrics) -> glam::Vec2 {
        // lines max width
        let mut width = 0f32;
        for line in self.lines.iter() {
            if line.screen_space_dimensions.x > width {
                width = line.screen_space_dimensions.x;
            }
        }

        // lines height
        let mut height = 0f32;
        if let Some(first_line) = self.lines.first() {
            height += first_line.screen_space_dimensions.y;

            for line in &self.lines[1..self.lines.len()] {
                height += line.screen_space_dimensions.y + font_metrics_ss.line_gap;
            }
        }

        glam::Vec2::new(width, height)
    }
}

#[derive(Clone)]
/// Data that is calculated before symbols are placed onto the screen
struct TextLayout {
    pub presymbols: Vec<Presymbol>,
    pub lines: TextLines,
    pub screen_space_dimensions: glam::Vec2,
}

impl TextLayout {
    pub fn new(
        presymbols: Vec<Presymbol>,
        lines: TextLines,
        screen_space_dimensions: glam::Vec2,
    ) -> Self {
        Self {
            presymbols,
            lines,
            screen_space_dimensions,
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

    /// Calculates the screen space dimensions of the text for a given clip rectangle
    pub fn update_layout(
        &mut self,
        font: &Font,
        clip_rectangle: &Rectangle<f32>,
        aspect_ratio: f32,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) {
        // converting the line metrics of Fontdue to screen space
        let font_metrics_ss = ScreenSpaceFontMetrics::new(
            self.configuration
                .size
                .to_screen_space_span(clip_rectangle.height(), viewport_dimensions_px.height),
            &font.line_metrics,
        );

        // calculating the screen space metrics of all symbols
        let presymbols = Self::generate_presymbols(&self, font, &font_metrics_ss, aspect_ratio);

        // getting lines
        let lines = TextLines::from_presymbols(&presymbols, clip_rectangle, &font_metrics_ss);

        // getting the dimensions of the lines
        let screen_space_dimensions = lines.screen_space_dimensions(&font_metrics_ss);

        self.layout = Some(TextLayout::new(presymbols, lines, screen_space_dimensions));
    }

    /// Updates/places/caluclates the symbol dimensions and locations from the Text's TextSegments,
    /// performing line wrapping, alignment etc
    pub fn place_symbols(
        &mut self,
        font: &Font,
        parent_rect: &Rectangle<f32>,
        aspect_ratio: f32,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) {
        let layout = match &mut self.layout {
            Some(layout) => layout,
            None => return,
        };

        // converting the line metrics of Fontdue to screen space
        let font_metrics_ss = ScreenSpaceFontMetrics::new(
            self.configuration
                .size
                .to_screen_space_span(parent_rect.height(), viewport_dimensions_px.height),
            &font.line_metrics,
        );

        // placing symbols
        self.symbols.clear();
        let mut origin = GlyphOrigin::at_top_left(parent_rect, &font_metrics_ss);
        
        // repositioning origin for vertical alignment
        origin.screen_space_position.y -= match self.configuration.vertical_alignment {
            TextAlignmentVertical::Top => 0f32,
            TextAlignmentVertical::Centre => (parent_rect.height() - layout.screen_space_dimensions.y) / 2f32,
            TextAlignmentVertical::Bottom => parent_rect.height() - layout.screen_space_dimensions.y,
        };

        for line in layout.lines.lines.iter() {
            // integrating text alignment
            let horizontal_offset = match self.configuration.horizontal_alignment {
                TextAlignmentHorizontal::Left => 0f32,
                TextAlignmentHorizontal::Centre => {
                    (parent_rect.width() - line.screen_space_dimensions.x) / 2f32
                }
                TextAlignmentHorizontal::Right => {
                    parent_rect.width() - line.screen_space_dimensions.x
                }
            };

            origin.screen_space_position.x += horizontal_offset;
            for presymbol in &layout.presymbols[*line.range.start()..=*line.range.end()] {
                self.symbols.push(origin.symbol_from_presymbol(presymbol));
                origin.increment_by_presymbol(presymbol);
            }
            origin.new_line(parent_rect, &font_metrics_ss);
        }
    }

    /// Gives the screen space span of the text along a given axis
    pub fn screen_space_span(&self, axis: Axis) -> Option<f32> {
        let axis_index = axis.to_index();
        if let Some(layout) = &self.layout {
            Some(layout.screen_space_dimensions[axis_index])
        } else {
            None
        }
    }

    /// Produces a Vec of TextVertexs for the Symbols of a Text object
    pub fn to_vertices(
        &self,
        // The clipping region of the parent widget, can not render fragments outside of this rect.
        // Given in NDC/screen space
        parent_clip_region: Rectangle<f32>,
        // The viewport dimensions in pixels, used to calculate the clip bounds for the text
        // fragment shader
        viewport_dimensions_px: glam::Vec2,
    ) -> Vec<TextVertex> {
        // the number of vertices produced by a symbol
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

    fn generate_presymbols(
        &self,
        font: &Font,
        font_metrics_ss: &ScreenSpaceFontMetrics,
        aspect_ratio: f32,
    ) -> Vec<Presymbol> {
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

                let symbol_metrics = ScreenSpaceSymbolMetrics::new(
                    &info.metrics,
                    &font.line_metrics,
                    &font_metrics_ss,
                    aspect_ratio,
                );

                ps.push(Presymbol {
                    character,
                    colour: segment.colour,
                    uv_region,
                    symbol_metrics,
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
    /// The screen space region of a symbol
    pub region: Rectangle<f32>,
    /// The UV region of the symbol, using UV coordinates where (0, 0) is top left, (1, 1) is bottom
    /// right
    pub uv_region: Rectangle<f32>,
}

#[derive(Clone)]
/// Holds the information required for placing a [`Symbol`], ie most things without the final position,
/// this makes it easier for text to be placed into lines, formatted etc.
struct Presymbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The UV region of the symbol, using UV coordinates where (0, 0) is top left, (1, 1) is bottom
    /// right
    pub uv_region: Rectangle<f32>,
    /// The screen space dimension/attributes of the symbol
    pub symbol_metrics: ScreenSpaceSymbolMetrics,
}

#[derive(Clone)]
/// The screen space metrics of a symbol
struct ScreenSpaceSymbolMetrics {
    pub width: f32,
    pub height: f32,
    pub x_shift: f32,
    pub y_shift: f32,
    pub advance_width: f32,
}

impl ScreenSpaceSymbolMetrics {
    /// Generates the screen space metrics for a symbol given its pixel metrics generated from
    /// fontdue.
    pub fn new(
        metrics_px: &fontdue::Metrics,
        line_metrics_px: &fontdue::LineMetrics,
        font_metrics_ss: &ScreenSpaceFontMetrics,
        aspect_ratio: f32,
    ) -> Self {
        let text_height_px = line_metrics_px.ascent - line_metrics_px.descent;
        let equivalent_horizontal_height_px = text_height_px * aspect_ratio;

        Self {
            width: metrics_px.width as f32 / equivalent_horizontal_height_px
                * font_metrics_ss.height,
            height: metrics_px.height as f32 / text_height_px * font_metrics_ss.height,
            x_shift: metrics_px.xmin as f32 / equivalent_horizontal_height_px
                * font_metrics_ss.height,
            y_shift: metrics_px.ymin as f32 / text_height_px * font_metrics_ss.height,
            advance_width: metrics_px.advance_width as f32 / equivalent_horizontal_height_px
                * font_metrics_ss.height,
        }
    }
}

/// The screen space line metrics of a font at a certain size
struct ScreenSpaceFontMetrics {
    /// The screen space span from ascent to descent
    pub height: f32,

    /// The ascent of the text in screen space
    pub ascent: f32,

    /// The descent of the text in screen space
    pub descent: f32,

    /// The gap between the descent of one line, and the ascent of the next, in screen space
    pub line_gap: f32,

    /// The distance between the baseline of one line, and the baseline of the next line, can be
    /// calculated as ascent - descent + line_gap
    pub new_line_size: f32,
}

impl ScreenSpaceFontMetrics {
    pub fn new(text_height_screen_space: f32, pixel_metrics: &fontdue::LineMetrics) -> Self {
        let rasterised_font_height_px = pixel_metrics.ascent - pixel_metrics.descent;
        // let rasterised_font_width_px = rasterised_font_height_px * aspect_ratio;

        Self {
            height: text_height_screen_space,
            ascent: pixel_metrics.ascent / rasterised_font_height_px * text_height_screen_space,
            descent: pixel_metrics.descent / rasterised_font_height_px * text_height_screen_space,
            line_gap: pixel_metrics.line_gap / rasterised_font_height_px * text_height_screen_space,
            new_line_size: pixel_metrics.new_line_size / rasterised_font_height_px
                * text_height_screen_space,
        }
    }
}

#[derive(Clone)]
/// Represents a line of text as it appears on screen, can contain either Presymbols or Symbols
struct TextLine {
    /// The symbols that are part of the line
    pub range: RangeInclusive<usize>,

    /// The width of the line
    pub screen_space_dimensions: glam::Vec2,
}

impl TextLine {
    fn new(range: RangeInclusive<usize>, screen_space_dimensions: glam::Vec2) -> Self {
        Self {
            range,
            screen_space_dimensions,
        }
    }
}

/// The origin accumulator that tracks where to place the next [`Symbol`]
struct GlyphOrigin {
    screen_space_position: glam::Vec2,
}

impl GlyphOrigin {
    /// Places the origin at the top left of the rectangle
    pub fn at_top_left(clip_region: &Rectangle<f32>, font_metrics_ss: &ScreenSpaceFontMetrics) -> Self {
        let screen_space_position = glam::Vec2::new(
            clip_region.x_min,
            clip_region.y_max - font_metrics_ss.ascent,
        );

        Self {
            screen_space_position,
        }
    }

    /// Places at any location
    pub fn at(screen_space_position: glam::Vec2) -> Self {
        Self {
            screen_space_position,
        }
    }

    /// Returns true iff the Presymbol will fit in the provided clipping Rectangle
    fn presymbol_fits_in_rect(&self, clip_rectangle: &Rectangle<f32>, presymbol: &Presymbol) -> bool {
        self.screen_space_position.x
            + presymbol.symbol_metrics.width
            + presymbol.symbol_metrics.x_shift
            < clip_rectangle.x_max
    }

    /// Moves the GlyphOrigin by the Presymbol, as though placing the symbol
    fn increment_by_presymbol(&mut self, presymbol: &Presymbol) {
        self.screen_space_position.x += presymbol.symbol_metrics.advance_width;
    }

    /// Gives the symbol for a presymbol at the GlyphOrigin's location
    fn symbol_from_presymbol(&self, presymbol: &Presymbol) -> Symbol {
        Symbol {
            character: presymbol.character,
            colour: presymbol.colour,
            region: Rectangle::new(
                self.screen_space_position.x + presymbol.symbol_metrics.x_shift,
                self.screen_space_position.x
                    + presymbol.symbol_metrics.x_shift
                    + presymbol.symbol_metrics.width,
                self.screen_space_position.y + presymbol.symbol_metrics.y_shift,
                self.screen_space_position.y
                    + presymbol.symbol_metrics.y_shift
                    + presymbol.symbol_metrics.height,
            ),
            uv_region: presymbol.uv_region,
        }
    }

    /// Resets the glyph origin to the left of the parent rectangle and moves downward for a new
    /// line
    fn new_line(&mut self, clip_rectangle: &Rectangle<f32>, font_metrics_ss: &ScreenSpaceFontMetrics) {
        self.screen_space_position.x = clip_rectangle.x_min;
        self.screen_space_position.y -= font_metrics_ss.new_line_size;
    }
}
