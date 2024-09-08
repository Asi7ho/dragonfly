use std::sync::Arc;

use pollster;

use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::core;

/// The application state.
///
/// Contains the window and the graphics context.
#[derive(Debug, Default)]
pub struct Dragonfly {
    /// The graphics context.
    ///
    /// Contains the data necessary to render the scene.
    context: Option<core::Context>,

    /// The window.
    ///
    /// The window is the platform-specific structure that holds the window
    /// and its associated resources.
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for Dragonfly {
    /// Handles the `Resumed` event, which is called when the event loop is
    /// started.
    ///
    /// If the window is `None`, the window is created and the context is
    /// initialized.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Dragonfly")
                .with_min_inner_size(winit::dpi::PhysicalSize {
                    width: 1980,
                    height: 1020,
                });
            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window."),
            );

            let context = pollster::block_on(core::Context::new(&window));
            self.window = Some(window);
            self.context = Some(context);
        }
    }

    /// Handles a window event.
    ///
    /// This method will be called when an event occurs on the window.
    ///
    /// # Errors
    ///
    /// Returns an error if a `RedrawRequested` event is received and the
    /// context cannot be rendered.
    ///
    /// # Panics
    ///
    /// Panics if the window id is not the same as the id of the window stored
    /// in the context.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                match self.context.as_mut().unwrap().render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        let size = self.context.as_ref().unwrap().size;
                        self.context.as_mut().unwrap().resize(size);
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    // All other errors (Outdated, Timeout) should be resolved
                    // by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            WindowEvent::Resized(physical_size) => {
                self.context.as_mut().unwrap().resize(physical_size);
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                self.context
                    .as_mut()
                    .unwrap()
                    .resize(self.window.as_ref().unwrap().inner_size());
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        state,
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Space),
                        ..
                    },
                ..
            } => {
                if state == winit::event::ElementState::Released {
                    let fig_idx = self.context.as_ref().unwrap().fig_idx;
                    let new_fig_idx = (fig_idx + 1) % 6;

                    self.context.as_mut().unwrap().fig_idx = new_fig_idx;

                    let figure = core::Figure::get_figure(new_fig_idx);
                    let (vertices, indices) = figure.get_vertices_and_indices();

                    self.context.as_mut().unwrap().vertex_buffer =
                        self.context.as_mut().unwrap().device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(vertices),
                                usage: wgpu::BufferUsages::VERTEX,
                            },
                        );
                    self.context.as_mut().unwrap().num_vertices = vertices.len() as u32;

                    self.context.as_mut().unwrap().index_buffer =
                        self.context.as_mut().unwrap().device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Index Buffer"),
                                contents: bytemuck::cast_slice(indices),
                                usage: wgpu::BufferUsages::INDEX,
                            },
                        );
                    self.context.as_mut().unwrap().num_indices = indices.len() as u32;
                }

                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}
