use zerengine_app::CustomEvents;

pub(crate) fn ctrlc_hook(event_loop_proxy: &winit::event_loop::EventLoopProxy<CustomEvents>) {
	let ctrlc_proxy = event_loop_proxy.clone();

	ctrlc::set_handler(move || {
		println!("\nReceived Ctrl+C, shutting down...");

		let _ = ctrlc_proxy.send_event(CustomEvents::Shutdown);
	})
	.unwrap();
}

pub(crate) fn panic_hook() {
	std::panic::set_hook(Box::new(|panic_info| {
		let location = panic_info
			.location()
			.map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
			.unwrap_or_else(|| "unknown".into());

		let message = panic_info
			.payload()
			.downcast_ref::<&str>()
			.copied()
			.or_else(|| panic_info.payload().downcast_ref::<String>().map(|s| s.as_str()))
			.unwrap_or("unknown panic");

		const RED_BOLD: &str = "\x1b[1;31m";
        const RESET: &str = "\x1b[0m";

        println!(
            "{}ZEROENGINE FATAL ERROR{} at {}: {}",
            RED_BOLD,
            RESET,
            location,
            message
        );
	}));
}
