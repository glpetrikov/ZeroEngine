use std::path::{Path, PathBuf};

use sysinfo::Disks;

#[derive(Debug, Clone)]
pub struct DiskInfo {
	mount_point: PathBuf,
	file_system: String,
	total_bytes: u64,
	available_bytes: u64,
}

impl DiskInfo {
	pub fn mount_point(&self) -> &Path { &self.mount_point }
	pub fn file_system(&self) -> &str { &self.file_system }
	pub fn total_bytes(&self) -> u64 { self.total_bytes }
	pub fn available_bytes(&self) -> u64 { self.available_bytes }

	pub fn used_bytes(&self) -> u64 { self.total_bytes.saturating_sub(self.available_bytes) }

	pub fn total_gb(&self) -> f64 { bytes_to_gib(self.total_bytes) }

	pub fn used_gb(&self) -> f64 { bytes_to_gib(self.used_bytes()) }

	pub fn available_gb(&self) -> f64 { bytes_to_gib(self.available_bytes) }

	pub fn usage_pct(&self) -> f32 {
		if self.total_bytes == 0 {
			return 0.0;
		}

		self.used_bytes() as f32 / self.total_bytes as f32 * 100.0
	}
}

pub fn refresh_disks() -> Vec<DiskInfo> {
	let disks = Disks::new_with_refreshed_list();

	disks
		.list()
		.iter()
		.map(|disk| DiskInfo {
			mount_point: disk.mount_point().to_path_buf(),
			file_system: disk.file_system().to_string_lossy().to_string(),
			total_bytes: disk.total_space(),
			available_bytes: disk.available_space(),
		})
		.collect()
}

pub fn root_disk(disks: &[DiskInfo]) -> Option<&DiskInfo> {
	disks
		.iter()
		.find(|disk| disk.mount_point() == Path::new("/"))
		.or_else(|| disks.first())
}

fn bytes_to_gib(bytes: u64) -> f64 { bytes as f64 / 1024.0 / 1024.0 / 1024.0 }
