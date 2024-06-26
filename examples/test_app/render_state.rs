use wgpu::{CommandEncoder, RenderPass, StoreOp, SurfaceTargetUnsafe, SurfaceTexture, TextureView};

pub struct RenderState<'a> {
    _adapter: wgpu::Adapter,
    pub surface: wgpu::Surface<'a>,
    _surface_format: wgpu::TextureFormat,
    surface_configuration: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    window_size: winit::dpi::PhysicalSize<u32>,
    skip_rendering: bool,

    pub surface_texture_view: Option<TextureView>,
    pub command_encoder: Option<CommandEncoder>,
    _surface_texture: Option<SurfaceTexture>,
}

#[allow(dead_code)]
impl<'a> RenderState<'a> {
    pub async fn new(window: &winit::window::Window) -> Self {
        let window_size = window.inner_size();

        // instance's purpose is to create adapters and surfaces
        // let default_backend = wgpu::Backends::VULKAN;
        // let instance = wgpu::Instance::new(default_backend);
        // let instance = wgpu::Instance::default();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = match unsafe {
            instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(window).unwrap())
        } {
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
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
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
            desired_maximum_frame_latency: 2,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *format,
            width: window_size.width,
            height: window_size.height,
            alpha_mode: wgpu::CompositeAlphaMode::Auto, // TODO
            present_mode: wgpu::PresentMode::AutoNoVsync,
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
            _surface_texture: None,
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
        self._surface_texture = match self.surface.get_current_texture() {
            Ok(t) => Some(t),
            Err(_) => {
                error!("failed to get surface texture!");
                return None;
            }
        };

        self.surface_texture_view = Some(
            self._surface_texture
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
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,    // TODO: check this
                    occlusion_query_set: None, // TODO: check this
                },
            );

            // Do rendering
            Some(render_pass)
        }

        // self.queue.submit(std::iter::once(
        //     self.command_encoder.take().unwrap().finish(),
        // ));
        // self.surface_texture.take().unwrap().present();
    }

    /// Submits the previously created RenderPass/Command encoder, must be done before presenting
    pub fn submit_command_encoder(&mut self) {
        let command_encoder = match self.command_encoder.take() {
            Some(ce) => ce,
            None => {
                error!("failed to submit non-existant command encoder!");
                return;
            }
        };

        // self.surface_texture.take();

        self.queue.submit(Some(command_encoder.finish()));
    }

    /// Presents the surfacecommand_encoder, must be done after submitting the command encoder
    pub fn present(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = match self._surface_texture.take() {
            Some(st) => st,
            None => {
                warn!("failed to get surface texture");
                return Ok(());
            }
        };

        Ok(surface_texture.present())
    }

    fn update_skip_rendering(&mut self) {
        self.skip_rendering = !(self.window_size.width > 0 || self.window_size.height > 0);
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn _queue(&self) -> &wgpu::Queue {
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
