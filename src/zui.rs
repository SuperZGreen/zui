mod font;
pub mod premade_widgets;
mod primitives;
mod renderer;
mod scene;
mod text_renderer;
mod text;
mod texture_atlas;
pub mod util;
mod widget;
// mod tree;

pub use font::Font;
pub use primitives::ScreenSpacePosition;
use renderer::Renderer;
pub use scene::{Scene, SceneHandle};
use text_renderer::TextRenderer;
pub use widget::{Axis, Colour, Span, Widget};
use winit::dpi::PhysicalPosition;

use self::{primitives::Rectangle, scene::Renderable, text_renderer::TextVertex};

pub struct Zui {
    font: Font,
    renderer: Renderer,
    text_renderer: TextRenderer,

    width_px: u32,
    height_px: u32,
    aspect_ratio: f32,

    cursor_state: Option<ScreenSpacePosition>,
}

impl Zui {
    pub fn new(
        file: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_configuration: &wgpu::SurfaceConfiguration,
        width_px: u32,
        height_px: u32,
    ) -> Result<Self, ()> {
        let font_default = match Font::new(file, 128, device, queue) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let text_renderer = TextRenderer::new(
            device,
            surface_configuration,
            font_default.texture_atlas.bind_group_layout(),
        );

        Ok(Self {
            font: font_default,
            renderer: Renderer::new(device, surface_configuration),
            text_renderer,
            width_px,
            height_px,
            aspect_ratio: width_px as f32 / height_px as f32,
            cursor_state: None,
        })
    }

    /// Adds the vertices for a symbol to text_vertices, returns the offset of the rendering position
    fn add_symbol(
        font: &Font,
        text_vertices: &mut Vec<TextVertex>,
        character: char,
        location: glam::Vec2,
        scale: f32,
    ) -> glam::Vec2 {
        let (symbol_info, top_left, bottom_right) = font.get_symbol(character).unwrap();

        let metrics = &symbol_info.metrics;

        let hspan = 0.5f32;
        let text_verts = [
            TextVertex::new(
                glam::Vec2::new(
                    (metrics.xmin) as f32 * scale + location.x(),
                    (metrics.ymin + metrics.height as i32) as f32 * scale + location.y(),
                ),
                glam::vec2(top_left.x(), top_left.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(
                    (metrics.xmin + metrics.width as i32) as f32 * scale + location.x(),
                    (metrics.ymin + metrics.height as i32) as f32 * scale + location.y(),
                ),
                glam::vec2(bottom_right.x(), top_left.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(
                    (metrics.xmin) as f32 * scale + location.x(),
                    (metrics.ymin) as f32 * scale + location.y(),
                ),
                glam::vec2(top_left.x(), bottom_right.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(
                    (metrics.xmin + metrics.width as i32) as f32 * scale + location.x(),
                    (metrics.ymin) as f32 * scale + location.y(),
                ),
                glam::vec2(bottom_right.x(), bottom_right.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
        ];

        text_vertices.push(text_verts[0]);
        text_vertices.push(text_verts[2]);
        text_vertices.push(text_verts[1]);

        text_vertices.push(text_verts[1]);
        text_vertices.push(text_verts[2]);
        text_vertices.push(text_verts[3]);

        glam::Vec2::new(
            metrics.advance_width * scale,
            metrics.advance_height * scale,
        )

    }

    fn generate_text_vertices(font: &Font, text: &str, starting_position: glam::Vec2, scale: f32) -> Vec<TextVertex> {
        let mut text_vertices = Vec::new();
        let mut current_position = starting_position;

        for character in text.chars() {
            let offset = Self::add_symbol(font, &mut text_vertices, character, current_position, scale);
            current_position += offset
        }
        
        text_vertices
    }

    /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    pub fn upload_vertices(&mut self, device: &wgpu::Device, renderable: &dyn Renderable) {
        let (simple_vertices, text_vertices) = renderable.to_vertices();

        // for vertex in vertices.iter() {
        //     info!("vert: {:?}", vertex);
        // }
        // info!("");
        // info!("verts len: {}", vertices.len());

        // let mut other_verts = Self::generate_text_vertices(&self.font, "Hello there! :^) ygubuylka", glam::Vec2::new(-1f32, 0f32), 0.0005f32);

        // text_vertices.append(&mut other_verts);

        self.renderer.upload(device, &simple_vertices);
        self.text_renderer.upload(device, &text_vertices);
    }

    /// Tells the Zui Renderer to draw the UI
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.renderer.render(render_pass);
        self.text_renderer
            .render(render_pass, &self.font.texture_atlas);
    }

    /// Resizes the zui context
    pub fn resize(&mut self, width_px: u32, height_px: u32) {
        self.width_px = width_px;
        self.height_px = height_px;
        self.aspect_ratio = width_px as f32 / height_px as f32;
    }

    /// Gives the current Font
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// Gives the overall aspect ratio of the application window surface
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Gives the width of the application window surface in pixels
    pub fn width_px(&self) -> u32 {
        self.width_px
    }

    /// Gives the height of the application window surface in pixels
    pub fn height_px(&self) -> u32 {
        self.height_px
    }

    /// Updates the cursor state tracked by zui
    pub fn update_cursor_state(&mut self, cursor_physical_position: PhysicalPosition<f64>) {
        self.cursor_state = Some(ScreenSpacePosition::from_cursor_physical_position(
            cursor_physical_position,
            self.width_px,
            self.height_px,
        ));
    }

    /// Gives the height of the application window surface in pixels
    pub fn cursor_state(&self) -> Option<ScreenSpacePosition> {
        self.cursor_state
    }
}
