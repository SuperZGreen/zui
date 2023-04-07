use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[macro_use]
extern crate log;

use env_logger::Env;

mod render_state;
use render_state::RenderState;

mod zui;
use zui::Zui;

use crate::zui::{Axis, Colour, Span, Widget};

fn main() {
    info!("starting!");

    // configuring log
    std::env::set_var("RUST_BACKTRACE", "1");
    let env = Env::default().filter_or("MY_LOG_LEVEL", "zui=trace");
    env_logger::init_from_env(env);

    // winit init
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();

    // render init
    let mut render_state = pollster::block_on(RenderState::new(&window));

    // zui
    let resolution = window.inner_size();
    let mut zui = Zui::new(
        "resources/roboto.ttf",
        14,
        render_state.device(),
        render_state.surface_configuration(),
        resolution.width,
        resolution.height,
    )
    .unwrap();

    // let main_contents = Widget::new()
    //     // .with_span(Span::ParentRatio(0.9f32))
    //     .with_background(Some(Colour::rgb(0.5f32, 0.5f32, 0.5f32)));

    // let central_container_padding_top = Widget::new().with_span(Span::ViewMin(0.05f32));
    // let central_container_padding_bottom = Widget::new().with_span(Span::ViewMin(0.05f32));
    // let central_container_padding_left = Widget::new().with_span(Span::ViewMin(0.05f32));
    // let central_container_padding_right = Widget::new().with_span(Span::ViewMin(0.05f32));

    // let central_container_vertical = Widget::new()
    //     .with_axis(Axis::Vertical)
    //     // .with_span(Span::ParentRatio(0.9f32))
    //     .with_background(Some(Colour::rgb(0.4f32, 0.4f32, 0.4f32)))
    //     .push(central_container_padding_top)
    //     .push(main_contents)
    //     .push(central_container_padding_bottom);

    // let central_container = Widget::new()
    //     .with_axis(Axis::Horizontal)
    //     .with_span(Span::ParentRatio(0.75f32))
    //     .with_background(Some(Colour::rgb(0.3f32, 0.3f32, 0.3f32)))
    //     .push(central_container_padding_left)
    //     .push(central_container_vertical)
    //     .push(central_container_padding_right);

    // let outer_padding_left = Widget::new();
    // let outer_padding_right = Widget::new();

    // zui.set_root_widget(Some(
    //     Widget::new()
    //         .with_axis(Axis::Horizontal)
    //         // .with_background(Some(Colour::rgb(0.2f32, 0.2f32, 0.2f32)))
    //         .push(outer_padding_left)
    //         .push(central_container)
    //         .push(outer_padding_right),
    // ));

    zui.set_root_widget(Some(
        Widget::new()
            .with_axis(Axis::Horizontal)
            // .with_background(Some(Colour::rgb(0.2f32, 0.2f32, 0.2f32)))
            .push(Widget::new())
            .push(
                Widget::new()
                    .with_span(Span::ViewMin(1f32))
                    .with_background(Some(Colour::rgb(0.1f32, 0.1f32, 0.1f32))),
            )
            .push(Widget::new()),
    ));

    zui.update_widget_rectangles();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        info!("exiting!");
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize(physical_size);
                        zui.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(*new_inner_size);
                    }
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::CursorMoved { .. } => {
                        // TODO
                    }
                    WindowEvent::ModifiersChanged(_) => {
                        // TODO
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {}
                    _ => {}
                }

                // TODO: Event passthrough
            }
            Event::NewEvents(_) => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {
                // TODO: Solving

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if !render_state.skip_rendering() {
                    // uploading
                    zui.renderer_upload(render_state.device());

                    // rendering
                    match render_state.render(&zui) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            warn!("wgpu::SurfaceError::Lost");

                            let size = render_state.window_size();
                            render_state.resize(&size);
                        }
                        Err(e) => {
                            warn!("encountered error: {:?}", e);
                        }
                    }
                }
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
            _ => {}
        };
    });
}
