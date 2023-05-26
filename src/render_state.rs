use image::GenericImageView;
use wgpu::{CommandEncoder, RenderPass, SurfaceTexture, TextureView};

use crate::zui::{Zui, Rectangle};

pub struct RenderState {
    _adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    _surface_format: wgpu::TextureFormat,
    surface_configuration: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window_size: winit::dpi::PhysicalSize<u32>,
    skip_rendering: bool,

    surface_texture_view: Option<TextureView>,
    command_encoder: Option<CommandEncoder>,
    surface_texture: Option<SurfaceTexture>,
}

impl RenderState {
    pub async fn new(window: &winit::window::Window) -> Self {
        let window_size = window.inner_size();

        // instance's purpose is to create adapters and surfaces
        // let default_backend = wgpu::Backends::VULKAN;
        // let instance = wgpu::Instance::new(default_backend);
        let instance = wgpu::Instance::default();

        let surface = match unsafe { instance.create_surface(&window) } {
            Ok(s) => s,
            Err(_) => {
                error!("failed to create surface!");
                panic!();
            }
        };

        // handle to the graphics card
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // power preference is helpful for extending battery life when low etc.
                power_preference: wgpu::PowerPreference::default(),

                // tells instance to find an adapter that can present to the compatible_surface
                compatible_surface: Some(&surface),

                // will typically force wgpu to use software renderer if true
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // getting the device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        // checking window width and height for surface_configuration validity
        if window_size.width == 0 || window_size.height == 0 {
            error!("cannot start with window size width or height = 0!");
            panic!();
        }

        // configuring the surface
        let swapchain_capabilites = surface.get_capabilities(&adapter);
        let format = match swapchain_capabilites.formats.first() {
            Some(f) => f,
            None => {
                error!("could not find any formats for the surface!");
                panic!();
            }
        };

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *format,
            width: window_size.width,
            height: window_size.height,
            alpha_mode: wgpu::CompositeAlphaMode::Auto, // TODO
            present_mode: wgpu::PresentMode::AutoVsync,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_configuration);

        let mut this = Self {
            _adapter: adapter,
            surface,
            _surface_format: *format,
            surface_configuration,
            device,
            queue,
            window_size,
            skip_rendering: false,
            surface_texture_view: None,
            command_encoder: None,
            surface_texture: None,
        };

        this.update_skip_rendering();

        this
    }

    pub fn resize(&mut self, window_size_new: &winit::dpi::PhysicalSize<u32>) {
        if window_size_new.width == 0 || window_size_new.height == 0 {
            self.skip_rendering = true;
            return;
        }

        self.window_size = *window_size_new;
        self.surface_configuration.width = window_size_new.width;
        self.surface_configuration.height = window_size_new.height;
        self.surface
            .configure(&self.device, &self.surface_configuration);

        self.update_skip_rendering();
    }

    /// Gives the RenderPass that does the initial clear of the screen
    pub fn render_clear(&mut self) -> Option<RenderPass> {
        if self.skip_rendering {
            return None;
        }

        self.surface_texture = match self.surface.get_current_texture() {
            Ok(t) => Some(t),
            Err(_) => {
                error!("failed to get surface texture!");
                return None;
            }
        };

        self.surface_texture_view = Some(
            self.surface_texture
                .as_mut()
                .unwrap()
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        self.command_encoder = Some(self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("render pass command encoder"),
            },
        ));

        {
            let render_pass = self.command_encoder.as_mut().unwrap().begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("world_render_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.surface_texture_view.as_ref().unwrap(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear({
                                let r = 0.1f64;
                                let g = 0.2f64;
                                let b = 0.3f64;
                                let a = 1f64;

                                wgpu::Color { r, g, b, a }
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                },
            );

            // Do rendering
            return Some(render_pass);
        }

        // self.queue.submit(std::iter::once(
        //     self.command_encoder.take().unwrap().finish(),
        // ));
        // self.surface_texture.take().unwrap().present();
    }

    /// Gives the RenderPass that does the initial clear of the screen
    pub fn render_with_clip_rectangle(&mut self, clip_rectangle: Option<Rectangle<f32>>) -> Option<RenderPass> {
        if self.skip_rendering {
            return None;
        }

        // self.surface_texture = match self.surface.get_current_texture() {
        //     Ok(t) => Some(t),
        //     Err(_) => {
        //         error!("failed to get surface texture!");
        //         return None;
        //     }
        // };

        // self.surface_texture_view = Some(
        //     self.surface_texture
        //         .as_mut()
        //         .unwrap()
        //         .texture
        //         .create_view(&wgpu::TextureViewDescriptor::default()),
        // );

        // self.command_encoder = Some(self.device.create_command_encoder(
        //     &wgpu::CommandEncoderDescriptor {
        //         label: Some("render pass command encoder"),
        //     },
        // ));

        {
            let mut render_pass = self.command_encoder.as_mut().unwrap().begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("world_render_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.surface_texture_view.as_ref().unwrap(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                },
            );

            if let Some(clip_rectangle) = clip_rectangle {
                // render_pass.set_scissor_rect(
                //     clip_rectangle.x_min,
                //     clip_rectangle.y_min,
                //     clip_rectangle.width(),
                //     clip_rectangle.height(),
                // );
            }

            // Do rendering
            return Some(render_pass);
        }
    }

    pub fn submit_command_encoder(&mut self) {
        let command_encoder = match self.command_encoder.take() {
            Some(ce) => ce,
            None => {
                error!("failed to submit non-existant command encoder!");
                return;
            }
        };

        // self.surface_texture.take();

        self.queue.submit(std::iter::once(command_encoder.finish()));
    }

    /// Presents the surfacecommand_encoder
    pub fn present(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = match self.surface_texture.take() {
            Some(st) => st,
            None => {
                warn!("failed to get surface texture");
                return Ok(());
            }
        };
        Ok(surface_texture.present())
    }

    // pub fn render(&mut self, zui: &Zui) -> Result<(), wgpu::SurfaceError> {
    //     if self.skip_rendering {
    //         return Ok(());
    //     }queue

    //     let surface_texture = self.surface.get_current_texture()?;
    //     let surface_texture_view = surface_texture
    //         .texture
    //         .surfacereate_view(&wgpu::TextureViewDescriptor::default());

    //     let mut command_encoder =
    //         self.device
    //             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //                 label: Some("render pass command encoder"),
    //             });

    //     {
    //         let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: Some("world_render_pass"),
    //             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //                 view: &surface_texture_view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear({
    //                         // TODO
    //                         let r = 0.1f64;
    //                         let g = 0.2f64;
    //                         let b = 0.3f64;
    //                         let a = 1f64;

    //                         wgpu::Color { r, g, b, a }
    //                     }),
    //                     store: true,
    //                 },
    //             })],
    //             depth_stencil_attachment: None,
    //         });

    //         // TODO: do rendering
    //         zui.render(&mut render_pass);
    //     }

    //     self.queue.submit(std::iter::once(command_encoder.finish()));

    //     surface_texture.present();

    //     Ok(())
    // }

    fn update_skip_rendering(&mut self) {
        self.skip_rendering = !(self.window_size.width > 0 || self.window_size.height > 0);
    }

    pub fn window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface_configuration(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_configuration
    }

    // pub fn deviceurface_format(&self) -> wgpu::TextureFormat {
    //     self._surface_format
    // }

    pub fn skip_rendering(&self) -> bool {
        self.skip_rendering
    }
}
