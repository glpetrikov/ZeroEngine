use std::{
	env,
	path::{Path, PathBuf},
	process::Command,
};

fn main() {
	let scripts_dir = workspace_root().join("assets/scripts");
	let api_dir = workspace_root().join("ZeroEngine/api/cs");
	let project_path = scripts_dir.join("Scripts.csproj");

	println!("cargo:rerun-if-changed={}", project_path.display());
	emit_cs_rerun_files(&scripts_dir).expect("failed to watch C# script files");
	emit_cs_rerun_files(&api_dir).expect("failed to watch ZeroEngine C# API files");

	let configuration = match env::var("PROFILE").as_deref() {
		Ok("release" | "dist") => "Release",
		_ => "Debug",
	};

	let status = Command::new("dotnet")
		.arg("build")
		.arg(&project_path)
		.arg("--configuration")
		.arg(configuration)
		.arg("--nologo")
		.status()
		.unwrap_or_else(|error| panic!("failed to run dotnet build for {}: {error}", project_path.display()));

	assert!(status.success(), "dotnet build failed for {}", project_path.display());
}

fn workspace_root() -> PathBuf {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"));

	manifest_dir
		.ancestors()
		.find(|path| path.join("assets/scripts").is_dir() || is_workspace_root(path))
		.unwrap_or_else(|| Path::new("."))
		.to_path_buf()
}

fn is_workspace_root(path: &Path) -> bool { path.join("Cargo.toml").is_file() && path.join("ZeroEngine").is_dir() }

fn emit_cs_rerun_files(directory: &Path) -> std::io::Result<()> {
	for entry in std::fs::read_dir(directory)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_dir() {
			emit_cs_rerun_files(&path)?;
		} else if path.extension().is_some_and(|extension| extension == "cs") {
			println!("cargo:rerun-if-changed={}", path.display());
		}
	}

	Ok(())
}
