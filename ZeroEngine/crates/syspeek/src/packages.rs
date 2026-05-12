use std::process::Command;

#[derive(Debug, Clone)]
pub struct PackageInfo {
	manager: String,
	count: usize,
}

impl PackageInfo {
	pub fn manager(&self) -> &str { &self.manager }
	pub fn count(&self) -> usize { self.count }
}

pub fn refresh_packages() -> Vec<PackageInfo> {
	let mut packages = Vec::new();

	if let Some(count) = count_command_lines("pacman", &["-Qq"]) {
		packages.push(PackageInfo {
			manager: "pacman".to_string(),
			count,
		});
	}

	if let Some(count) = count_command_lines("flatpak", &["list", "--app"]) {
		packages.push(PackageInfo {
			manager: "flatpak".to_string(),
			count,
		});
	}

	if let Some(count) = count_command_lines("brew", &["list"]) {
		packages.push(PackageInfo {
			manager: "brew".to_string(),
			count,
		});
	}

	if let Some(count) = count_command_lines("scoop", &["list"]) {
		packages.push(PackageInfo {
			manager: "scoop".to_string(),
			count,
		});
	}

	if let Some(count) = count_command_lines("winget", &["list"]) {
		packages.push(PackageInfo {
			manager: "winget".to_string(),
			count: count.saturating_sub(1),
		});
	}

	packages
}

fn count_command_lines(command: &str, args: &[&str]) -> Option<usize> {
	let output = Command::new(command).args(args).output().ok()?;

	if !output.status.success() {
		return None;
	}

	let text = String::from_utf8_lossy(&output.stdout);
	let count = text.lines().filter(|line| !line.trim().is_empty()).count();

	Some(count)
}
