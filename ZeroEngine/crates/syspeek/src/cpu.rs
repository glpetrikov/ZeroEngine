use std::thread;

use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct CpuInfo {
	name: String,
	cores: usize,
	physical_cores: Option<usize>,
	frequency_mhz: Option<u64>,
	arch: &'static str,
	usage: f32,
	core_usages: Vec<f32>,
	temperature: Option<f32>,
}

impl CpuInfo {
	pub fn refresh() -> Self {
		let mut sys = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
		// Two refreshes needed for accurate CPU usage
		thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
		sys.refresh_cpu_all();

		let temperature = {
			let mut components = sysinfo::Components::new_with_refreshed_list();
			components.refresh(true);
			components
				.iter()
				.find(|c| c.label().to_lowercase().contains("cpu"))
				.and_then(|c| c.temperature())
		};

		Self {
			name: sys
				.cpus()
				.first()
				.map(|c| c.brand().to_string())
				.unwrap_or_else(|| "Unknown".to_string()),
			cores: sys.cpus().len(),
			physical_cores: System::physical_core_count(),
			frequency_mhz: sys.cpus().first().map(|c| c.frequency()),
			arch: std::env::consts::ARCH,
			usage: sys.global_cpu_usage(),
			core_usages: sys.cpus().iter().map(|c| c.cpu_usage()).collect(),
			temperature,
		}
	}

	/// Returns the CPU brand/model name
	pub fn name(&self) -> &str { &self.name }

	/// Returns the number of logical CPU cores
	pub fn cores(&self) -> usize { self.cores }

	/// Returns the number of physical CPU cores
	pub fn physical_cores(&self) -> Option<usize> { self.physical_cores }

	/// Returns the CPU frequency in MHz
	pub fn frequency_mhz(&self) -> Option<u64> { self.frequency_mhz }

	/// Returns the CPU frequency in GHz
	pub fn frequency_ghz(&self) -> Option<f32> { self.frequency_mhz.map(|f| f as f32 / 1000.0) }

	/// Returns the CPU architecture (x86_64, aarch64, etc.)
	pub fn arch(&self) -> &str { self.arch }

	/// Returns overall CPU usage (0.0 - 100.0)
	pub fn usage(&self) -> f32 { self.usage }

	/// Returns per-core usage as Vec<f32> (0.0 - 100.0)
	pub fn core_usages(&self) -> &[f32] { &self.core_usages }

	/// Returns CPU temperature in Celsius if available
	pub fn temperature(&self) -> Option<f32> { self.temperature }
}
