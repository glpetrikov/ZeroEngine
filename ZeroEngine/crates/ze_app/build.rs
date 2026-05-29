fn main() {
	let source_assets_dir = std::path::Path::new("../../../assets");
	println!("cargo:rerun-if-changed={}", source_assets_dir.display());

	let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is not set"));
	let profile = std::env::var("PROFILE").expect("PROFILE is not set");
	let Some(profile_dir) = out_dir.ancestors().find(|path| {
		path.file_name()
			.and_then(|name| name.to_str())
			.is_some_and(|name| name == profile)
	}) else {
		panic!("cannot determine target profile directory from {}", out_dir.display());
	};

	copy_assets(source_assets_dir, &profile_dir.join("assets")).expect("failed to copy assets to target profile dir");
}

fn copy_assets(source: &std::path::Path, target: &std::path::Path) -> std::io::Result<()> {
	std::fs::create_dir_all(target)?;

	for entry in std::fs::read_dir(source)? {
		let entry = entry?;
		let source_path = entry.path();
		let target_path = target.join(entry.file_name());

		if entry.file_name() == ".compiled" {
			continue;
		}

		if source_path.is_dir() {
			copy_assets(&source_path, &target_path)?;
		} else {
			if let Some(parent) = target_path.parent() {
				std::fs::create_dir_all(parent)?;
			}
			std::fs::copy(&source_path, &target_path)?;
		}
	}

	Ok(())
}
