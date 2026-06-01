use std::{
	env, fs, io,
	path::{Component, Path, PathBuf},
	process::Command,
	thread,
	time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let workspace_root = workspace_root()?;
	let assets_dir = workspace_root.join("assets");
	let api_dir = workspace_root.join("ZeroEngine/api/cs");
	let scripts_dir = assets_dir.join("scripts");

	compile_engine_shaders(&assets_dir)?;
	emit_asset_rerun_files(&assets_dir)?;
	emit_cs_rerun_files(&scripts_dir)?;
	emit_cs_rerun_files(&api_dir)?;
	build_scripts_assembly(&workspace_root)?;
	stage_dist(&workspace_root)?;

	Ok(())
}

fn compile_engine_shaders(assets_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let shader_dir = assets_dir.join("shaders/engine");
	println!("cargo:rerun-if-changed={}", shader_dir.join("sprite.wesl").display());
	wesl::Wesl::new(shader_dir).build_artifact(&"package::sprite".parse()?, "engine_sprite");
	Ok(())
}

fn build_scripts_assembly(workspace_root: &Path) -> io::Result<()> {
	let project_path = workspace_root.join("assets/scripts/Scripts.csproj");
	let configuration = if is_optimized_profile()? { "Release" } else { "Debug" };

	println!("cargo:rerun-if-changed={}", project_path.display());

	let status = Command::new("dotnet")
		.arg("build")
		.arg(&project_path)
		.arg("--configuration")
		.arg(configuration)
		.arg("--nologo")
		.status()
		.map_err(|error| {
			io::Error::new(
				error.kind(),
				format!("failed to run dotnet build for {}: {error}", project_path.display()),
			)
		})?;

	if !status.success() {
		return Err(io::Error::other(format!(
			"dotnet build failed for {}",
			project_path.display()
		)));
	}

	Ok(())
}

fn stage_dist(workspace_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let profile_dir = active_profile_directory()?;
	let dist_dir = profile_dir.join("dist");
	let assets_dir = workspace_root.join("assets");
	let _lock = DistLock::acquire(&profile_dir)?;
	let staged_assets_dir = create_staged_assets_without_scripts(&profile_dir, &assets_dir)?;

	replace_directory(&dist_dir)?;
	copy_file(&workspace_root.join("NOTICE"), &dist_dir.join("NOTICE"))?;
	zepack::pack_directory(&staged_assets_dir, dist_dir.join("assets.zepack"))?;
	copy_file(
		&workspace_root.join("assets/scripts/bin/Scripts.dll"),
		&dist_dir.join("Scripts.dll"),
	)?;
	remove_dir_if_exists(&staged_assets_dir)?;

	Ok(())
}

fn emit_asset_rerun_files(directory: &Path) -> io::Result<()> {
	emit_rerun_files_filtered(directory, &|path| {
		!path.components().any(|component| {
			matches!(
				component,
				Component::Normal(name)
					if name == std::ffi::OsStr::new(".compiled")
						|| name == std::ffi::OsStr::new("bin")
						|| name == std::ffi::OsStr::new("obj")
			)
		})
	})
}

fn emit_cs_rerun_files(directory: &Path) -> io::Result<()> {
	emit_rerun_files_filtered(directory, &|path| {
		path.extension().and_then(|extension| extension.to_str()) == Some("cs")
	})
}

fn emit_rerun_files_filtered(directory: &Path, include_file: &impl Fn(&Path) -> bool) -> io::Result<()> {
	if !directory.exists() {
		return Ok(());
	}

	for entry in fs::read_dir(directory)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_dir() {
			emit_rerun_files_filtered(&path, include_file)?;
		} else if include_file(&path) {
			println!("cargo:rerun-if-changed={}", path.display());
		}
	}

	Ok(())
}

fn active_profile_directory() -> io::Result<PathBuf> {
	let out_dir = PathBuf::from(env_var("OUT_DIR")?);

	if let Some(profile_dir) = profile_directory_from_out_dir_layout(&out_dir) {
		return Ok(profile_dir);
	}

	let profile_names = known_profile_names();

	for ancestor in out_dir.ancestors() {
		let Some(name) = ancestor.file_name().and_then(|name| name.to_str()) else {
			continue;
		};

		if profile_names.iter().any(|profile| profile == name) {
			return Ok(ancestor.to_path_buf());
		}
	}

	Err(io::Error::new(
		io::ErrorKind::NotFound,
		format!(
			"cannot determine target profile directory from {} using profiles [{}]",
			out_dir.display(),
			profile_names.join(", ")
		),
	))
}

fn workspace_root() -> io::Result<PathBuf> {
	let manifest_dir = PathBuf::from(env_var("CARGO_MANIFEST_DIR")?);

	for ancestor in manifest_dir.ancestors() {
		if ancestor.join("Cargo.toml").is_file()
			&& ancestor.join("NOTICE").is_file()
			&& ancestor.join("ZeroEngine").is_dir()
		{
			return Ok(ancestor.to_path_buf());
		}
	}

	Err(io::Error::new(
		io::ErrorKind::NotFound,
		format!("cannot find workspace root from {}", manifest_dir.display()),
	))
}

