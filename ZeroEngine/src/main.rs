use timetrace::*;
use winit::event_loop::{ControlFlow, EventLoop};
use zerengine_app::App;

// #[profile_session("Main Loop", "trace.json")]
#[profile_function]
fn main() {
	zerengine_log::init();

	zerengine_log::debug!("Event Loop creating");
	let event_loop = EventLoop::new().unwrap();

	zerengine_log::debug!("Event Loop setting control flow");
	event_loop.set_control_flow(ControlFlow::Poll);

	zerengine_log::debug!("Creating App");
	let mut app = App::default();

	zerengine_log::debug!("Running App");
	event_loop.run_app(&mut app).unwrap();
}
