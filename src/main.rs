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
mod game_scene;
use game_scene::GameScene;

use crate::zui::SceneStore;

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
    info!("starting!");
    let mut frame_counter = 0u64;

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
    let viewport_dimensions_px = window.inner_size();
    let mut zui = Zui::new(
        render_state.device(),
        render_state.queue(),
        render_state.surface_configuration(),
        viewport_dimensions_px,
    )
    .unwrap();

    // setting up the scenes
    let mut scene_store = SceneStore::new();
    scene_store.add_scene(SceneIdentifier::StartMenu, Box::new(MainScene::new()));
    scene_store.add_scene(SceneIdentifier::OptionsMenu, Box::new(OptionsScene::new()));
    scene_store.add_scene(SceneIdentifier::GameScene, Box::new(GameScene::new()));
    _ = scene_store.change_scene(SceneIdentifier::StartMenu, &zui.context());

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
                        scene_store.current_scene_mut().unwrap().resize_scene(
                            &mut zui.context_mut_typeface(),
                            render_state.device(),
                            render_state.queue(),
                        );
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
                            _ = scene_store
                                .change_scene(SceneIdentifier::StartMenu, &zui.context());
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
                let current_scene_mut = scene_store.current_scene_mut();
                zui.handle_winit_window_event(
                    event,
                    current_scene_mut,
                );
            }
            Event::NewEvents(_) => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {
                // // TODO: Solving
                let current_scene_mut = scene_store.current_scene_mut().unwrap();
                current_scene_mut.handle_message(UiMessage::IncrementCounter(1));

                current_scene_mut.update(
                    &mut zui.context_mut_typeface(),
                    render_state.device(),
                    render_state.queue(),
                );

                // solving user behaviour
                let mut set_scene_destination = None;
                while let Some(message) = current_scene_mut.pop_external_message() {
                    match message {
                        UiMessage::GoToScene(scene_identifier) => {
                            println!("going to scene: {:?}", scene_identifier);
                            set_scene_destination = Some(scene_identifier);
                        }
                        UiMessage::Exit => exit(control_flow),
                        _ => {
                            println!("unhandled message!")
                        }
                    }
                }

                if let Some(scene_identifier) = set_scene_destination {
                    _ = scene_store.change_scene(scene_identifier, &zui.context());
                }

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if !render_state.skip_rendering() {
                    // clearing the screen, this is where the world render pass would go
                    _ = render_state.render_clear();

                    // rendering the current scene, submits its own command encoder
                    let current_scene = scene_store.current_scene().unwrap();
                    zui.render_scene_handle(current_scene, &mut render_state);

                    // presenting the final surface
                    match render_state.present() {
                        Err(e) => error!("wgpu::SurfaceError: {e}"),
                        Ok(_) => {}
                    }
                }

                // incrementing the frame counter for testing
                frame_counter += 1;
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
