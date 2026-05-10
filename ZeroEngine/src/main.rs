use winit::event_loop::{ControlFlow, EventLoop};
use zerengine_app::*;

fn main() {
	render_banner();

	zerengine_log::init();

	zerengine_log::debug!("Event Loop creating");
	let event_loop = EventLoop::<CustomEvents>::with_user_event().build().unwrap();
	let event_loop_proxy = event_loop.create_proxy();

	zerengine_log::trace!("Event Loop setting control flow");
	event_loop.set_control_flow(ControlFlow::Poll);

	ctrlc::set_handler(move || {
		println!("\nReceived Ctrl+C, shutting down...");
		event_loop_proxy.send_event(CustomEvents::Shutdown).unwrap();
	})
	.unwrap();

	zerengine_log::debug!("Creating App");
	let mut app = App::default();

	zerengine_log::debug!("Running App");
	event_loop.run_app(&mut app).unwrap();
}

fn render_banner() {
	use terminal_size::{Width, terminal_size};
	if let Some((Width(w), _)) = terminal_size() {
		if w < 100 {
			println!(
				r#"
# If possible, increase the width of your terminal so that the full-size banner is displayed
▗▄▄▄▄▖▗▄▄▄▖▗▄▄▖  ▗▄▖ ▗▄▄▄▖▗▖  ▗▖ ▗▄▄▖▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖
   ▗▞▘▐▌   ▐▌ ▐▌▐▌ ▐▌▐▌   ▐▛▚▖▐▌▐▌     █  ▐▛▚▖▐▌▐▌
 ▗▞▘  ▐▛▀▀▘▐▛▀▚▖▐▌ ▐▌▐▛▀▀▘▐▌ ▝▜▌▐▌▝▜▌  █  ▐▌ ▝▜▌▐▛▀▀▘
▐▙▄▄▄▖▐▙▄▄▖▐▌ ▐▌▝▚▄▞▘▐▙▄▄▖▐▌  ▐▌▝▚▄▞▘▗▄█▄▖▐▌  ▐▌▐▙▄▄▖
ZeroEngine v{}
"#,
				env!("CARGO_PKG_VERSION")
			)
		} else {
			println!(
				r#"
███████╗███████╗██████╗  ██████╗ ███████╗███╗   ██╗ ██████╗ ██╗███╗   ██╗███████╗
╚══███╔╝██╔════╝██╔══██╗██╔═══██╗██╔════╝████╗  ██║██╔════╝ ██║████╗  ██║██╔════╝
  ███╔╝ █████╗  ██████╔╝██║   ██║█████╗  ██╔██╗ ██║██║  ███╗██║██╔██╗ ██║█████╗
 ███╔╝  ██╔══╝  ██╔══██╗██║   ██║██╔══╝  ██║╚██╗██║██║   ██║██║██║╚██╗██║██╔══╝
███████╗███████╗██║  ██║╚██████╔╝███████╗██║ ╚████║╚██████╔╝██║██║ ╚████║███████╗
╚══════╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝╚═╝  ╚═══╝╚══════╝
ZeroEngine v{}
"#,
				env!("CARGO_PKG_VERSION")
			);
		}
	} else {
		println!("ZeroEngine v{}", env!("CARGO_PKG_VERSION"));
	}
}
