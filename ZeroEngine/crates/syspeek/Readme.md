# syspeek

A fast and simple system information library for Rust.

`syspeek` provides a clean API for collecting CPU, GPU, RAM, swap, OS, disk, battery, network, package, and environment information.

It is used by [`zerofetch`](../cli/zerofetch), but can also be used as a standalone library in CLIs, launchers, dashboards, monitoring tools, game engines, and developer utilities.

## Features

- CPU information
  - brand/model
  - logical cores
  - physical cores
  - frequency
  - architecture
  - usage percentage
  - per-core usage
  - temperature when available

- GPU information
  - model name
  - vendor
  - family
  - device ID
  - VRAM total/used/free
  - VRAM usage percentage
  - GPU load
  - temperature when available

- RAM and swap information
  - total memory
  - used memory
  - available memory
  - free memory
  - usage percentages
  - swap total/used/free
  - swap usage percentages

- OS information
  - OS name/version
  - kernel version
  - hostname
  - uptime

- Disk information
  - mount point
  - filesystem
  - total/used/available space
  - usage percentage

- Battery information
  - vendor/model
  - charge percentage
  - state
  - health percentage
  - time to full/empty when available

- Environment information
  - shell
  - desktop environment
  - session type
  - terminal
  - locale

- Network information
  - local IP addresses
  - network interface names

- Package information
  - supported package manager counts where available

## Installation

Add `syspeek` to your `Cargo.toml`:

```toml
[dependencies]
syspeek = "1.3.0"
````

When developing inside the ZeroEngine workspace, prefer using the local path dependency:

```toml
[dependencies]
syspeek = { path = "../../crates/syspeek" }
```

## Quick start

```rust
use syspeek::SystemInfo;

fn main() {
	let info = SystemInfo::refresh();

	println!("OS: {}", info.os.long_version());
	println!("Kernel: {}", info.os.kernel_version());
	println!("Host: {}", info.os.hostname());
	println!("Uptime: {}", info.os.uptime_formatted());

	println!("CPU: {}", info.cpu.name());
	println!("CPU cores: {}", info.cpu.cores());

	if let Some(gpu) = &info.gpu {
		println!("GPU: {}", gpu.name());
	}

	println!(
		"Memory: {:.2} GiB / {:.2} GiB ({:.0}%)",
		info.ram.used_gb(),
		info.ram.total_gb(),
		info.ram.used_pct()
	);
}
```

## CPU

```rust
use syspeek::CpuInfo;

fn main() {
	let cpu = CpuInfo::refresh();

	println!("Name: {}", cpu.name());
	println!("Logical cores: {}", cpu.cores());

	if let Some(physical_cores) = cpu.physical_cores() {
		println!("Physical cores: {physical_cores}");
	}

	if let Some(freq) = cpu.frequency_ghz() {
		println!("Frequency: {freq:.2} GHz");
	}

	println!("Architecture: {}", cpu.arch());
	println!("Usage: {:.1}%", cpu.usage());

	for (index, usage) in cpu.core_usages().iter().enumerate() {
		println!("Core #{index}: {usage:.1}%");
	}

	if let Some(temp) = cpu.temperature() {
		println!("Temperature: {temp:.1}°C");
	}
}
```

## RAM and swap

```rust
use syspeek::RamInfo;

fn main() {
	let ram = RamInfo::refresh();

	println!(
		"Memory: {:.2} GiB / {:.2} GiB ({:.1}%)",
		ram.used_gb(),
		ram.total_gb(),
		ram.used_pct()
	);

	println!("Available: {:.2} GiB", ram.available_gb());
	println!("Free: {:.2} GiB", ram.free_gb());

	if ram.is_swap_enabled() {
		println!(
			"Swap: {:.2} GiB / {:.2} GiB ({:.1}%)",
			ram.swap_used_gb(),
			ram.swap_total_gb(),
			ram.swap_used_pct()
		);

		println!("Swap available: {:.2} GiB", ram.swap_available_gb());
	} else {
		println!("Swap: disabled");
	}
}
```

## GPU

```rust
use syspeek::GpuInfo;

