//! Model viewer initialization and event processing
//!
//! Provides the functionality to create a window and perform basic viewing
//! with programmed models.

use std::error;

use fj_host::Watcher;
use fj_operations::shape_processor::ShapeProcessor;
use fj_viewer::{
    camera::Camera,
    graphics::{self, DrawConfig, Renderer},
    input,
    screen::{NormalizedPosition, Screen as _, Size},
};
use futures::executor::block_on;
use tracing::{trace, warn};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
};

use crate::window::{self, Window};

/// Initializes a model viewer for a given model and enters its process loop.
pub fn run(
    watcher: Watcher,
    shape_processor: ShapeProcessor,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    let mut previous_cursor = None;
    let mut held_mouse_button = None;

    let mut input_handler = input::Handler::default();
    let mut renderer = block_on(Renderer::new(&window))?;

    let mut draw_config = DrawConfig::default();

    let mut shape = None;
    let mut camera = None;

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        if let Some(new_shape) = watcher.receive() {
            match shape_processor.process(&new_shape) {
                Ok(new_shape) => {
                    renderer.update_geometry(
                        (&new_shape.mesh).into(),
                        (&new_shape.debug_info).into(),
                        new_shape.aabb,
                    );

                    if camera.is_none() {
                        camera = Some(Camera::new(&new_shape.aabb));
                    }

                    shape = Some(new_shape);
                }
                Err(err) => {
                    // Can be cleaned up, once `Report` is stable:
                    // https://doc.rust-lang.org/std/error/struct.Report.html

                    println!("Shape processing error: {}", err);

                    let mut current_err = &err as &dyn error::Error;
                    while let Some(err) = current_err.source() {
                        println!();
                        println!("Caused by:");
                        println!("    {}", err);

                        current_err = err;
                    }
                }
            }
        }

        //

        if let Event::WindowEvent {
            event: window_event,
            ..
        } = &event
        {
            //
            // Note: In theory we could/should check if `egui` wants "exclusive" use
            //       of this event here.
            //
            //       But with the current integration with Fornjot we're kinda blurring
            //       the lines between "app" and "platform", so for the moment we pass
            //       every event to both `egui` & Fornjot.
            //
            //       The primary visible impact of this currently is that if you drag
            //       a title bar that overlaps the model then both the model & window
            //       get moved.
            //
            // TODO: Revisit this.
            //
            // TODO: Encapsulate the egui state/context access better.
            //
            renderer
                .egui
                .winit_state
                .on_event(&renderer.egui.context, window_event);
        }

        // Window events
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let size = Size {
                    width: size.width,
                    height: size.height,
                };
                renderer.handle_resize(size);
            }
            Event::MainEventsCleared => {
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let (Some(shape), Some(camera)) = (&shape, &mut camera) {
                    camera.update_planes(&shape.aabb);

                    if let Err(err) =
                        renderer.draw(camera, &mut draw_config, window.window())
                    {
                        warn!("Draw error: {}", err);
                    }
                }
            }
            _ => {}
        }

        // Viewer events
        let event = match event {
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(virtual_key_code),
                                ..
                            },
                        ..
                    },
                ..
            } => match virtual_key_code {
                VirtualKeyCode::Escape => Some(input::Event::Exit),
                VirtualKeyCode::Key1 => Some(input::Event::ToggleModel),
                VirtualKeyCode::Key2 => Some(input::Event::ToggleMesh),
                VirtualKeyCode::Key3 => Some(input::Event::ToggleDebug),

                _ => None,
            },
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let [width, height] = window.size().as_f64();
                let aspect_ratio = width / height;

                // Cursor position in normalized coordinates (-1 to +1) with
                // aspect ratio taken into account.
                let current = NormalizedPosition {
                    x: position.x / width * 2. - 1.,
                    y: -(position.y / height * 2. - 1.) / aspect_ratio,
                };
                let event = match (previous_cursor, held_mouse_button) {
                    (Some(previous), Some(button)) => match button {
                        MouseButton::Left => {
                            Some(input::Event::Orbit { previous, current })
                        }
                        MouseButton::Right => {
                            Some(input::Event::Pan { previous, current })
                        }
                        _ => None,
                    },
                    _ => None,
                };
                previous_cursor = Some(current);
                event
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                match state {
                    ElementState::Pressed => held_mouse_button = Some(button),
                    ElementState::Released => held_mouse_button = None,
                };
                match (&shape, &camera, button) {
                    (
                        Some(shape),
                        Some(camera),
                        MouseButton::Left | MouseButton::Right,
                    ) => Some(input::Event::FocusPoint(
                        camera.focus_point(previous_cursor, &shape.mesh),
                    )),
                    _ => None,
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => Some(input::Event::Zoom(match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    (y as f64) * ZOOM_FACTOR_LINE
                }
                MouseScrollDelta::PixelDelta(PhysicalPosition {
                    y, ..
                }) => y * ZOOM_FACTOR_PIXEL,
            })),
            _ => None,
        };

        if let (Some(event), Some(camera)) = (event, &mut camera) {
            input_handler.handle_event(event, camera, &mut actions);
        }

        if actions.exit {
            *control_flow = ControlFlow::Exit;
        }
        if actions.toggle_model {
            draw_config.draw_model = !draw_config.draw_model;
        }
        if actions.toggle_mesh {
            draw_config.draw_mesh = !draw_config.draw_mesh;
        }
        if actions.toggle_debug {
            draw_config.draw_debug = !draw_config.draw_debug;
        }
    });
}

/// Error in main loop
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error initializing window
    #[error("Error initializing window")]
    WindowInit(#[from] window::Error),

    /// Error initializing graphics
    #[error("Error initializing graphics")]
    GraphicsInit(#[from] graphics::InitError),
}

/// Affects the speed of zoom movement given a scroll wheel input in lines.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR_LINE: f64 = 0.075;

/// Affects the speed of zoom movement given a scroll wheel input in pixels.
///
/// Smaller values will move the camera less with the same input.
/// Larger values will move the camera more with the same input.
const ZOOM_FACTOR_PIXEL: f64 = 0.005;
