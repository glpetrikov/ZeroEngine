#[cfg(feature = "cpu")]
pub mod cpu;
#[cfg(feature = "gpu")]
pub mod gpu;
#[cfg(feature = "os")]
pub mod os;
#[cfg(feature = "ram")]
pub mod ram;

#[cfg(feature = "cpu")]
pub use cpu::CpuInfo;
#[cfg(feature = "gpu")]
pub use gpu::GpuInfo;
#[cfg(feature = "os")]
pub use os::OsInfo;
#[cfg(feature = "ram")]
pub use ram::RamInfo;

pub struct SystemInfo {
	#[cfg(feature = "cpu")]
	pub cpu: CpuInfo,
	#[cfg(feature = "gpu")]
	pub gpu: Option<GpuInfo>,
	#[cfg(feature = "os")]
	pub os: OsInfo,
	#[cfg(feature = "ram")]
	pub ram: RamInfo,
}

impl SystemInfo {
	/// Collects all system information in one snapshot
	pub fn refresh() -> Self {
		Self {
			#[cfg(feature = "cpu")]
			cpu: CpuInfo::refresh(),
			#[cfg(feature = "gpu")]
			gpu: GpuInfo::refresh(),
			#[cfg(feature = "os")]
			os: OsInfo::refresh(),
			#[cfg(feature = "ram")]
			ram: RamInfo::refresh(),
		}
	}
}