fn main() {
	let Some(gpu) = GpuInfo::refresh() else {
		println!("GPU: unknown");
		return;
	};

	println!("Name: {}", gpu.name());
	println!("Vendor: {}", gpu.vendor());
	println!("Family: {}", gpu.family());
	println!("Device ID: 0x{:X}", gpu.device_id());

	if let Some(total) = gpu.vram_total_gb() {
		println!("VRAM total: {total:.2} GiB");
	}

	if let Some(used) = gpu.vram_used_gb() {
		println!("VRAM used: {used:.2} GiB");
	}

	if let Some(available) = gpu.vram_available_gb() {
		println!("VRAM available: {available:.2} GiB");
	}

	if let Some(usage) = gpu.vram_usage_pct() {
		println!("VRAM usage: {usage:.1}%");
	}

	if let Some(load) = gpu.load() {
		println!("Load: {load:.1}%");
	}

	if let Some(temp) = gpu.temperature() {
		println!("Temperature: {temp:.1}°C");
	}
}
```

## Battery

```rust
use syspeek::SystemInfo;

fn main() {
	let info = SystemInfo::refresh();

	for battery in &info.batteries {
		println!("Battery: {}", battery.display_name());
		println!("Charge: {:.0}%", battery.charge_pct());
		println!("State: {:?}", battery.state());
		println!("Health: {:.0}%", battery.health_pct());
	}
}
```

## Disk

```rust
use syspeek::{SystemInfo, disk};

fn main() {
	let info = SystemInfo::refresh();

	if let Some(root) = disk::root_disk(&info.disks) {
		println!("Mount: {}", root.mount_point().display());
		println!("Filesystem: {}", root.file_system());
		println!(
			"Disk: {:.2} GiB / {:.2} GiB ({:.0}%)",
			root.used_gb(),
			root.total_gb(),
			root.usage_pct()
		);
	}
}
```

## Environment

```rust
use syspeek::EnvInfo;

fn main() {
	let env = EnvInfo::refresh();

	if let Some(shell) = env.shell() {
		println!("Shell: {shell}");
	}

	if let Some(desktop) = env.desktop() {
		println!("Desktop: {desktop}");
	}

	if let Some(session) = env.session_type() {
		println!("Session: {session}");
	}

	if let Some(terminal) = env.terminal() {
		println!("Terminal: {terminal}");
	}

	if let Some(locale) = env.locale() {
		println!("Locale: {locale}");
	}
}
```

## Network

```rust
use syspeek::network;

fn main() {
	let ips = network::refresh_local_ips();

	for ip in ips {
		println!("{}: {}", ip.interface(), ip.address());
	}
}
```

## Packages

```rust
use syspeek::packages;

fn main() {
	let packages = packages::refresh_packages();

	for package_info in packages {
		println!("{}: {}", package_info.manager(), package_info.count());
	}
}
```

## Refresh options

`SystemInfo::refresh()` collects all enabled information using default settings.

For a faster refresh, use:

```rust
use syspeek::SystemInfo;

fn main() {
	let info = SystemInfo::refresh_fast();

	println!("OS: {}", info.os.long_version());
}
```

For more control, use `SystemRefreshOptions`:

```rust
use syspeek::{SystemInfo, SystemRefreshOptions};

fn main() {
	let info = SystemInfo::refresh_with_options(&SystemRefreshOptions {
		cpu_usage: true,
		cpu_temperature: false,
		gpu_runtime: true,
		packages: false,
	});

	println!("CPU: {}", info.cpu.name());
}
```

## Feature flags

By default, `syspeek` enables all main modules:

```toml
[features]
default = [
	"gpu",
	"cpu",
	"ram",
	"os",
	"battery",
	"disk",
	"env",
	"network",
	"packages",
]
```

Available features:

| Feature    | Description                                               |
| ---------- | --------------------------------------------------------- |
| `cpu`      | CPU information                                           |
| `gpu`      | GPU information                                           |
| `ram`      | RAM and swap information                                  |
| `os`       | OS, kernel, hostname, and uptime information              |
| `battery`  | Battery information                                       |
| `disk`     | Disk and filesystem information                           |
| `env`      | Shell, desktop, session, terminal, and locale information |
| `network`  | Local IP address information                              |
| `packages` | Package manager package counts                            |

Example with only CPU, RAM, and OS:

```toml
[dependencies]
syspeek = { version = "1.3.0", default-features = false, features = ["cpu", "ram", "os"] }
```

## Notes

Some information may be unavailable depending on the platform, permissions, drivers, or hardware.

For example:

* GPU load and VRAM usage depend on backend and driver support.
* CPU temperature depends on whether sensors are exposed by the OS.
* Battery information is unavailable on most desktop PCs.
* Package counts depend on installed package managers.
* Environment information depends on environment variables.

`syspeek` returns optional values where data may not be available.

## Used by

* [`zerofetch`](../cli/zerofetch) - a fast Zero-style system fetch CLI
* [`ZeroEngine`](../../..) - a modern game engine written in Rust.

## License

Licensed under the Apache 2.0 License.
