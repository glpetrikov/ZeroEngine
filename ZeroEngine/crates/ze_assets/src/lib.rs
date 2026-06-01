use std::{
	borrow::Cow,
	fs,
	path::{Component, Path, PathBuf},
};

use anyhow::{Result, anyhow, bail};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetSource {
	Engine,
	Game,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetRef {
	pub source: AssetSource,
	pub path: String,
}

impl AssetRef {
	pub fn engine(path: impl Into<String>) -> Self {
		Self {
			source: AssetSource::Engine,
			path: path.into(),
		}
	}

	pub fn game(path: impl Into<String>) -> Self {
		Self {
			source: AssetSource::Game,
			path: path.into(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct ResourceManager {
	game_assets_root: PathBuf,
}

impl ResourceManager {
	pub fn new(game_assets_root: impl Into<PathBuf>) -> Self {
		Self {
			game_assets_root: game_assets_root.into(),
		}
	}

	pub fn for_runtime(fallback_assets_root: impl Into<PathBuf>) -> Self {
		let fallback_assets_root = fallback_assets_root.into();
		let exe_assets_root = std::env::current_exe()
			.ok()
			.and_then(|path| path.parent().map(|parent| parent.join("assets")));

		if let Some(exe_assets_root) = exe_assets_root
			&& exe_assets_root.exists()
		{
			return Self::new(exe_assets_root);
		}

		Self::new(fallback_assets_root)
	}

	pub fn game_assets_root(&self) -> &Path { &self.game_assets_root }

	pub fn bytes(&self, asset: &AssetRef) -> Result<Cow<'static, [u8]>> {
		match asset.source {
			AssetSource::Engine => Ok(Cow::Borrowed(self.engine_bytes(&asset.path)?)),
			AssetSource::Game => Ok(Cow::Owned(self.game_bytes(&asset.path)?)),
		}
	}

	pub fn string(&self, asset: &AssetRef) -> Result<Cow<'static, str>> {
		match asset.source {
			AssetSource::Engine => Ok(Cow::Borrowed(self.engine_string(&asset.path)?)),
			AssetSource::Game => Ok(Cow::Owned(self.game_string(&asset.path)?)),
		}
	}

	pub fn game_bytes(&self, path: &str) -> Result<Vec<u8>> { Ok(fs::read(self.resolve_game_path(path)?)?) }

	pub fn game_string(&self, path: &str) -> Result<String> { Ok(fs::read_to_string(self.resolve_game_path(path)?)?) }

	pub fn engine_bytes(&self, path: &str) -> Result<&'static [u8]> {
		match path {
			"shaders/engine/sprite.wgsl" | "shaders/sprite.wgsl" => {
				Ok(include_bytes!(concat!(env!("OUT_DIR"), "/engine_sprite.wgsl")))
			}
			_ => bail!("unknown engine asset: {path}"),
		}
	}

	pub fn engine_string(&self, path: &str) -> Result<&'static str> {
		match path {
			"shaders/engine/sprite.wgsl" | "shaders/sprite.wgsl" => {
				Ok(include_str!(concat!(env!("OUT_DIR"), "/engine_sprite.wgsl")))
			}
			_ => bail!("unknown engine asset: {path}"),
		}
	}

	pub fn compile_game_shaders(&self) -> Result<()> {
		let source_root = self.game_assets_root.join("shaders").join("game");
		let target_root = self.game_assets_root.join(".compiled").join("shaders").join("game");

		if !source_root.exists() {
			return Ok(());
		}

		fs::create_dir_all(&target_root)?;

		let compiler = wesl::Wesl::new(&source_root);
		compile_wesl_directory(&compiler, &source_root, &target_root, &source_root)
	}

	fn resolve_game_path(&self, path: &str) -> Result<PathBuf> {
		let relative = Path::new(path);

		if relative.is_absolute() {
			bail!("asset path must be relative: {path}");
		}

		for component in relative.components() {
			if matches!(
				component,
				Component::ParentDir | Component::RootDir | Component::Prefix(_)
			) {
				bail!("asset path cannot leave assets root: {path}");
			}
		}

		let resolved = self
			.resolve_compiled_game_shader_path(relative)
			.unwrap_or_else(|| self.game_assets_root.join(relative));
		if !resolved.exists() {
			return Err(anyhow!("asset not found: {}", resolved.display()));
		}

		Ok(resolved)
	}

	fn resolve_compiled_game_shader_path(&self, relative: &Path) -> Option<PathBuf> {
		if relative.extension().and_then(|extension| extension.to_str()) != Some("wgsl") {
			return None;
		}

		let shader_path = relative.strip_prefix(Path::new("shaders").join("game")).ok()?;
		Some(
			self.game_assets_root
				.join(".compiled")
				.join("shaders")
				.join("game")
				.join(shader_path),
		)
	}
}

fn compile_wesl_directory(
	compiler: &wesl::Wesl<wesl::StandardResolver>,
	source_root: &Path,
	target_root: &Path,
	directory: &Path,
) -> Result<()> {
	for entry in fs::read_dir(directory)? {
		let entry = entry?;
		let source_path = entry.path();

		if source_path.is_dir() {
			compile_wesl_directory(compiler, source_root, target_root, &source_path)?;
			continue;
		}

		if source_path.extension().and_then(|extension| extension.to_str()) != Some("wesl") {
			continue;
		}

		let relative = source_path.strip_prefix(source_root)?;
		let module_path = wesl_module_path(relative)?;
		let compiled = compiler.compile(&module_path.parse()?)?.to_string();

		let mut target_path = target_root.join(relative);
		target_path.set_extension("wgsl");
		if let Some(parent) = target_path.parent() {
			fs::create_dir_all(parent)?;
		}
		fs::write(target_path, compiled)?;
	}

	Ok(())
}

fn wesl_module_path(relative: &Path) -> Result<String> {
	let mut module = String::from("package");

	for component in relative.with_extension("").components() {
		let Component::Normal(part) = component else {
			bail!("invalid WESL module path: {}", relative.display());
		};
		let Some(part) = part.to_str() else {
			bail!("non-utf8 WESL module path: {}", relative.display());
		};
		module.push_str("::");
		module.push_str(part);
	}

	Ok(module)
}

pub fn copy_game_assets_to_target(
	source_assets_dir: impl AsRef<Path>,
	target_assets_dir: impl AsRef<Path>,
) -> Result<()> {
	fn copy_dir(source: &Path, target: &Path) -> Result<()> {
		fs::create_dir_all(target)?;

		for entry in fs::read_dir(source)? {
			let entry = entry?;
			let source_path = entry.path();
			let target_path = target.join(entry.file_name());

			if source_path.is_dir() {
				copy_dir(&source_path, &target_path)?;
			} else {
				if let Some(parent) = target_path.parent() {
					fs::create_dir_all(parent)?;
				}
				fs::copy(&source_path, &target_path)?;
			}
		}

		Ok(())
	}

	copy_dir(source_assets_dir.as_ref(), target_assets_dir.as_ref())
}
