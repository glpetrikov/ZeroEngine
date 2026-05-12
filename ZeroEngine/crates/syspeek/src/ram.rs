use sysinfo::{MemoryRefreshKind, RefreshKind, System};

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

	// ── Total ────────────────────────────────────────────────────────────────

	/// Returns total RAM in bytes
	pub fn total_bytes(&self) -> u64 { self.total_bytes }

	/// Returns total RAM in KB
	pub fn total_kb(&self) -> u64 { self.total_bytes / 1024 }

	/// Returns total RAM in MB
	pub fn total_mb(&self) -> u64 { self.total_bytes / 1024 / 1024 }

	/// Returns total RAM in GB
	pub fn total_gb(&self) -> f64 { self.total_bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

	// ── Used ─────────────────────────────────────────────────────────────────

	/// Returns used RAM in bytes
	pub fn used_bytes(&self) -> u64 { self.used_bytes }

	/// Returns used RAM in KB
	pub fn used_kb(&self) -> u64 { self.used_bytes / 1024 }

	/// Returns used RAM in MB
	pub fn used_mb(&self) -> u64 { self.used_bytes / 1024 / 1024 }

	/// Returns used RAM in GB
	pub fn used_gb(&self) -> f64 { self.used_bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

	// ── Available ─────────────────────────────────────────────────────────────

	/// Returns available RAM in bytes
	pub fn available_bytes(&self) -> u64 { self.available_bytes }

	/// Returns available RAM in KB
	pub fn available_kb(&self) -> u64 { self.available_bytes / 1024 }

	/// Returns available RAM in MB
	pub fn available_mb(&self) -> u64 { self.available_bytes / 1024 / 1024 }

	/// Returns available RAM in GB
	pub fn available_gb(&self) -> f64 { self.available_bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

	// ── Free ──────────────────────────────────────────────────────────────────

	/// Returns free RAM in bytes
	pub fn free_bytes(&self) -> u64 { self.free_bytes }

	/// Returns free RAM in MB
	pub fn free_mb(&self) -> u64 { self.free_bytes / 1024 / 1024 }

	/// Returns free RAM in GB
	pub fn free_gb(&self) -> f64 { self.free_bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

	// ── Swap ──────────────────────────────────────────────────────────────────

	/// Returns total swap in bytes
	pub fn swap_total_bytes(&self) -> u64 { self.swap_total_bytes }

	/// Returns total swap in MB
	pub fn swap_total_mb(&self) -> u64 { self.swap_total_bytes / 1024 / 1024 }

	/// Returns total swap in GB
	pub fn swap_total_gb(&self) -> f64 { self.swap_total_bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

	/// Returns used swap in bytes
	pub fn swap_used_bytes(&self) -> u64 { self.swap_used_bytes }

	/// Returns used swap in MB
	pub fn swap_used_mb(&self) -> u64 { self.swap_used_bytes / 1024 / 1024 }

	/// Returns used swap in GB
	pub fn swap_used_gb(&self) -> f64 { self.swap_used_bytes as f64 / 1024.0 / 1024.0 / 1024.0 }

	// ── Usage % ───────────────────────────────────────────────────────────────

	/// Returns RAM usage as percentage (0.0 - 100.0)
	pub fn usage_pct(&self) -> f32 {
		if self.total_bytes == 0 {
			return 0.0;
		}
		self.used_bytes as f32 / self.total_bytes as f32 * 100.0
	}

	/// Returns swap usage as percentage (0.0 - 100.0)
	pub fn swap_usage_pct(&self) -> f32 {
		if self.swap_total_bytes == 0 {
			return 0.0;
		}
		self.swap_used_bytes as f32 / self.swap_total_bytes as f32 * 100.0
	}
}
