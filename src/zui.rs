mod colour;
mod position_constraint;
pub mod premade_widgets;
mod primitives;
mod render_layer;
mod renderable;
mod scene;
mod scene_handle;
mod simple_renderer;
mod span_constraint;
mod stopwatch;
pub mod text;
mod text_renderer;
mod texture_atlas;
pub mod typeface;
pub mod util;
mod widget;
mod widget_store;

pub use colour::named as named_colours;
pub use colour::Colour;
pub use position_constraint::{PaddingWeights, PositionConstraint};
pub use primitives::Rectangle;
pub use renderable::Renderable;
pub use scene::Scene;
pub use scene_handle::SceneHandle;
use simple_renderer::SimpleRenderer;
pub use span_constraint::{ParentHeight, ParentWidth, SpanConstraint, ViewHeight, ViewWidth};
pub use text::{
    LineWrapping, Text, TextAlignmentHorizontal, TextAlignmentVertical, TextConfiguration, TextSegment, TextSize,
};
use text_renderer::TextRenderer;
pub use typeface::Typeface;
pub use widget::{Axis, Event, MouseEvent, Widget};
pub use widget_store::{EntryOverrideDescriptor, EntryChildren, WidgetId, WidgetStore};

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, WindowEvent},
};

use self::primitives::Dimensions;

pub struct Zui {
    typeface: Typeface,

    renderer: SimpleRenderer,
    text_renderer: TextRenderer,

    viewport_dimensions_px: Dimensions<u32>,
    aspect_ratio: f32,
    cursor_position: Option<PhysicalPosition<f64>>,
}

