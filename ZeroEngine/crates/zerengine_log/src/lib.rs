pub use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{EnvFilter, fmt};

pub fn init() {
	let filter = EnvFilter::builder()
		.with_default_directive(tracing::Level::INFO.into())
		.from_env_lossy()
		.add_directive("calloop=off".parse().unwrap())
		.add_directive("winit=warn".parse().unwrap())
		.add_directive("sctk=warn".parse().unwrap());

	let _ = fmt()
		.with_env_filter(filter)
		.with_target(true)
		.with_file(true)
		.with_line_number(true)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_ansi(true)
		.with_timer(fmt::time::uptime())
		.compact()
		.try_init();
}
