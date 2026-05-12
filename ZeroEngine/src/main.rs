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
	use syspeek::SystemInfo;
	use terminal_size::{Width, terminal_size};

	// Collect system info
	let info = SystemInfo::refresh();
	let sys_lines = vec![
		format!("OS:      {}", info.os.long_version()),
		format!("Kernel:  {}", info.os.kernel_version()),
		format!("CPU:     {} ({} cores)", info.cpu.name(), info.cpu.cores()),
		match &info.gpu {
			Some(gpu) => format!("GPU:     {}", gpu.name()),
			None => "GPU:     Unknown".to_string(),
		},
		format!("RAM:     {:.1} / {:.1} GB", info.ram.used_gb(), info.ram.total_gb()),
		format!("Uptime:  {}", info.os.uptime_formatted()),
		format!("Version: v{}", env!("CARGO_PKG_VERSION")),
	];

	if let Some((Width(w), _)) = terminal_size() {
		if w < 100 {
			println!(
				r#"
# Terminal too narrow for full banner
▗▄▄▄▄▖▗▄▄▄▖▗▄▄▖  ▗▄▖ ▗▄▄▄▖▗▖  ▗▖ ▗▄▄▖▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖
   ▗▞▘▐▌   ▐▌ ▐▌▐▌ ▐▌▐▌   ▐▛▚▖▐▌▐▌     █  ▐▛▚▖▐▌▐▌
 ▗▞▘  ▐▛▀▀▘▐▛▀▚▖▐▌ ▐▌▐▛▀▀▘▐▌ ▝▜▌▐▌▝▜▌  █  ▐▌ ▝▜▌▐▛▀▀▘
▐▙▄▄▄▖▐▙▄▄▖▐▌ ▐▌▝▚▄▞▘▐▙▄▄▖▐▌  ▐▌▝▚▄▞▘▗▄█▄▖▐▌  ▐▌▐▙▄▄▖
"#
			);
			for line in &sys_lines {
				println!("  {}", line);
			}
		} else {
			let banner_lines: Vec<&str> = vec![
				"███████╗███████╗██████╗  ██████╗ ███████╗███╗   ██╗ ██████╗ ██╗███╗   ██╗███████╗",
				"╚══███╔╝██╔════╝██╔══██╗██╔═══██╗██╔════╝████╗  ██║██╔════╝ ██║████╗  ██║██╔════╝",
				"  ███╔╝ █████╗  ██████╔╝██║   ██║█████╗  ██╔██╗ ██║██║  ███╗██║██╔██╗ ██║█████╗  ",
				" ███╔╝  ██╔══╝  ██╔══██╗██║   ██║██╔══╝  ██║╚██╗██║██║   ██║██║██║╚██╗██║██╔══╝  ",
				"███████╗███████╗██║  ██║╚██████╔╝███████╗██║ ╚████║╚██████╔╝██║██║ ╚████║███████╗",
				"╚══════╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝╚═╝  ╚═══╝╚══════╝",
				"                                                                                 ",
			];

			for (i, banner_line) in banner_lines.iter().enumerate() {
				if let Some(sys_line) = sys_lines.get(i) {
					println!("{}    {}", banner_line, sys_line);
				} else {
					println!("{}", banner_line);
				}
			}
		}
	} else {
		println!("ZeroEngine v{}", env!("CARGO_PKG_VERSION"));
	}
}