impl Zui {
    pub fn new(
        device: &wgpu::Device,
        surface_configuration: &wgpu::SurfaceConfiguration,
        viewport_dimensions_px: Dimensions<u32>,
    ) -> Result<Self, ()> {
        let typeface = match Typeface::new(
            Some("resources/zui/fonts/ArimoNerdFont-Regular.ttf"),
            Some("resources/zui/fonts/Roboto-Bold.ttf"),
            Some("resources/zui/fonts/Roboto-Italic.ttf"),
            device,
        ) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let text_renderer = TextRenderer::new(
            device,
            surface_configuration,
            typeface.texture_atlas.bind_group_layout(),
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

    /// Converts a clip rectangle from f32 to u32
    fn rectangle_f32_to_u32(rect: &Rectangle<f32>) -> Rectangle<u32> {
        Rectangle::new(
            rect.x_min.round() as u32,
            rect.x_max.round() as u32,
            rect.y_min.round() as u32,
            rect.y_max.round() as u32,
        )
    }

    /// Returns a rectangle that gives the intersection of the y-up rectangle with the viewport
    fn rectangle_viewport_intersection(
        rect: &Rectangle<u32>,
        viewport_dimensions_px: Dimensions<u32>,
    ) -> Option<Rectangle<u32>> {
        let viewport_rect_px = Rectangle::new(
            0u32,
            viewport_dimensions_px.width,
            0u32,
            viewport_dimensions_px.height,
        );

        rect.intersection(&viewport_rect_px)
    }

    /// Converts a rectangle from y-up pixel coordinates to y-down framebuffer coordinates
    fn rectangle_pixel_to_framebuffer(
        rect: &Rectangle<u32>,
        viewport_dimensions_px: Dimensions<u32>,
    ) -> Rectangle<u32> {
        Rectangle::new(
            rect.x_min,
            rect.x_max,
            viewport_dimensions_px.height - rect.y_max as u32,
            viewport_dimensions_px.height - rect.y_min as u32,
        )
    }

    /// Returns true if the rectangle width or height is zero
    fn rectangle_has_no_area(rect: &Rectangle<u32>) -> bool {
        rect.width() == 0 || rect.height() == 0
    }

    /// Tries to draw with a render pass, if it exists
    fn try_render_pass_draw(&self, render_pass_opt: Option<wgpu::RenderPass>) {
        match render_pass_opt {
            Some(mut rp) => {
                self.renderer.render(&mut rp);
                self.text_renderer
                    .render(&mut rp, &self.typeface.texture_atlas);
            }
            None => {}
        };
    }

    /// Renders a scene handle
    pub fn render_scene_handle<Message>(
        &mut self,
        scene_handle: &SceneHandle<Message>,
        device: &wgpu::Device,
        output_texture_view: &wgpu::TextureView,
        command_encoder: &mut wgpu::CommandEncoder,
    ) where
        Message: Copy + Clone,
    {
        /// Describes how the RenderLayer should be rendered
        #[derive(Debug)]
        enum RenderLayerBehaviour {
            WithFramebufferClipRectangle(Rectangle<u32>),
            WithoutClipRectangle,
            DoNotRender,
        }

        let render_layers = scene_handle.to_render_layers(&self.context());

        // rendering each of the render layers
        for render_layer in render_layers {
            // determining whether the layer should be rendered or not
            let render_layer_behaviour = match render_layer.clip_rectangle {
                Some(clip_rect) => {
                    // converting to whole-pixel u32 coordinates
                    let clip_rect = Self::rectangle_f32_to_u32(&clip_rect);

                    // getting intersection with the viewport
                    match Self::rectangle_viewport_intersection(
                        &clip_rect,
                        self.viewport_dimensions_px,
                    ) {
                        // if in viewport render with clip rectangle
                        Some(clip_rect) => match Self::rectangle_has_no_area(&clip_rect) {
                            false => RenderLayerBehaviour::WithFramebufferClipRectangle(
                                Self::rectangle_pixel_to_framebuffer(
                                    &clip_rect,
                                    self.viewport_dimensions_px,
                                ),
                            ),
                            true => RenderLayerBehaviour::DoNotRender,
                        },

                        // if not intersecting with viewport, render nothing
                        None => RenderLayerBehaviour::DoNotRender,
                    }
                }

                // if no clip rectangle, render the whole screen, ie - without clip rectangle
                None => RenderLayerBehaviour::WithoutClipRectangle,
            };

            match render_layer_behaviour {
                RenderLayerBehaviour::WithFramebufferClipRectangle(fcr) => {
                    self.renderer.upload(device, &render_layer.simple_vertices);
                    self.text_renderer
                        .upload(device, &render_layer.text_vertices);

                    let render_pass_opt = Self::try_render_pass_with_clip_rectangle(
                        command_encoder,
                        output_texture_view,
                        Some(fcr),
                    );
                    self.try_render_pass_draw(render_pass_opt);
                }
                RenderLayerBehaviour::WithoutClipRectangle => {
                    self.renderer.upload(device, &render_layer.simple_vertices);
                    self.text_renderer
                        .upload(device, &render_layer.text_vertices);

                    let render_pass_opt = Self::try_render_pass_with_clip_rectangle(
                        command_encoder,
                        output_texture_view,
                        None,
                    );
                    self.try_render_pass_draw(render_pass_opt);
                }
                RenderLayerBehaviour::DoNotRender => {
                    // Do nothing!
                }
            };
        }

        // Note: command encoder should be submitted by the end user!

        // submitting the command encoder
        // render_state.submit_command_encoder();
    }

    /// Gives a RenderPass, with a clip rectangle set if provided
    fn try_render_pass_with_clip_rectangle<'a>(
        command_encoder: &'a mut wgpu::CommandEncoder,
        viewport_texture_view: &'a wgpu::TextureView,
        clip_rectangle: Option<Rectangle<u32>>,
    ) -> Option<wgpu::RenderPass<'a>> {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("zui_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: viewport_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        if let Some(clip_rectangle) = clip_rectangle {
            render_pass.set_scissor_rect(
                clip_rectangle.x_min,
                clip_rectangle.y_min,
                clip_rectangle.width(),
                clip_rectangle.height(),
            );
        }

