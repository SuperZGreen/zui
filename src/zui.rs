mod font;
mod primitives;
mod renderer;
pub mod util;
mod widget;

pub use font::Font;
pub use renderer::Renderer;
pub use widget::{Axis, Colour, Span, Widget};

use self::{primitives::Rectangle, renderer::Vertex};

pub struct Zui {
    font: Font,
    root_widget: Option<Widget>,
    renderer: Renderer,
    
    width_px: u32,
    height_px: u32,
    aspect_ratio: f32,
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
            root_widget: None,
            renderer: Renderer::new(device, surface_configuration),
            width_px,
            height_px,
            aspect_ratio: width_px as f32 / height_px as f32,
        })
    }

    pub fn set_font(mut self, file: &str, size_px: u32) -> Result<(), ()> {
        self.font = match Font::new(file, size_px) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        Ok(())
    }

    pub fn set_root_widget(&mut self, root_widget: Option<Widget>) {
        self.root_widget = root_widget;
    }

    pub fn update_widget_rectangles(&mut self) {
        let root_widget = match &mut self.root_widget {
            Some(rw) => rw,
            None => return,
        };

        // updating rectangles
        root_widget.set_rectangle(Some(Rectangle::new(
            glam::Vec2::new(-1f32, 1f32),
            glam::Vec2::new(2f32, 2f32),
        )));

        root_widget.update_child_rectangles_recursively(self.aspect_ratio);

        root_widget.traverse(&mut |widget| {
            match widget.rectangle {
                Some(r) => println!("rect: {:?}", r),
                None => println!("rect: None"),
            };
        });
    }

    /// Turns the Widget's rectangles into vertices, uploads them to the GPU
    pub fn renderer_upload(&mut self, device: &wgpu::Device) {
        let mut vertices = Vec::new();
        match &self.root_widget {
            Some(rw) => rw.traverse(&mut |widget| {
                let colour = match widget.background {
                    Some(c) => c,
                    None => return,
                };

                let rectangle = match widget.rectangle {
                    Some(r) => r,
                    None => return,
                };

                let a = Vertex::new(rectangle.top_left, colour.into());
                let b = Vertex::new(
                    rectangle.top_left + glam::Vec2::new(rectangle.dimensions[0], 0f32),
                    colour.into(),
                );
                let c = Vertex::new(
                    rectangle.top_left + glam::Vec2::new(0f32, -rectangle.dimensions[1]),
                    colour.into(),
                );
                let d = Vertex::new(
                    rectangle.top_left
                        + glam::Vec2::new(rectangle.dimensions[0], -rectangle.dimensions[1]),
                    colour.into(),
                );

                vertices.push(a);
                vertices.push(c);
                vertices.push(b);

                vertices.push(b);
                vertices.push(c);
                vertices.push(d);
            }),
            None => {}
        }

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
}
