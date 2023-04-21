mod font;
pub mod premade_widgets;
mod primitives;
mod renderer;
mod scene;
mod text_renderer;
mod texture_atlas;
pub mod util;
mod widget;
// mod tree;

pub use font::Font;
pub use primitives::ScreenSpacePosition;
use renderer::Renderer;
pub use scene::{Scene, SceneHandle};
use text_renderer::TextRenderer;
use wgpu::Device;
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

    // pub fn set_font(mut self, file: &str, size_px: u32) -> Result<(), ()> {
    //     self._font = match Font::new(file, size_px, queue, ) {
    //         Ok(f) => f,
    //         Err(_) => return Err(()),
    //     };

    //     Ok(())
    // }

    pub fn update_widget_rectangles(&mut self) {}

    /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    pub fn renderer_upload(&mut self, device: &wgpu::Device, renderable: &dyn Renderable) {
        let (simple_vertices, mut text_vertices) = renderable.to_vertices();

        // for vertex in vertices.iter() {
        //     info!("vert: {:?}", vertex);
        // }
        // info!("");
        // info!("verts len: {}", vertices.len());

        // test text vertices
        let (_symbol_info, top_left, bottom_right) = self.font.get_symbol('a').unwrap();

        let hspan = 0.5f32;
        let text_verts = [
            TextVertex::new(
                glam::Vec2::new(-hspan, hspan),
                glam::vec2(top_left.x(), top_left.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(hspan, hspan),
                glam::vec2(bottom_right.x(), top_left.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(-hspan, -hspan),
                glam::vec2(top_left.x(), bottom_right.y()),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(hspan, -hspan),
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

        self.update_widget_rectangles();
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
