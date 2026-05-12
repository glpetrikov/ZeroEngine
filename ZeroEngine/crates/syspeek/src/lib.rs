#[cfg(feature = "battery")]
pub mod battery;
#[cfg(feature = "cpu")]
pub mod cpu;
#[cfg(feature = "disk")]
pub mod disk;
#[cfg(feature = "env")]
pub mod env;
#[cfg(feature = "gpu")]
pub mod gpu;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "os")]
pub mod os;
#[cfg(feature = "packages")]
pub mod packages;
#[cfg(feature = "ram")]
pub mod ram;

#[cfg(feature = "battery")]
pub use battery::*;
#[cfg(feature = "cpu")]
pub use cpu::*;
#[cfg(feature = "disk")]
pub use disk::DiskInfo;
#[cfg(feature = "env")]
pub use env::EnvInfo;
#[cfg(feature = "gpu")]
pub use gpu::GpuInfo;
#[cfg(feature = "network")]
pub use network::LocalIpInfo;
#[cfg(feature = "os")]
pub use os::OsInfo;
#[cfg(feature = "packages")]
pub use packages::PackageInfo;
#[cfg(feature = "ram")]
pub use ram::RamInfo;

#[derive(Debug, Clone)]
pub struct SystemRefreshOptions {
	pub cpu_usage: bool,
	pub cpu_temperature: bool,
	pub gpu_runtime: bool,
	pub packages: bool,
}

impl Default for SystemRefreshOptions {
	fn default() -> Self {
		Self {
			cpu_usage: true,
			cpu_temperature: true,
			gpu_runtime: true,
			packages: true,
		}
	}
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
	#[cfg(feature = "cpu")]
	pub cpu: CpuInfo,

	#[cfg(feature = "gpu")]
	pub gpu: Option<GpuInfo>,

	#[cfg(feature = "os")]
	pub os: OsInfo,

	#[cfg(feature = "ram")]
	pub ram: RamInfo,

	#[cfg(feature = "battery")]
	pub batteries: Vec<BatteryInfo>,

	#[cfg(feature = "env")]
	pub env: EnvInfo,

	#[cfg(feature = "disk")]
	pub disks: Vec<DiskInfo>,

	#[cfg(feature = "network")]
	pub local_ips: Vec<LocalIpInfo>,

	#[cfg(feature = "packages")]
	pub packages: Vec<PackageInfo>,
}

impl SystemInfo {
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
		Self {
			#[cfg(feature = "cpu")]
			cpu: CpuInfo::refresh_with_options(options),

			#[cfg(feature = "gpu")]
			gpu: GpuInfo::refresh(),

			#[cfg(feature = "os")]
			os: OsInfo::refresh(),

			#[cfg(feature = "ram")]
			ram: RamInfo::refresh(),

			#[cfg(feature = "battery")]
			batteries: battery::refresh_batteries(),

			#[cfg(feature = "env")]
			env: EnvInfo::refresh(),

			#[cfg(feature = "disk")]
			disks: disk::refresh_disks(),

			#[cfg(feature = "network")]
			local_ips: network::refresh_local_ips(),

			#[cfg(feature = "packages")]
			packages: {
				if options.packages {
					packages::refresh_packages()
				} else {
					Vec::new()
				}
			},
		}
	}
}
