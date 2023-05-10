use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
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

mod options_scene;
use options_scene::OptionsScene;

use crate::zui::SceneHandle;

#[derive(Clone, Copy, Debug)]
pub enum SceneIdentifier {
    StartMenu,
    OptionsMenu,
}

#[derive(Clone, Copy)]
pub enum UiMessage {
    StartMenuMessage(StartMenuMessage),
    OptionsMenuMessage(OptionsMenuMessage),
    GoToScene(SceneIdentifier),
    Exit,
}

#[derive(Clone, Copy)]
pub enum StartMenuMessage {
    StartClicked,
}

#[derive(Clone, Copy)]
pub enum OptionsMenuMessage {
    BackClicked,
}

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
        render_state.device(),
        render_state.queue(),
        render_state.surface_configuration(),
        resolution.width,
        resolution.height,
    )
    .unwrap();

    let mut main_scene_handle: SceneHandle<MainScene> =
        SceneHandle::new(MainScene::new(), zui.font(), zui.aspect_ratio());
    let mut options_scene_handle =
        SceneHandle::new(OptionsScene::new(), zui.font(), zui.aspect_ratio());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        exit(control_flow);
                    }
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize(physical_size);
                        zui.resize(physical_size.width, physical_size.height);
                        main_scene_handle.queue_widget_recreation();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(*new_inner_size);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::X),
                                ..
                            },
                        ..
                    } => {
                        exit(control_flow);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        zui.update_cursor_position(*position);
                    }
                    WindowEvent::ModifiersChanged(_) => {
                        // TODO
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {
                        zui.cursor_left();
                    }
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { button, state, .. } => {
                        zui.mouse_input(*button, *state);
                    }
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
                main_scene_handle.update(zui.cursor_state(), zui.font(), zui.aspect_ratio());
                zui.update();

                // solving user behaviour
                while let Some(message) = main_scene_handle.pop_external_message() {
                    match message {
                        UiMessage::GoToScene(scene_identifier) => {
                            println!("going to scene: {:?}", scene_identifier)
                        }
                        UiMessage::Exit => exit(control_flow),
                        _ => {println!("unhandled message!")},
                    }
                }

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if !render_state.skip_rendering() {
                    // uploading
                    zui.upload_vertices(render_state.device(), &main_scene_handle);

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

fn exit(control_flow: &mut ControlFlow) {
    info!("exiting via exit function!");
    *control_flow = ControlFlow::Exit;
}
