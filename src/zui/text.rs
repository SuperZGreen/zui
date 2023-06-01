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
    Pixels(i32),
}

impl TextSize {
    /// Gives the text height in terms of screen space
    pub fn to_viewport_pixels(&self, parent_rectangle_height: f32) -> i32 {
        match self {
            TextSize::ParentHeight(proportion) => (proportion * parent_rectangle_height) as i32,
            TextSize::Pixels(pixels) => *pixels as i32,
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
        font_metrics_px: &PixelFontMetrics,
    ) -> Self {

        // flooring the max of the clip rect seems to help against jitter
        let mut clip_rect = clip_rectangle.clone();
        // clip_rect.x_max = clip_rect.x_max.floor();

        let mut lines = Vec::new();

        if presymbols.len() > 0 {
            let mut origin = GlyphOrigin::at_top_left(&clip_rect, font_metrics_px);
            let mut line_start_index = 0usize;
            let mut line_min_x = origin.viewport_px_position.x;
            let mut line_max_x = origin.viewport_px_position.x;

            for (index, presymbol) in presymbols.iter().enumerate() {
                if origin.presymbol_fits_in_rect(&clip_rect, presymbol) {
                    // tracking the new max width of the line
                    line_max_x = origin.viewport_px_position.x
                        + presymbol.symbol_metrics.x_shift
                        + presymbol.symbol_metrics.width;
                } else {
                    // resetting the origin to the start of the line
                    origin = GlyphOrigin::at_top_left(&clip_rect, font_metrics_px);

                    // calculating the new line x min
                    line_min_x = origin.viewport_px_position.x;

                    // pushing the line
                    let line_end_index = index.checked_sub(1).unwrap_or(0);
                    lines.push(TextLine::new(
                        line_start_index..=line_end_index,
                        glam::IVec2::new(line_max_x - line_min_x, font_metrics_px.height),
                    ));
                    line_start_index = index;
                }

                origin.increment_by_presymbol(presymbol);
            }

            // pushing final line
            lines.push(TextLine::new(
                line_start_index..=presymbols.len() - 1,
                glam::IVec2::new(line_max_x - line_min_x, font_metrics_px.height),
            ));
        }

        Self { lines }
    }

    pub fn viewport_px_dimensions(&self, font_metrics_ss: &PixelFontMetrics) -> glam::IVec2 {
        // lines max width
        let mut width = 0i32;
        for line in self.lines.iter() {
            if line.viewport_px_dimensions.x > width {
                width = line.viewport_px_dimensions.x;
            }
        }

        // lines height
        let mut height = 0i32;
        if let Some(first_line) = self.lines.first() {
            height += first_line.viewport_px_dimensions.y;

            for line in &self.lines[1..self.lines.len()] {
                height += line.viewport_px_dimensions.y + font_metrics_ss.line_gap;
            }
        }

        glam::IVec2::new(width, height)
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
        _viewport_dimensions_px: PhysicalSize<u32>,
    ) {
        // converting the line metrics of Fontdue to screen space
        let font_metrics_px = PixelFontMetrics::new(
            self.configuration
                .size
                .to_viewport_pixels(clip_rectangle.height()),
            &font.line_metrics,
        );

        // calculating the screen space metrics of all symbols
        let presymbols = Self::generate_presymbols(&self, font, &font_metrics_px, aspect_ratio);

        // getting lines
        let lines = TextLines::from_presymbols(&presymbols, clip_rectangle, &font_metrics_px);

        // getting the dimensions of the lines
        let screen_space_dimensions = lines.viewport_px_dimensions(&font_metrics_px);

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
        let font_metrics_ss = PixelFontMetrics::new(
            self.configuration
                .size
                .to_viewport_pixels(parent_rect.height()),
            &font.line_metrics,
        );

        // placing symbols
        self.symbols.clear();
        let mut origin = GlyphOrigin::at_top_left(parent_rect, &font_metrics_ss);

        // repositioning origin for vertical alignment
        origin.viewport_px_position.y -= match self.configuration.vertical_alignment {
            TextAlignmentVertical::Top => 0i32,
            TextAlignmentVertical::Centre => {
                ((parent_rect.height() - layout.viewport_dimensions_px.y as f32) / 2f32) as i32
            }
            TextAlignmentVertical::Bottom => {
                parent_rect.height() as i32 - layout.viewport_dimensions_px.y
            }
        };

        for line in layout.lines.lines.iter() {
            // integrating text alignment
            let horizontal_offset = match self.configuration.horizontal_alignment {
                TextAlignmentHorizontal::Left => 0i32,
                TextAlignmentHorizontal::Centre => {
                    ((parent_rect.width() - line.viewport_px_dimensions.x as f32) / 2f32) as i32
                }
                TextAlignmentHorizontal::Right => {
                    parent_rect.width() as i32 - line.viewport_px_dimensions.x
                }
            };

            origin.viewport_px_position.x += horizontal_offset;
            for presymbol in &layout.presymbols[*line.range.start()..=*line.range.end()] {
                self.symbols.push(origin.symbol_from_presymbol(presymbol));
                origin.increment_by_presymbol(presymbol);
            }
            origin.new_line(parent_rect, &font_metrics_ss);
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

    fn generate_presymbols(
        &self,
        font: &Font,
        font_metrics_ss: &PixelFontMetrics,
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

                // let symbol_metrics = ScreenSpaceSymbolMetrics::new(
                //     &info.metrics,
                //     &font.line_metrics,
                //     &font_metrics_ss,
                //     aspect_ratio,
                // );

                ps.push(Presymbol {
                    character,
                    colour: segment.colour,
                    uv_region,
                    symbol_metrics: PixelSymbolMetrics::new(&info.metrics),
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
struct Presymbol {
    /// The character that a symbol represents
    pub character: char,
    /// The colour of a character
    pub colour: Colour,
    /// The UV region of the symbol, using UV coordinates where (0, 0) is top left, (1, 1) is bottom
    /// right
    pub uv_region: Rectangle<f32>,
    /// The screen space dimension/attributes of the symbol
    pub symbol_metrics: PixelSymbolMetrics,
}

#[derive(Clone)]
/// The screen space metrics of a symbol
struct PixelSymbolMetrics {
    pub width: i32,
    pub height: i32,
    pub x_shift: i32,
    pub y_shift: i32,
    pub advance_width: i32,
}

impl PixelSymbolMetrics {
    /// Generates the screen space metrics for a symbol given its pixel metrics generated from
    /// fontdue.
    pub fn new(metrics: &fontdue::Metrics) -> Self {
        Self {
            width: metrics.width as i32,
            height: metrics.height as i32,
            x_shift: metrics.xmin as i32,
            y_shift: metrics.ymin as i32,
            advance_width: metrics.advance_width as i32,
        }
    }
}

/// The screen space line metrics of a font at a certain size
struct PixelFontMetrics {
    /// The screen space span from ascent to descent
    pub height: i32,

    /// The ascent of the text in screen space
    pub ascent: i32,

    /// The descent of the text in screen space
    pub descent: i32,

    /// The gap between the descent of one line, and the ascent of the next, in screen space
    pub line_gap: i32,

    /// The distance between the baseline of one line, and the baseline of the next line, can be
    /// calculated as ascent - descent + line_gap
    pub new_line_size: i32,
}

impl PixelFontMetrics {
    pub fn new(text_height_px: i32, pixel_metrics: &fontdue::LineMetrics) -> Self {
        let rasterised_font_height_px = pixel_metrics.ascent - pixel_metrics.descent;
        // let rasterised_font_width_px = rasterised_font_height_px * aspect_ratio;

        Self {
            height: text_height_px,
            ascent: pixel_metrics.ascent as i32,
            descent: pixel_metrics.descent as i32,
            line_gap: pixel_metrics.line_gap as i32,
            new_line_size: pixel_metrics.new_line_size as i32,
        }
    }
}

#[derive(Clone)]
/// Represents a line of text as it appears on screen, can contain either Presymbols or Symbols
struct TextLine {
    /// The symbols that are part of the line
    pub range: RangeInclusive<usize>,

    /// The width of the line
    pub viewport_px_dimensions: glam::IVec2,
}

impl TextLine {
    fn new(range: RangeInclusive<usize>, pixel_dimensions: glam::IVec2) -> Self {
        Self {
            range,
            viewport_px_dimensions: pixel_dimensions,
        }
    }
}

/// The origin accumulator that tracks where to place the next [`Symbol`]
struct GlyphOrigin {
    viewport_px_position: glam::IVec2,
}

impl GlyphOrigin {
    /// Places the origin at the top left of the rectangle
    pub fn at_top_left(clip_region: &Rectangle<f32>, font_metrics_px: &PixelFontMetrics) -> Self {
        let screen_space_position = glam::IVec2::new(
            clip_region.x_min as i32,
            clip_region.y_max as i32 - font_metrics_px.ascent,
        );

        Self {
            viewport_px_position: screen_space_position,
        }
    }

    // /// Places at any location
    // pub fn at(screen_space_position: glam::Vec2) -> Self {
    //     Self {
    //         viewport_px_position: screen_space_position,
    //     }
    // }

    /// Returns true iff the Presymbol will fit in the provided clipping Rectangle
    fn presymbol_fits_in_rect(
        &self,
        clip_rectangle: &Rectangle<f32>,
        presymbol: &Presymbol,
    ) -> bool {
        ((self.viewport_px_position.x
            + presymbol.symbol_metrics.width
            + presymbol.symbol_metrics.x_shift) as f32)
            < clip_rectangle.x_max
    }

    /// Moves the GlyphOrigin by the Presymbol, as though placing the symbol
    fn increment_by_presymbol(&mut self, presymbol: &Presymbol) {
        self.viewport_px_position.x += presymbol.symbol_metrics.advance_width;
    }

    /// Gives the symbol for a presymbol at the GlyphOrigin's location
    fn symbol_from_presymbol(&self, presymbol: &Presymbol) -> Symbol {
        Symbol {
            character: presymbol.character,
            colour: presymbol.colour,
            region: Rectangle::new(
                (self.viewport_px_position.x + presymbol.symbol_metrics.x_shift) as f32,
                (self.viewport_px_position.x
                    + presymbol.symbol_metrics.x_shift
                    + presymbol.symbol_metrics.width) as f32,
                (self.viewport_px_position.y + presymbol.symbol_metrics.y_shift) as f32,
                (self.viewport_px_position.y
                    + presymbol.symbol_metrics.y_shift
                    + presymbol.symbol_metrics.height) as f32,
            ),
            uv_region: presymbol.uv_region,
        }
    }

    /// Resets the glyph origin to the left of the parent rectangle and moves downward for a new
    /// line
    fn new_line(&mut self, clip_rectangle: &Rectangle<f32>, font_metrics_px: &PixelFontMetrics) {
        self.viewport_px_position.x = clip_rectangle.x_min as i32;
        self.viewport_px_position.y -= font_metrics_px.new_line_size;
    }
}
