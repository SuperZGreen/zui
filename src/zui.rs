mod font;
pub mod premade_widgets;
mod primitives;
mod renderable;
mod renderer;
mod scene;
mod scene_handle;
mod text;
mod text_renderer;
mod texture_atlas;
pub mod util;
mod widget;
mod scene_store;
mod colour;
// mod tree;

pub use font::Font;
pub use primitives::ScreenSpacePosition;
pub use renderable::Renderable;
use renderer::Renderer;
pub use scene::Scene;
pub use scene_handle::SceneHandle;
pub use text::{Text, TextSegment, TextConfiguration, LineWrapping, TextSize};
use text_renderer::TextRenderer;
pub use widget::{Axis, Span, Widget};
pub use scene_store::SceneStore;
pub use colour::Colour;

use winit::{dpi::PhysicalPosition, event::{MouseButton, ElementState}};

use self::primitives::Rectangle;

pub struct CursorState {
    is_clicked: bool,
    screen_space_position: ScreenSpacePosition,
}

impl CursorState {
    pub fn new(is_clicked: bool, screen_space_position: ScreenSpacePosition) -> Self {
        Self {
            is_clicked,
            screen_space_position,
        }
    }

    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }

    pub fn screen_space_position(&self) -> ScreenSpacePosition {
        self.screen_space_position
    }
}

pub struct Zui {
    font: Font,

    renderer: Renderer,
    text_renderer: TextRenderer,

    width_px: u32,
    height_px: u32,
    aspect_ratio: f32,
    cursor_state: Option<CursorState>,
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

    /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    pub fn upload_vertices(&mut self, device: &wgpu::Device, renderable: &dyn Renderable) {
        let (simple_vertices, text_vertices) =
            renderable.to_vertices(glam::Vec2::new(self.width_px as f32, self.height_px as f32));

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

    /// Updates the cursor state tracked by zui, will only ever be called when cursor is over viewport
    pub fn update_cursor_position(&mut self, cursor_physical_position: PhysicalPosition<f64>) {
        let screen_space_position = ScreenSpacePosition::from_cursor_physical_position(
            cursor_physical_position,
            self.width_px,
            self.height_px,
        );

        self.cursor_state = if screen_space_position.is_in_viewport_bounds() {
            Some(CursorState {
                is_clicked: false,
                screen_space_position,
            })
        } else {
            None
        }
    }
    
    /// Called when the cursor leaves the screen
    pub fn cursor_left(&mut self) {
        self.cursor_state = None;
    }
    
    /// Handles mouse clicks via winit's types
    pub fn mouse_input(&mut self, _button: MouseButton, state: ElementState) {
        if let Some(cursor_state) = &mut self.cursor_state {
            cursor_state.is_clicked = match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            }
        }
    }
    
    pub fn update(&mut self) {
        if let Some(cursor_state) = &mut self.cursor_state {
            cursor_state.is_clicked = false
        }
    }

    pub fn cursor_state(&self) -> &Option<CursorState> {
        &self.cursor_state
    }
}
