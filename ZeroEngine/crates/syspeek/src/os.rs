use sysinfo::System;

#[derive(Clone, Debug)]
pub struct OsInfo {
	name: String,
	version: String,
	long_version: String,
	kernel_version: String,
	hostname: String,
	arch: &'static str,
	family: &'static str,
	distro_id: String,
	uptime_secs: u64,
	boot_time: u64,
}

impl OsInfo {
	pub fn refresh() -> Self {
		Self {
			name: System::name().unwrap_or_else(|| "Unknown".to_string()),
			version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
			long_version: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
			kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
			hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
			arch: std::env::consts::ARCH,
			family: std::env::consts::FAMILY,
			distro_id: System::distribution_id(),
			uptime_secs: System::uptime(),
			boot_time: System::boot_time(),
		}
	}

	/// Returns the OS name (e.g. "Linux", "Windows", "macOS")
	pub fn name(&self) -> &str { &self.name }

	/// Returns the OS version (e.g. "24.04", "11")
	pub fn version(&self) -> &str { &self.version }

	/// Returns the full OS version string (e.g. "Ubuntu 24.04 LTS")
	pub fn long_version(&self) -> &str { &self.long_version }

	/// Returns the kernel version
	pub fn kernel_version(&self) -> &str { &self.kernel_version }

	/// Returns the hostname
	pub fn hostname(&self) -> &str { &self.hostname }

	/// Returns the CPU architecture (x86_64, aarch64, etc.)
	pub fn arch(&self) -> &str { self.arch }

	/// Returns the OS family (unix, windows)
	pub fn family(&self) -> &str { self.family }

	/// Returns the distribution ID on Linux (e.g. "arch", "ubuntu")
	pub fn distro_id(&self) -> &str { &self.distro_id }

	/// Returns the number of seconds since boot
	pub fn uptime_secs(&self) -> u64 { self.uptime_secs }

	/// Returns uptime formatted as "Xh Ym Zs"
	pub fn uptime_formatted(&self) -> String {
		let secs = self.uptime_secs;
		let h = secs / 3600;
		let m = (secs % 3600) / 60;
		let s = secs % 60;
		if h > 0 {
			format!("{}h {}m {}s", h, m, s)
		} else if m > 0 {
			format!("{}m {}s", m, s)
		} else {
			format!("{}s", s)
		}
	}

	/// Returns the boot time as Unix timestamp
	pub fn boot_time(&self) -> u64 { self.boot_time }
}
