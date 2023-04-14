mod font;
mod primitives;
mod renderer;
mod scene;
pub mod util;
mod widget;

pub use font::Font;
pub use primitives::ScreenSpacePosition;
pub use renderer::Renderer;
pub use scene::{Scene, SceneHandle};
pub use widget::{Axis, Colour, Span, Widget};
use winit::dpi::PhysicalPosition;

use self::{primitives::Rectangle, renderer::Vertex, scene::Renderable};

pub struct Zui {
    font: Font,
    renderer: Renderer,

    width_px: u32,
    height_px: u32,
    aspect_ratio: f32,

    cursor_state: Option<ScreenSpacePosition>,
}

impl Zui {
    pub fn new(
        file: &str,
        size_px: u32,
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        width_px: u32,
        height_px: u32,
    ) -> Result<Self, ()> {
        let font_default = match Font::new(file, size_px) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        Ok(Self {
            font: font_default,
            renderer: Renderer::new(device, surface_configuration),
            width_px,
            height_px,
            aspect_ratio: width_px as f32 / height_px as f32,
            cursor_state: None,
        })
    }

    pub fn set_font(mut self, file: &str, size_px: u32) -> Result<(), ()> {
        self.font = match Font::new(file, size_px) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        Ok(())
    }

    pub fn update_widget_rectangles(&mut self) {}

    /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    pub fn renderer_upload(&mut self, device: &wgpu::Device, renderable: &dyn Renderable) {
        let vertices = renderable.to_vertices();

        // for vertex in vertices.iter() {
        //     info!("vert: {:?}", vertex);
        // }
        // info!("");
        // info!("verts len: {}", vertices.len());

        self.renderer.upload(device, &vertices);
    }

    /// Tells the Zui Renderer to draw the UI
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.renderer.render(render_pass);
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
