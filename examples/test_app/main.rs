use std::time::Duration;

use scenes::BaseSceneMessage;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Fullscreen, WindowBuilder},
};

#[macro_use]
extern crate log;

use zui::{SceneHandle, Zui};

mod render_state;
use render_state::RenderState;

use crate::scenes::BaseScene;

mod scenes;

#[derive(Clone, Copy, Debug)]
pub enum UiMessage {
    BaseSceneMessage(BaseSceneMessage),
    ToggleFullscreen,
    Exit,
}

fn main() {
    // configuring log
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .filter_module("zui", log::LevelFilter::Trace)
        .init();

    info!("starting!");

    // winit init
    let event_loop = EventLoop::new().unwrap();
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
        &render_state.queue,
        render_state.surface_configuration(),
        viewport_dimensions_px.into(),
    )
    .unwrap();

    // setting up the scenes
    let mut scene_handle = SceneHandle::new(Box::new(BaseScene::new()));
    let mut old_instant = std::time::Instant::now();

    _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::RedrawRequested => {
                        // getting the new frame time
                        let current_instant = std::time::Instant::now();
                        let frame_time = (current_instant - old_instant).as_secs_f32();
                        old_instant = current_instant;

                        // updating the scene to show the frame time
                        scene_handle.handle_message(UiMessage::BaseSceneMessage(
                            BaseSceneMessage::UpdateFrameTime(frame_time),
                        ));

                        // updating the scene handle
                        scene_handle.update(
                            &mut zui.context_mut_typeface(),
                            &render_state.device,
                            &render_state.queue,
                        );

                        // solving user behaviour
                        while let Some(message) = scene_handle.pop_external_message() {
                            match message {
                                UiMessage::Exit => {
                                    exit(elwt);
                                }
                                UiMessage::ToggleFullscreen => {
                                    let fullscreen = match window.fullscreen() {
                                        Some(_) => None,
                                        None => Some(Fullscreen::Borderless(None)),
                                    };

                                    window.set_fullscreen(fullscreen);
                                }
                                _ => {
                                    println!("unhandled message!");
                                }
                            }
                        }

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
                        } else {
                            std::thread::sleep(Duration::from_millis(1));
                        }

                        // NOTE: this poll appears to be required to clean up dropped assets
                        // used by the program. This appears to fix memory leaks created by
                        // creating image textures and other wgpu resource-creating calls...
                        render_state.device.poll(wgpu::Maintain::Poll);

                        window.request_redraw();
                    }
                    WindowEvent::CloseRequested => {
                        exit(elwt);
                    }
                    WindowEvent::Resized(physical_size) => {
                        info!("resized to: {physical_size:?}");
                        render_state.resize(physical_size);
                    }
                    // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //     render_state.resize(*new_inner_size);
                    // }
                    // WindowEvent::KeyboardInput {
                    //     input:
                    //         KeyboardInput {
                    //             // virtual_keycode: Some(VirtualKeyCode::X),
                    //             virtual_keycode: Some(virtual_key_code),
                    //             state: winit::event::ElementState::Pressed,
                    //             ..
                    //         },
                    //     ..
                    // } => match virtual_key_code {
                    //     VirtualKeyCode::X => {
                    //         exit(control_flow);
                    //     }
                    //     VirtualKeyCode::A => {
                    //         scene_handle.handle_message(UiMessage::BaseSceneMessage(
                    //             BaseSceneMessage::AddSidebarText,
                    //         ));
                    //     }
                    //     VirtualKeyCode::D => {
                    //         scene_handle.handle_message(UiMessage::BaseSceneMessage(
                    //             BaseSceneMessage::ClearSidebarText,
                    //         ));
                    //     }
                    //     VirtualKeyCode::Key1 => {
                    //         scene_handle.handle_message(UiMessage::BaseSceneMessage(
                    //             BaseSceneMessage::ChangeScene(SceneIdentifier::ContainerDemo),
                    //         ));
                    //     }
                    //     VirtualKeyCode::Key2 => {
                    //         scene_handle.handle_message(UiMessage::BaseSceneMessage(
                    //             BaseSceneMessage::ChangeScene(SceneIdentifier::TextDemo),
                    //         ));
                    //     }
                    //     VirtualKeyCode::Key3 => {
                    //         scene_handle.handle_message(UiMessage::BaseSceneMessage(
                    //             BaseSceneMessage::ChangeScene(SceneIdentifier::ButtonDemo),
                    //         ));
                    //     }
                    //     VirtualKeyCode::F10 => {
                    //         zui.debug_try_save_typeface_texture_atlas("out.png");
                    //     }
                    //     VirtualKeyCode::Escape => {
                    //         // Do nothing
                    //     }
                    //     _ => {}
                    // },
                    WindowEvent::CursorMoved { position, .. } => {
                        // Do nothing, handled in Zui::handle_winit_window_event
                        // TODO: hacky
                        let mut position = *position;
                        position.y =
                            zui.context().viewport_dimensions_px.height as f64 - position.y;
                        scene_handle.handle_message(UiMessage::BaseSceneMessage(
                            BaseSceneMessage::MoveCursor(Some(position)),
                        ));
                    }
                    WindowEvent::ModifiersChanged(_) => {
                        // TODO
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {
                        // Do nothing, handled in Zui::handle_winit_window_event
                    }
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {
                        // Do nothing, handled in Zui::handle_winit_window_event
                    }
                    _ => {}
                }

                // event passthrough
                zui.handle_winit_window_event(event, Some(&mut scene_handle));
            }
            Event::DeviceEvent {
                event: DeviceEvent::Key(rke),
                ..
            } => {
                println!("rke: {rke:?}");
            }

            _ => {}
        };
    });
}

fn exit(elwt: &EventLoopWindowTarget<()>) {
    info!("exiting via exit function!");
    elwt.exit();
}
