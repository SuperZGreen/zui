mod colour;
mod font;
pub mod premade_widgets;
mod primitives;
mod renderable;
mod renderer;
mod scene;
mod scene_handle;
mod scene_store;
mod text;
mod text_renderer;
mod texture_atlas;
pub mod util;
mod widget;
// mod tree;

pub use colour::Colour;
pub use font::Font;
pub use primitives::ScreenSpacePosition;
pub use renderable::Renderable;
use renderer::Renderer;
pub use scene::Scene;
pub use scene_handle::SceneHandle;
pub use scene_store::SceneStore;
pub use text::{LineWrapping, Text, TextConfiguration, TextSegment, TextSize};
use text_renderer::TextRenderer;
pub use widget::{Axis, BaseWidget, Event, MouseEvent, Span, Widget};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use self::primitives::Rectangle;

pub struct Zui {
    font: Font,

    renderer: Renderer,
    text_renderer: TextRenderer,

    viewport_dimensions_px: PhysicalSize<u32>,
    aspect_ratio: f32,
    cursor_position: Option<ScreenSpacePosition>,
}

impl Zui {
    pub fn new(
        file: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_configuration: &wgpu::SurfaceConfiguration,
        viewport_dimensions_px: PhysicalSize<u32>,
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
            viewport_dimensions_px,
            aspect_ratio: viewport_dimensions_px.width as f32
                / viewport_dimensions_px.height as f32,
            cursor_position: None,
        })
    }

    /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    pub fn upload_vertices(&mut self, device: &wgpu::Device, renderable: &dyn Renderable) {
        let (simple_vertices, text_vertices) = renderable.to_vertices(glam::Vec2::new(
            self.viewport_dimensions_px.width as f32,
            self.viewport_dimensions_px.height as f32,
        ));

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
    pub fn resize(&mut self, viewport_dimensions_px: PhysicalSize<u32>) {
        self.viewport_dimensions_px = viewport_dimensions_px;
        self.aspect_ratio =
            viewport_dimensions_px.width as f32 / viewport_dimensions_px.height as f32;
    }

    /// Gives the current Font
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// Gives the overall aspect ratio of the application window surface
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Updates the cursor state tracked by zui, will only ever be called when cursor is over viewport
    pub fn update_cursor_position(&mut self, cursor_physical_position: PhysicalPosition<f64>) {
        let screen_space_position = ScreenSpacePosition::from_cursor_physical_position(
            cursor_physical_position,
            self.viewport_dimensions_px,
        );

        // self.cursor_position = if screen_space_position.is_in_viewport_bounds() {
        //     Some(screen_space_position)
        // } else {
        //     None
        // }

        // NOTE: the cursor can be out of the window if clicked, held and dragged out of the window
        self.cursor_position = Some(screen_space_position)
    }

    /// Called when the cursor leaves the screen
    pub fn cursor_left(&mut self) {
        self.cursor_position = None;
    }

    // /// Handles mouse clicks via winit's types
    // pub fn mouse_input(&mut self, _button: MouseButton, state: ElementState) {
    //     if let Some(cursor_state) = &mut self.cursor_state {
    //         cursor_state.is_clicked = match state {
    //             ElementState::Pressed => true,
    //             ElementState::Released => false,
    //         }
    //     }
    // }

    // pub fn update(&mut self) {
    //     if let Some(cursor_state) = &mut self.cursor_state {
    //         cursor_state.is_clicked = false
    //     }
    // }

    pub fn cursor_position(&self) -> Option<ScreenSpacePosition> {
        self.cursor_position
    }

    /// Gets the context for an event
    pub fn context<'a>(&self) -> Context {
        Context {
            font: &self.font,
            aspect_ratio: self.aspect_ratio,
            cursor_position: self.cursor_position,
            viewport_dimensions_px: self.viewport_dimensions_px,
        }
    }
}

/// The context for an event
pub struct Context<'a> {
    pub font: &'a Font,
    pub aspect_ratio: f32,
    pub cursor_position: Option<ScreenSpacePosition>,
    pub viewport_dimensions_px: PhysicalSize<u32>,
}

impl<'a> std::fmt::Debug for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}
