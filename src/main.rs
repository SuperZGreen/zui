use winit::{
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode},
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

mod main_scene;
use main_scene::MainScene;

use crate::zui::SceneHandle;

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
        "resources/zui/fonts/Roboto-Regular.ttf",
        14,
        render_state.device(),
        render_state.queue(),
        render_state.surface_configuration(),
        resolution.width,
        resolution.height,
    )
    .unwrap();

    let main_scene = MainScene::new();
    let mut scene_handle = SceneHandle::new(main_scene, zui.aspect_ratio());

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
                        scene_handle.queue_widget_recreation();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(*new_inner_size);
                    }
                    WindowEvent::KeyboardInput { input : KeyboardInput {virtual_keycode: Some(VirtualKeyCode::X), ..}, ..} => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        zui.update_cursor_state(*position);
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
                if let Some(cursor_state) = zui.cursor_state() {
                    scene_handle.update(cursor_state, zui.aspect_ratio());
                }

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if !render_state.skip_rendering() {
                    // uploading
                    zui.renderer_upload(render_state.device(), &scene_handle);

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
