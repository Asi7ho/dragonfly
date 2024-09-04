use winit::event_loop::{ControlFlow, EventLoop};

mod core;

mod dragonfly;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    event_loop.set_control_flow(ControlFlow::default());

    let mut app = dragonfly::Dragonfly::default();
    match event_loop.run_app(&mut app) {
        Ok(_) => {}
        Err(e) => log::error!("Failed to run app: {:?}", e),
    };
}
