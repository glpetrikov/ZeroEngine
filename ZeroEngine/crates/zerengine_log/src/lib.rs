pub use tracing::{debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span};
use tracing_subscriber::{EnvFilter, fmt};

pub fn init() {
	let filter = EnvFilter::builder()
		.with_default_directive(tracing::Level::TRACE.into())
		.from_env_lossy()
		.add_directive("calloop=off".parse().unwrap())
		.add_directive("winit=warn".parse().unwrap())
		.add_directive("sctk=warn".parse().unwrap());

	let registry = fmt()
		.with_env_filter(filter)
		.with_target(true)
		.with_file(true)
		.with_line_number(true)
		.with_thread_names(true)
		.with_thread_ids(true)
		.with_ansi(std::io::IsTerminal::is_terminal(&std::io::stdout()))
		.with_timer(fmt::time::uptime())
		.compact();

	if let Err(e) = registry.try_init() {
		eprintln!("Failed to initialize ZeroEngine Logger: {e}");
	}
}
