use std::thread;

use sysinfo::{CpuRefreshKind, RefreshKind, System};

use crate::SystemRefreshOptions;

#[derive(Debug, Clone)]
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
	pub fn refresh() -> Self { Self::refresh_with_options(&SystemRefreshOptions::default()) }

	pub fn refresh_fast() -> Self {
		Self::refresh_with_options(&SystemRefreshOptions {
			cpu_usage: false,
			cpu_temperature: false,
			gpu_runtime: false,
			packages: false,
		})
	}

	pub fn refresh_with_options(options: &SystemRefreshOptions) -> Self {
		if options.cpu_usage {
			Self::refresh_with_usage_and_options(options)
		} else {
			Self::refresh_instant_with_options(options)
		}
	}

	pub fn refresh_instant() -> Self { Self::refresh_instant_with_options(&SystemRefreshOptions::default()) }

	pub fn refresh_with_usage() -> Self { Self::refresh_with_usage_and_options(&SystemRefreshOptions::default()) }

	fn refresh_instant_with_options(options: &SystemRefreshOptions) -> Self {
		let mut sys = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
		sys.refresh_cpu_all();

		Self::from_system(&sys, options)
	}

	fn refresh_with_usage_and_options(options: &SystemRefreshOptions) -> Self {
		let mut sys = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

		sys.refresh_cpu_all();
		thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
		sys.refresh_cpu_all();

		Self::from_system(&sys, options)
	}

	fn from_system(sys: &System, options: &SystemRefreshOptions) -> Self {
		let temperature = if options.cpu_temperature {
			detect_cpu_temperature()
		} else {
			None
		};

		Self {
			name: sys
				.cpus()
				.first()
				.map(|cpu| cpu.brand().to_string())
				.unwrap_or_else(|| "Unknown".to_string()),
			cores: sys.cpus().len(),
			physical_cores: System::physical_core_count(),
			frequency_mhz: sys.cpus().first().map(|cpu| cpu.frequency()),
			arch: std::env::consts::ARCH,
			usage: sys.global_cpu_usage(),
			core_usages: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
			temperature,
		}
	}

	pub fn name(&self) -> &str { &self.name }

	pub fn cores(&self) -> usize { self.cores }

	pub fn logical_cores(&self) -> usize { self.cores }

	pub fn physical_cores(&self) -> Option<usize> { self.physical_cores }

	pub fn frequency_mhz(&self) -> Option<u64> { self.frequency_mhz }

	pub fn frequency_ghz(&self) -> Option<f32> { self.frequency_mhz.map(|frequency| frequency as f32 / 1000.0) }

	pub fn arch(&self) -> &str { self.arch }

	pub fn usage(&self) -> f32 { self.usage }

	pub fn core_usages(&self) -> &[f32] { &self.core_usages }

	pub fn temperature(&self) -> Option<f32> { self.temperature }
}

fn detect_cpu_temperature() -> Option<f32> {
	let mut components = sysinfo::Components::new_with_refreshed_list();
	components.refresh(true);

	components
		.iter()
		.find(|component| component.label().to_lowercase().contains("cpu"))
		.and_then(|component| component.temperature())
}
