use std::{env, path::Path};

#[derive(Debug, Clone)]
pub struct EnvInfo {
	shell: Option<String>,
	desktop: Option<String>,
	session_type: Option<String>,
	terminal: Option<String>,
	locale: Option<String>,
}

impl EnvInfo {
	pub fn refresh() -> Self {
		Self {
			shell: detect_shell(),
			desktop: env::var("XDG_CURRENT_DESKTOP").ok(),
			session_type: env::var("XDG_SESSION_TYPE").ok(),
			terminal: detect_terminal(),
			locale: detect_locale(),
		}
	}

	pub fn shell(&self) -> Option<&str> { self.shell.as_deref() }
	pub fn desktop(&self) -> Option<&str> { self.desktop.as_deref() }
	pub fn session_type(&self) -> Option<&str> { self.session_type.as_deref() }
	pub fn terminal(&self) -> Option<&str> { self.terminal.as_deref() }
	pub fn locale(&self) -> Option<&str> { self.locale.as_deref() }
}

fn detect_shell() -> Option<String> {
	let shell = env::var("SHELL").ok()?;
	let name = Path::new(&shell).file_name()?.to_str()?.to_string();

	Some(name)
}

fn detect_terminal() -> Option<String> {
	env::var("TERM_PROGRAM")
		.ok()
		.or_else(|| env::var("TERMINAL").ok())
		.or_else(|| env::var("TERM").ok())
}

fn detect_locale() -> Option<String> {
	env::var("LANG")
		.ok()
		.or_else(|| env::var("LC_ALL").ok())
		.or_else(|| env::var("LC_CTYPE").ok())
}
