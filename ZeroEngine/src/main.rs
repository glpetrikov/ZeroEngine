use winit::event_loop::{ControlFlow, EventLoop};

use zerengine_app::App;

use timetrace::*;

// #[profile_session("Main Loop", "trace.json")]
#[profile_function]
fn main() {
	let event_loop = EventLoop::new().unwrap();

	event_loop.set_control_flow(ControlFlow::Poll);

	let mut app = App::default();
	event_loop.run_app(&mut app).unwrap();
}
