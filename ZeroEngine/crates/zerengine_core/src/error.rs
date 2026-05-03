#[derive(Debug)]
pub enum ZeroError {
	WindowCreationFailed(String),
	RendererInitFailed(String),
	AssetNotFound(String),
	ScriptError(String),
	Unknown(String),
}

impl std::fmt::Display for ZeroError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ZeroError::WindowCreationFailed(s) => write!(f, "Window creation failed: {s}"),
			ZeroError::RendererInitFailed(s) => write!(f, "Renderer init failed: {s}"),
			ZeroError::AssetNotFound(s) => write!(f, "Asset not found: {s}"),
			ZeroError::ScriptError(s) => write!(f, "Script error: {s}"),
			ZeroError::Unknown(s) => write!(f, "Unknown error: {s}"),
		}
	}
}

impl std::error::Error for ZeroError {}

pub type ZResult<T> = Result<T, ZeroError>;