fn is_optimized_profile() -> io::Result<bool> { Ok(matches!(env_var("PROFILE")?.as_str(), "release" | "dist")) }

fn replace_directory(path: &Path) -> io::Result<()> {
	match fs::remove_dir_all(path) {
		Ok(()) => {}
		Err(error) if error.kind() == io::ErrorKind::NotFound => {}
		Err(error) => return Err(error),
	}

	fs::create_dir_all(path)
}

fn remove_dir_if_exists(path: &Path) -> io::Result<()> {
	match fs::remove_dir_all(path) {
		Ok(()) => Ok(()),
		Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
		Err(error) => Err(error),
	}
}

fn copy_file(source: &Path, target: &Path) -> io::Result<()> {
	if let Some(parent) = target.parent() {
		fs::create_dir_all(parent)?;
	}

	fs::copy(source, target).map(|_| ())
}

fn env_var(name: &str) -> io::Result<String> {
	env::var(name).map_err(|error| io::Error::new(io::ErrorKind::NotFound, format!("{name} is not set: {error}")))
}

fn known_profile_names() -> Vec<String> {
	let mut names = Vec::new();

	if let Ok(cargo_profile) = env::var("CARGO_PROFILE") {
		names.push(cargo_profile);
	}

	if let Ok(profile) = env::var("PROFILE")
		&& !names.iter().any(|name| name == &profile)
	{
		names.push(profile);
	}

	for profile in ["debug", "release", "dist"] {
		if !names.iter().any(|name| name == profile) {
			names.push(profile.to_string());
		}
	}

	names
}

fn profile_directory_from_out_dir_layout(out_dir: &Path) -> Option<PathBuf> {
	let normal_components = out_dir
		.components()
		.filter_map(|component| match component {
			Component::Normal(part) => part.to_str(),
			_ => None,
		})
		.collect::<Vec<_>>();

	let target_index = normal_components.iter().position(|part| *part == "target")?;
	let build_index = normal_components.iter().position(|part| *part == "build")?;

	if build_index <= target_index + 1 {
		return None;
	}

	let between = &normal_components[(target_index + 1)..build_index];
	let profile_name = between.iter().rev().find(|part| is_profile_component(part))?;

	for ancestor in out_dir.ancestors() {
		let Some(name) = ancestor.file_name().and_then(|name| name.to_str()) else {
			continue;
		};
		if name == *profile_name {
			return Some(ancestor.to_path_buf());
		}
	}

	None
}

fn is_profile_component(component: &str) -> bool {
	!component.contains('-') && component != "build" && component != "deps"
}

fn create_staged_assets_without_scripts(profile_dir: &Path, assets_dir: &Path) -> io::Result<PathBuf> {
	let temp_name = format!(
		"ze-build-assets-{}-{}",
		std::process::id(),
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_or(0, |duration| duration.as_nanos())
	);
	let staged_assets_dir = profile_dir.join(temp_name);
	remove_dir_if_exists(&staged_assets_dir)?;
	fs::create_dir_all(&staged_assets_dir)?;
	copy_directory_excluding_names(assets_dir, &staged_assets_dir, &["scripts"])?;
	Ok(staged_assets_dir)
}

fn copy_directory_excluding_names(source: &Path, target: &Path, excluded_names: &[&str]) -> io::Result<()> {
	for entry in fs::read_dir(source)? {
		let entry = entry?;
		let source_path = entry.path();
		let name = entry.file_name();

		if excluded_names
			.iter()
			.any(|excluded| name == std::ffi::OsStr::new(excluded))
		{
			continue;
		}

		let target_path = target.join(&name);
		if source_path.is_dir() {
			fs::create_dir_all(&target_path)?;
			copy_directory_excluding_names(&source_path, &target_path, excluded_names)?;
		} else {
			copy_file(&source_path, &target_path)?;
		}
	}

	Ok(())
}

struct DistLock {
	path: PathBuf,
}

impl DistLock {
	fn acquire(profile_dir: &Path) -> io::Result<Self> {
		let path = profile_dir.join(".ze-build-dist.lock");
		let started_at = Instant::now();

		loop {
			match fs::create_dir(&path) {
				Ok(()) => return Ok(Self { path }),
				Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
					if started_at.elapsed() > Duration::from_secs(30) {
						return Err(io::Error::new(
							io::ErrorKind::TimedOut,
							format!("timed out waiting for dist lock {}", path.display()),
						));
					}
					thread::sleep(Duration::from_millis(50));
				}
				Err(error) => return Err(error),
			}
		}
	}
}

impl Drop for DistLock {
	fn drop(&mut self) { let _ = fs::remove_dir(&self.path); }
}
