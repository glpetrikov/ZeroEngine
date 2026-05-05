use timetrace::*;
use winit::event_loop::{ControlFlow, EventLoop};
use zerengine_app::*;

// #[profile_session("Main Loop", "trace.json")]
#[profile_function]
fn main() {
	let event_loop = EventLoop::<CustomEvents>::with_user_event().build().unwrap();
	let event_loop_proxy = event_loop.create_proxy();

	event_loop.set_control_flow(ControlFlow::Poll);

	ctrlc::set_handler(move || {
		println!("Received Ctrl+C, shutting down...");
		event_loop_proxy.send_event(CustomEvents::Shutdown).unwrap();
	})
	.unwrap();

	let mut app = App::default();
	event_loop.run_app(&mut app).unwrap();

	println!("{:?}", Input::get_mouse_pos());
}
