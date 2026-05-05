use timetrace::*;
use winit::event_loop::{ControlFlow, EventLoop};
use zerengine_app::*;

// #[profile_session("Main Loop", "trace.json")]
#[profile_function]
fn main() {
	zerengine_log::init();

	zerengine_log::debug!("Event Loop creating");
	let event_loop = EventLoop::<CustomEvents>::with_user_event().build().unwrap();
	let event_loop_proxy = event_loop.create_proxy();

	zerengine_log::debug!("Event Loop setting control flow");
	event_loop.set_control_flow(ControlFlow::Poll);

	ctrlc::set_handler(move || {
		println!("Received Ctrl+C, shutting down...");
		event_loop_proxy.send_event(CustomEvents::Shutdown).unwrap();
	})
	.unwrap();

	zerengine_log::debug!("Creating App");
	let mut app = App::default();

	zerengine_log::debug!("Running App");
	event_loop.run_app(&mut app).unwrap();
}
