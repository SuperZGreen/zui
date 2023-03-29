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

fn main() {
    info!("starting!");

    // configuring log
    std::env::set_var("RUST_BACKTRACE", "1");
    let env = Env::default().filter_or("MY_LOG_LEVEL", "iced_test=trace");
    env_logger::init_from_env(env);

    // winit init
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();

    // render init
    let mut render_state = pollster::block_on(RenderState::new(&window));

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
                match render_state.render() {
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
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
            _ => {}
        }
    });
}