        // Do rendering
        return Some(render_pass);
    }

    /// Resizes the zui context
    fn handle_winit_resized(&mut self, viewport_dimensions_px: Dimensions<u32>) {
        self.viewport_dimensions_px = viewport_dimensions_px;
        self.aspect_ratio =
            viewport_dimensions_px.width as f32 / viewport_dimensions_px.height as f32;
    }

    /// Updates the cursor state tracked by zui, will only ever be called when cursor is over viewport
    fn handle_winit_cursor_moved(&mut self, cursor_physical_position: PhysicalPosition<f64>) {
        // NOTE: the cursor can be out of the window if clicked, held and dragged out of the window
        let position = PhysicalPosition::new(
            cursor_physical_position.x,
            self.viewport_dimensions_px.height as f64 - cursor_physical_position.y,
        );

        self.cursor_position = Some(position)
    }

    /// Called when the cursor leaves the screen
    fn handle_winit_cursor_left(&mut self) {
        self.cursor_position = None;
    }

    /// Gets the context for a Widget event
    pub fn context<'a>(&self) -> Context {
        Context {
            typeface: &self.typeface,
            aspect_ratio: self.aspect_ratio,
            cursor_position: self.cursor_position,
            viewport_dimensions_px: self.viewport_dimensions_px,
        }
    }

    /// Gets the context for rebuilding widgets with a mutable typeface
    pub fn context_mut_typeface<'a>(&mut self) -> ContextMutTypeface {
        ContextMutTypeface {
            typeface: &mut self.typeface,
            aspect_ratio: self.aspect_ratio,
            cursor_position: self.cursor_position,
            viewport_dimensions_px: self.viewport_dimensions_px,
        }
    }

    /// tries to save the typeface texture atlas image to the provided path
    pub fn debug_try_save_typeface_texture_atlas(&self, path: &str) {
        _ = self.typeface.texture_atlas.save_texture_to_disk(path);
    }

    /// Handles a winit event, can pass the relevant zui::Event onto a SceneHandle if suitable
    pub fn handle_winit_window_event<Message>(
        &mut self,
        event: &winit::event::WindowEvent<'_>,
        scene_handle: Option<&mut SceneHandle<Message>>,
    ) where
        Message: Copy,
    {
        match event {
            WindowEvent::CursorMoved {
                position: cursor_physical_position,
                ..
            } => {
                self.handle_winit_cursor_moved(*cursor_physical_position);
                scene_handle.map(|sh| {
                    sh.handle_event(Event::MouseEvent(MouseEvent::CursorMoved), &self.context())
                });
            }
            WindowEvent::CursorLeft { .. } => {
                self.handle_winit_cursor_left();
                scene_handle.map(|sh| {
                    sh.handle_event(
                        Event::MouseEvent(MouseEvent::CursorExitedWindow),
                        &self.context(),
                    )
                });
            }
            WindowEvent::Resized(physical_size) => {
                self.handle_winit_resized(physical_size.into());
                // TODOWS
                // scene_handle.map(|sh| sh.queue_rebuild_scene());
            }
            WindowEvent::MouseInput { state, .. } => {
                let event = match state {
                    ElementState::Pressed => Event::MouseEvent(MouseEvent::ButtonPressed),
                    ElementState::Released => Event::MouseEvent(MouseEvent::ButtonReleased),
                };
                scene_handle.map(|sh| sh.handle_event(event, &self.context()));
            }
            _ => {}
        }
    }
}

/// The 'context' for zui. Intended to give a Widget everything it needs to know to rebuild itself
/// correctly.
pub struct Context<'a> {
    pub typeface: &'a Typeface,
    pub aspect_ratio: f32,
    pub cursor_position: Option<PhysicalPosition<f64>>,
    pub viewport_dimensions_px: Dimensions<u32>,
}

/// Same as the previously declared context, but with a mutable TypeFace
pub struct ContextMutTypeface<'a> {
    pub typeface: &'a mut Typeface,
    pub aspect_ratio: f32,
    pub cursor_position: Option<PhysicalPosition<f64>>,
    pub viewport_dimensions_px: Dimensions<u32>,
}

impl<'a> std::fmt::Debug for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}
