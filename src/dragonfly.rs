use std::sync::Arc;

use pollster;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::core;

#[derive(Debug, Default)]
pub struct Dragonfly {
    context: Option<core::Context>,
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for Dragonfly {
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
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
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
                self.context.as_mut().unwrap().update_color =
                    state == winit::event::ElementState::Released;
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}
