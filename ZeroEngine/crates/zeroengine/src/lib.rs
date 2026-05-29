mod hooks;

use winit::event_loop::{ControlFlow, EventLoop};
use zerengine_app::*;

pub fn run() {
	zerengine_log::init(); // TODO: add file logging

	zerengine_log::trace!("Setting up panic hook");
	hooks::panic_hook();
	// ===================================================
	// Creating Event Loop
	// ===================================================
	zerengine_log::debug!("Event Loop creating");
	let event_loop = EventLoop::<CustomEvents>::with_user_event().build().unwrap();
	let event_loop_proxy = event_loop.create_proxy();

	zerengine_log::trace!("Event Loop setting control flow");
	event_loop.set_control_flow(ControlFlow::Poll);

	render_banner();

	// ===================================================
	// Setup Ctrl+C hook
	// ===================================================
	hooks::ctrlc_hook(&event_loop_proxy);

	// ===================================================
	// Running App
	// ===================================================
	zerengine_log::debug!("Creating App");
	let mut app = match App::new() {
		Ok(app) => app,
		Err(error) => {
			zerengine_log::error!("Failed to create app: {error:?}");
			std::process::exit(1);
		}
	};

	zerengine_log::debug!("Running App");
	event_loop.run_app(&mut app).unwrap();
}

// ===================================================
// Banner render
// ===================================================
use owo_colors::OwoColorize;

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
				"{}",
				r#"
# Terminal too narrow for full banner
▗▄▄▄▄▖▗▄▄▄▖▗▄▄▖  ▗▄▖ ▗▄▄▄▖▗▖  ▗▖ ▗▄▄▖▗▄▄▄▖▗▖  ▗▖▗▄▄▄▖
   ▗▞▘▐▌   ▐▌ ▐▌▐▌ ▐▌▐▌   ▐▛▚▖▐▌▐▌     █  ▐▛▚▖▐▌▐▌
 ▗▞▘  ▐▛▀▀▘▐▛▀▚▖▐▌ ▐▌▐▛▀▀▘▐▌ ▝▜▌▐▌▝▜▌  █  ▐▌ ▝▜▌▐▛▀▀▘
▐▙▄▄▄▖▐▙▄▄▖▐▌ ▐▌▝▚▄▞▘▐▙▄▄▖▐▌  ▐▌▝▚▄▞▘▗▄█▄▖▐▌  ▐▌▐▙▄▄▖
"#
				.bright_black()
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
					let banner_text = format!("{banner_line:<86}");
					let colored_banner = banner_text.bright_black();
					println!("{colored_banner}    {sys_line}");
				} else {
					println!("{}", banner_line.bright_black());
				}
			}
		}
	} else {
		println!("ZeroEngine v{}", env!("CARGO_PKG_VERSION"));
	}
}
