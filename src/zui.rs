mod colour;
pub mod font;
pub mod premade_widgets;
mod primitives;
mod render_layer;
mod renderable;
mod scene;
mod scene_handle;
mod scene_store;
mod simple_renderer;
pub mod text;
mod text_renderer;
mod texture_atlas;
pub mod util;
mod widget;

pub use colour::Colour;
pub use font::Typeface;
pub use primitives::{Rectangle, ScreenSpacePosition};
pub use renderable::Renderable;
pub use scene::Scene;
pub use scene_handle::SceneHandle;
pub use scene_store::SceneStore;
use simple_renderer::SimpleRenderer;
pub use text::{
    LineWrapping, Text, TextAlignmentHorizontal, TextConfiguration, TextSegment, TextSize,
};
use text_renderer::TextRenderer;
pub use widget::{Axis, Event, MouseEvent, Span, Widget};

use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::render_state::RenderState;

use self::font::FontStyle;

pub struct Zui {
    typeface: Typeface,

    renderer: SimpleRenderer,
    text_renderer: TextRenderer,

    viewport_dimensions_px: PhysicalSize<u32>,
    aspect_ratio: f32,
    cursor_position: Option<PhysicalPosition<f64>>,
}

impl Zui {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_configuration: &wgpu::SurfaceConfiguration,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> Result<Self, ()> {
        let mut typeface = match Typeface::new(
            Some("resources/zui/fonts/Roboto-Regular.ttf"),
            Some("resources/zui/fonts/Roboto-Bold.ttf"),
            Some("resources/zui/fonts/Roboto-Italic.ttf"),
        ) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        typeface.queue_rasterise(FontStyle::Regular, 32);
        typeface.queue_rasterise(FontStyle::Bold, 32);
        typeface.queue_rasterise(FontStyle::Italic, 32);

        typeface.queue_rasterise(FontStyle::Regular, 64);

        typeface.rasterise(device, queue);

        let text_renderer = TextRenderer::new(
            device,
            surface_configuration,
            typeface.texture_atlas.as_ref().unwrap().bind_group_layout(),
        );

        Ok(Self {
            typeface,
            renderer: SimpleRenderer::new(device, surface_configuration),
            text_renderer,
            viewport_dimensions_px,
            aspect_ratio: viewport_dimensions_px.width as f32
                / viewport_dimensions_px.height as f32,
            cursor_position: None,
        })
    }

    pub fn render_scene_handle<Message>(
        &mut self,
        scene_handle: &SceneHandle<Message>,
        render_state: &mut RenderState,
    ) where
        Message: Copy + Clone,
    {
        let render_layers = scene_handle.to_render_layers(self.viewport_dimensions_px);

        // clearing the screen, this is where the world render pass would go
        _ = render_state.render_clear();
        // render_state.submit_command_encoder();

        // for each layer in render_layers
        for render_layer in render_layers {
            self.renderer
                .upload(render_state.device(), &render_layer.simple_vertices);
            self.text_renderer
                .upload(render_state.device(), &render_layer.text_vertices);

            let clip_rectangle = match render_layer.clip_rectangle {
                Some(clip_rect) => {
                    let scissor_rect_framebuffer_coordinates = Rectangle::new(
                        clip_rect.x_min as u32,
                        clip_rect.x_max as u32,
                        self.viewport_dimensions_px.height - clip_rect.y_max as u32,
                        self.viewport_dimensions_px.height - clip_rect.y_min as u32,
                    );

                    // wgpu will panic if the scissor rectangle has a width or height of 0
                    if scissor_rect_framebuffer_coordinates.width() == 0
                        || scissor_rect_framebuffer_coordinates.height() == 0
                    {
                        None
                    } else {
                        Some(scissor_rect_framebuffer_coordinates)
                    }
                }
                None => None,
            };

            let render_pass_opt = render_state.render_with_clip_rectangle(clip_rectangle);
            match render_pass_opt {
                Some(mut rp) => {
                    self.renderer.render(&mut rp);
                    self.text_renderer
                        .render(&mut rp, &self.typeface.texture_atlas.as_ref().unwrap());
                }
                None => {}
            };
        }
        // render_state.submit_command_encoder();

        // for render_layer in render_layers {
        //     self.renderer
        //         .upload(render_state.device(), &render_layer.simple_vertices);
        //     self.text_renderer
        //         .upload(render_state.device(), &render_layer.text_vertices);
        // }
        // for (index, render_layer) in render_layers.iter().enumerate() {
        //     println!("layer: {}", index);
        // }

        render_state.submit_command_encoder();
        _ = render_state.present();
    }

    // /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    // pub fn upload_vertices(&mut self, device: &wgpu::Device, renderable: &dyn Renderable) {
    //     let (simple_vertices, text_vertices) = renderable.to_vertices(glam::Vec2::new(
    //         self.viewport_dimensions_px.width as f32,
    //         self.viewport_dimensions_px.height as f32,
    //     ));

    //     self.renderer.upload(device, &simple_vertices);
    //     self.text_renderer.upload(device, &text_vertices);
    // }

    // /// Tells the Zui Renderer to draw the UI
    // pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
    //     self.renderer.render(render_pass);
    //     self.text_renderer
    //         .render(render_pass, &self.font.texture_atlas);
    // }

    /// Resizes the zui context
    pub fn resize(&mut self, viewport_dimensions_px: PhysicalSize<u32>) {
        self.viewport_dimensions_px = viewport_dimensions_px;
        self.aspect_ratio =
            viewport_dimensions_px.width as f32 / viewport_dimensions_px.height as f32;
    }

    /// Updates the cursor state tracked by zui, will only ever be called when cursor is over viewport
    pub fn update_cursor_position(&mut self, cursor_physical_position: PhysicalPosition<f64>) {
        // NOTE: the cursor can be out of the window if clicked, held and dragged out of the window
        let position = PhysicalPosition::new(
            cursor_physical_position.x,
            self.viewport_dimensions_px.height as f64 - cursor_physical_position.y,
        );

        self.cursor_position = Some(position)
    }

    /// Called when the cursor leaves the screen
    pub fn cursor_left(&mut self) {
        self.cursor_position = None;
    }

    pub fn cursor_position(&self) -> Option<PhysicalPosition<f64>> {
        self.cursor_position
    }

    /// Gets the context for an event
    pub fn context<'a>(&self) -> Context {
        Context {
            font: &self.typeface,
            aspect_ratio: self.aspect_ratio,
            cursor_position: self.cursor_position,
            viewport_dimensions_px: self.viewport_dimensions_px,
        }
    }
}

/// The 'context' for zui. Intended to give a Widget everything it needs to know to rebuild itself
/// correctly.
pub struct Context<'a> {
    pub font: &'a Typeface,
    pub aspect_ratio: f32,
    pub cursor_position: Option<PhysicalPosition<f64>>,
    pub viewport_dimensions_px: PhysicalSize<u32>,
}

impl<'a> std::fmt::Debug for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}
