use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[macro_use]
extern crate log;

use env_logger::Env;

use zui::{
    Zui, SceneHandle,
};

mod render_state;
use render_state::RenderState;

mod main_scene;
use main_scene::MainScene;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SceneIdentifier {
    StartMenu,
    OptionsMenu,
    GameScene,
}

#[derive(Clone, Copy)]
pub enum UiMessage {
    StartMenuMessage(StartMenuMessage),
    OptionsMenuMessage(OptionsMenuMessage),
    GoToScene(SceneIdentifier),
    SetCounter(u64),
    IncrementCounter(u64),
    Exit,
}

#[derive(Clone, Copy)]
pub enum StartMenuMessage {
    StartClicked,
}

#[derive(Clone, Copy)]
pub enum OptionsMenuMessage {
    MasterVolumeChanged(f32),
    MusicVolumeChanged(f32),
    SoundEffectsVolumeChanged(f32),

    BackClicked,
}

fn main() {
    // configuring log
    std::env::set_var("RUST_BACKTRACE", "1");
    let env = Env::default().filter_or("MY_LOG_LEVEL", "zui=trace");
    env_logger::init_from_env(env);

    info!("starting!");

    // winit init
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();

    // render init
    let mut render_state = pollster::block_on(RenderState::new(&window));

    // zui
    let viewport_dimensions_px = window.inner_size();
    let mut zui = Zui::new(
        render_state.device(),
        render_state.surface_configuration(),
        viewport_dimensions_px,
    )
    .unwrap();

    // setting up the scenes
    let mut scene_handle = SceneHandle::new(Box::new(MainScene::new()));

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
                        info!("resized to: {physical_size:?}");
                        render_state.resize(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(*new_inner_size);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                // virtual_keycode: Some(VirtualKeyCode::X),
                                virtual_keycode: Some(virtual_key_code),
                                ..
                            },
                        ..
                    } => match virtual_key_code {
                        VirtualKeyCode::X => {
                            exit(control_flow);
                        }
                        VirtualKeyCode::F10 => {
                            zui.debug_try_save_typeface_texture_atlas("out.png");
                        }
                        VirtualKeyCode::Escape => {
                            // Do nothing
                        }
                        _ => {}
                    },
                    WindowEvent::CursorMoved {
                        ..
                    } => {
                        // Do nothing, handled in Zui::handle_winit_window_event
                    }
                    WindowEvent::ModifiersChanged(_) => {
                        // TODO
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {
                        // Do nothing, handled in Zui::handle_winit_window_event
                    }
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput {
                        ..
                    } => {
                        // Do nothing, handled in Zui::handle_winit_window_event
                    }
                    _ => {}
                }

                // event passthrough
                zui.handle_winit_window_event(
                    event,
                    Some(&mut scene_handle),
                );
            }
            Event::NewEvents(_) => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {
                // // TODO: Solving
                scene_handle.handle_message(UiMessage::IncrementCounter(1));

                scene_handle.update(
                    &mut zui.context_mut_typeface(),
                    render_state.device(),
                    render_state.queue(),
                );

                // solving user behaviour
                while let Some(message) = scene_handle.pop_external_message() {
                    match message {
                        UiMessage::Exit => exit(control_flow),
                        _ => {
                            println!("unhandled message!")
                        }
                    }
                }

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if !render_state.skip_rendering() {
                    // clearing the screen, this is where the world render pass would go
                    _ = render_state.render_clear();

                    // rendering the current scene, submits its own command encoder
                    zui.render_scene_handle(
                        &scene_handle, 
                        &render_state.device,
                        &render_state.surface_texture_view.as_ref().unwrap(),
                        &mut render_state.command_encoder.as_mut().unwrap(),
                    );

                    render_state.submit_command_encoder();

                    // presenting the final surface
                    match render_state.present() {
                        Err(e) => error!("wgpu::SurfaceError: {e}"),
                        Ok(_) => {}
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
