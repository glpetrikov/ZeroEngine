use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Debug, Clone)]
pub struct RamInfo {
	total_bytes: u64,
	used_bytes: u64,
	available_bytes: u64,
	free_bytes: u64,
	swap_total_bytes: u64,
	swap_used_bytes: u64,
}

impl RamInfo {
	pub fn refresh() -> Self {
		let mut sys = System::new_with_specifics(RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()));
		sys.refresh_memory();

		Self {
			total_bytes: sys.total_memory(),
			used_bytes: sys.used_memory(),
			available_bytes: sys.available_memory(),
			free_bytes: sys.free_memory(),
			swap_total_bytes: sys.total_swap(),
			swap_used_bytes: sys.used_swap(),
		}
	}

	pub fn total_bytes(&self) -> u64 { self.total_bytes }
	pub fn total_kb(&self) -> u64 { bytes_to_kib(self.total_bytes) }
	pub fn total_mb(&self) -> u64 { bytes_to_mib(self.total_bytes) }
	pub fn total_gb(&self) -> f64 { bytes_to_gib(self.total_bytes) }

	pub fn used_bytes(&self) -> u64 { self.used_bytes }
	pub fn used_kb(&self) -> u64 { bytes_to_kib(self.used_bytes) }
	pub fn used_mb(&self) -> u64 { bytes_to_mib(self.used_bytes) }
	pub fn used_gb(&self) -> f64 { bytes_to_gib(self.used_bytes) }

	pub fn available_bytes(&self) -> u64 { self.available_bytes }
	pub fn available_kb(&self) -> u64 { bytes_to_kib(self.available_bytes) }
	pub fn available_mb(&self) -> u64 { bytes_to_mib(self.available_bytes) }
	pub fn available_gb(&self) -> f64 { bytes_to_gib(self.available_bytes) }

	pub fn free_bytes(&self) -> u64 { self.free_bytes }
	pub fn free_kb(&self) -> u64 { bytes_to_kib(self.free_bytes) }
	pub fn free_mb(&self) -> u64 { bytes_to_mib(self.free_bytes) }
	pub fn free_gb(&self) -> f64 { bytes_to_gib(self.free_bytes) }

	pub fn remaining_bytes(&self) -> u64 { self.available_bytes }
	pub fn remaining_kb(&self) -> u64 { self.available_kb() }
	pub fn remaining_mb(&self) -> u64 { self.available_mb() }
	pub fn remaining_gb(&self) -> f64 { self.available_gb() }

	pub fn swap_total_bytes(&self) -> u64 { self.swap_total_bytes }
	pub fn swap_total_kb(&self) -> u64 { bytes_to_kib(self.swap_total_bytes) }
	pub fn swap_total_mb(&self) -> u64 { bytes_to_mib(self.swap_total_bytes) }
	pub fn swap_total_gb(&self) -> f64 { bytes_to_gib(self.swap_total_bytes) }

	pub fn swap_used_bytes(&self) -> u64 { self.swap_used_bytes }
	pub fn swap_used_kb(&self) -> u64 { bytes_to_kib(self.swap_used_bytes) }
	pub fn swap_used_mb(&self) -> u64 { bytes_to_mib(self.swap_used_bytes) }
	pub fn swap_used_gb(&self) -> f64 { bytes_to_gib(self.swap_used_bytes) }

	pub fn swap_available_bytes(&self) -> u64 { self.swap_total_bytes.saturating_sub(self.swap_used_bytes) }
	pub fn swap_available_kb(&self) -> u64 { bytes_to_kib(self.swap_available_bytes()) }
	pub fn swap_available_mb(&self) -> u64 { bytes_to_mib(self.swap_available_bytes()) }
	pub fn swap_available_gb(&self) -> f64 { bytes_to_gib(self.swap_available_bytes()) }

	pub fn usage_pct(&self) -> f32 { usage_pct(self.used_bytes, self.total_bytes) }
	pub fn used_pct(&self) -> f32 { self.usage_pct() }
	pub fn available_pct(&self) -> f32 { usage_pct(self.available_bytes, self.total_bytes) }
	pub fn free_pct(&self) -> f32 { usage_pct(self.free_bytes, self.total_bytes) }
	pub fn remaining_pct(&self) -> f32 { self.available_pct() }

	pub fn swap_usage_pct(&self) -> f32 { usage_pct(self.swap_used_bytes, self.swap_total_bytes) }
	pub fn swap_used_pct(&self) -> f32 { self.swap_usage_pct() }
	pub fn swap_available_pct(&self) -> f32 { usage_pct(self.swap_available_bytes(), self.swap_total_bytes) }

	pub fn is_swap_enabled(&self) -> bool { self.swap_total_bytes > 0 }
	pub fn is_empty(&self) -> bool { self.total_bytes == 0 }
}

fn bytes_to_kib(bytes: u64) -> u64 { bytes / 1024 }
fn bytes_to_mib(bytes: u64) -> u64 { bytes / 1024 / 1024 }
fn bytes_to_gib(bytes: u64) -> f64 { bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

fn usage_pct(used: u64, total: u64) -> f32 {
	if total == 0 {
		return 0.0;
	}

	used as f32 / total as f32 * 100.0
}
