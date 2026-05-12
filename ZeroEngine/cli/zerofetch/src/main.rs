use std::{env, net::IpAddr};

use owo_colors::OwoColorize;
use serde_json::json;
use syspeek::{BatteryInfo, BatteryState, SystemInfo};

const LOGO: &[&str] = &[
	"███████╗███████╗██████╗  ██████╗ ",
	"╚══███╔╝██╔════╝██╔══██╗██╔═══██╗",
	"  ███╔╝ █████╗  ██████╔╝██║   ██║",
	" ███╔╝  ██╔══╝  ██╔══██╗██║   ██║",
	"███████╗███████╗██║  ██║╚██████╔╝",
	"╚══════╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ",
	"",
	"Zero Fetch",
];
#[derive(Debug, Clone, Copy)]
struct Config {
	verbose: bool,
	show_logo: bool,
	color: bool,
	json: bool,
}

impl Config {
	fn from_args() -> Self {
		let args = env::args().skip(1).collect::<Vec<_>>();

		Self {
			verbose: args.iter().any(|arg| arg == "--verbose" || arg == "-v"),
			show_logo: !args.iter().any(|arg| arg == "--no-logo"),
			color: !args.iter().any(|arg| arg == "--no-color"),
			json: args.iter().any(|arg| arg == "--json"),
		}
	}
}

fn main() {
	if env::args().any(|arg| arg == "--help" || arg == "-h") {
		print_help();
		return;
	}

	let config = Config::from_args();
	let info = SystemInfo::refresh();

	if config.json {
		render_json(&info, config);
		return;
	}

	let entries = build_entries(&info, config);

	if config.show_logo {
		render_with_logo(LOGO, &entries, config);
	} else {
		render_plain(&entries, config);
	}
}

fn build_entries(info: &SystemInfo, config: Config) -> Vec<(String, String)> {
	let mut entries = vec![
		("Header".to_string(), format!("{}@{}", username(), info.os.hostname())),
		("Separator".to_string(), "---------".to_string()),
		("OS:".to_string(), info.os.long_version().to_string()),
		("Host:".to_string(), info.os.hostname().to_string()),
		("Kernel:".to_string(), format!("Linux {}", info.os.kernel_version())),
		("Uptime:".to_string(), info.os.uptime_formatted()),
	];

	push_package_entries(&mut entries, info);
	push_env_entries(&mut entries, info);
	push_cpu_entries(&mut entries, info, config);
	push_gpu_entries(&mut entries, info, config);
	push_memory_entries(&mut entries, info, config);
	push_disk_entries(&mut entries, info);
	push_network_entries(&mut entries, info);
	push_battery_entries(&mut entries, info);
	push_locale_entries(&mut entries, info);

	entries
}

fn username() -> String {
	env::var("USER")
		.or_else(|_| env::var("USERNAME"))
		.unwrap_or_else(|_| "user".to_string())
}

fn push_package_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo) {
	if let Some(packages) = format_packages(info) {
		entries.push(("Packages:".to_string(), packages));
	}
}

fn push_env_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo) {
	if let Some(shell) = info.env.shell() {
		entries.push(("Shell:".to_string(), shell.to_string()));
	}

	if let Some(desktop) = info.env.desktop() {
		entries.push(("DE:".to_string(), desktop.to_string()));
	}

	if let Some(session_type) = info.env.session_type() {
		entries.push(("Session:".to_string(), session_type.to_string()));
	}

	if let Some(terminal) = info.env.terminal() {
		entries.push(("Terminal:".to_string(), terminal.to_string()));
	}
}

fn push_cpu_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo, config: Config) {
	let cpu_name = clean_cpu_name(info.cpu.name());

	if !config.verbose {
		entries.push(("CPU:".to_string(), cpu_name));
		return;
	}

	entries.push(("CPU:".to_string(), cpu_name.clone()));
	entries.push(("CPU Name:".to_string(), cpu_name));

	if let Some(freq) = info.cpu.frequency_ghz() {
		entries.push(("CPU Freq:".to_string(), format!("{freq:.2} GHz")));
	}

	if let Some(physical_cores) = info.cpu.physical_cores() {
		entries.push((
			"CPU Cores:".to_string(),
			format!("{physical_cores} physical / {} logical", info.cpu.cores()),
		));
	} else {
		entries.push(("CPU Cores:".to_string(), format!("{} logical", info.cpu.cores())));
	}

	entries.push(("CPU Arch:".to_string(), info.cpu.arch().to_string()));
	entries.push(("CPU Usage:".to_string(), format!("{:.1}%", info.cpu.usage())));

	if let Some(temp) = info.cpu.temperature() {
		entries.push(("CPU Temp:".to_string(), format!("{temp:.1}°C")));
	}

	for (index, usage) in info.cpu.core_usages().iter().enumerate() {
		entries.push(("CPU Core:".to_string(), format!("#{index}: {usage:.1}%")));
	}
}

fn push_gpu_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo, config: Config) {
	let Some(gpu) = &info.gpu else {
		entries.push(("GPU:".to_string(), "Unknown".to_string()));
		return;
	};

	if !config.verbose {
		entries.push(("GPU:".to_string(), gpu.name().to_string()));
		return;
	}

	entries.push(("GPU:".to_string(), gpu.name().to_string()));
	entries.push(("GPU Name:".to_string(), gpu.name().to_string()));

	if !gpu.vendor().is_empty() {
		entries.push(("GPU Vendor:".to_string(), gpu.vendor().to_string()));
	}

	if !gpu.family().is_empty() {
		entries.push(("GPU Family:".to_string(), gpu.family().to_string()));
	}

	entries.push(("GPU ID:".to_string(), format!("0x{:X}", gpu.device_id())));

	if let Some(total) = gpu.vram_total_gb() {
		entries.push(("VRAM Total:".to_string(), format!("{total:.2} GiB")));
	}

	if let Some(used) = gpu.vram_used_gb() {
		entries.push(("VRAM Used:".to_string(), format!("{used:.2} GiB")));
	}

	if let Some(available) = gpu.vram_available_gb() {
		entries.push(("VRAM Free:".to_string(), format!("{available:.2} GiB")));
	}

	if let Some(pct) = gpu.vram_usage_pct() {
		entries.push(("VRAM Usage:".to_string(), format!("{pct:.1}%")));
	}

	if let Some(load) = gpu.load() {
		entries.push(("GPU Load:".to_string(), format!("{load:.1}%")));
	}

	if let Some(temp) = gpu.temperature() {
		entries.push(("GPU Temp:".to_string(), format!("{temp:.1}°C")));
	}
}

fn push_memory_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo, config: Config) {
	if !config.verbose {
		entries.push((
			"Memory:".to_string(),
			format!(
				"{:.2} GiB / {:.2} GiB ({:.0}%)",
				info.ram.used_gb(),
				info.ram.total_gb(),
				info.ram.used_pct()
			),
		));

		entries.push(("Swap:".to_string(), format_swap(info)));
		return;
	}

	entries.push((
		"Memory:".to_string(),
		format!(
			"{:.2} GiB / {:.2} GiB ({:.1}%)",
			info.ram.used_gb(),
			info.ram.total_gb(),
			info.ram.used_pct()
		),
	));

	entries.push(("Mem Total:".to_string(), format!("{:.2} GiB", info.ram.total_gb())));
	entries.push(("Mem Used:".to_string(), format!("{:.2} GiB", info.ram.used_gb())));
	entries.push(("Mem Avail:".to_string(), format!("{:.2} GiB", info.ram.available_gb())));
	entries.push(("Mem Free:".to_string(), format!("{:.2} GiB", info.ram.free_gb())));
	entries.push(("Mem Used%:".to_string(), format!("{:.1}%", info.ram.used_pct())));
	entries.push(("Mem Avail%:".to_string(), format!("{:.1}%", info.ram.available_pct())));
	entries.push(("Mem Free%:".to_string(), format!("{:.1}%", info.ram.free_pct())));

	if !info.ram.is_swap_enabled() {
		entries.push(("Swap:".to_string(), "None".to_string()));
		return;
	}

	entries.push((
		"Swap:".to_string(),
		format!(
			"{:.2} GiB / {:.2} GiB ({:.1}%)",
			info.ram.swap_used_gb(),
			info.ram.swap_total_gb(),
			info.ram.swap_used_pct()
		),
	));

	entries.push((
		"Swap Total:".to_string(),
		format!("{:.2} GiB", info.ram.swap_total_gb()),
	));
	entries.push(("Swap Used:".to_string(), format!("{:.2} GiB", info.ram.swap_used_gb())));
	entries.push((
		"Swap Avail:".to_string(),
		format!("{:.2} GiB", info.ram.swap_available_gb()),
	));
	entries.push(("Swap Used%:".to_string(), format!("{:.1}%", info.ram.swap_used_pct())));
	entries.push((
		"Swap Avail%:".to_string(),
		format!("{:.1}%", info.ram.swap_available_pct()),
	));
}

fn push_disk_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo) {
	if let Some(disk) = syspeek::disk::root_disk(&info.disks) {
		entries.push((
			"Disk (/):".to_string(),
			format!(
				"{:.2} GiB / {:.2} GiB ({:.0}%) - {}",
				disk.used_gb(),
				disk.total_gb(),
				disk.usage_pct(),
				disk.file_system()
			),
		));
	}
}

fn push_network_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo) {
	if let Some(ip) = info
		.local_ips
		.iter()
		.find(|ip| should_show_ip(ip.interface(), ip.address()))
	{
		entries.push((format!("Local IP ({}):", ip.interface()), ip.address().to_string()));
	}
}

fn push_battery_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo) {
	for battery in &info.batteries {
		let name = clean_battery_name(&battery.display_name());

		entries.push((
			format!("Battery ({name}):"),
			format!(
				"{:.0}% [{}]",
				battery.charge_pct(),
				format_battery_state_for_display(battery)
			),
		));
	}
}

fn clean_battery_name(name: &str) -> String {
	let name = clean_text(name);

	name.strip_prefix("ASUS ")
		.or_else(|| name.strip_prefix("ASUSTeK "))
		.unwrap_or(&name)
		.to_string()
}

fn push_locale_entries(entries: &mut Vec<(String, String)>, info: &SystemInfo) {
	if let Some(locale) = info.env.locale() {
		entries.push(("Locale:".to_string(), locale.to_string()));
	}
}

fn clean_cpu_name(name: &str) -> String {
	let Some((before_at, _)) = name.split_once('@') else {
		return clean_text(name);
	};

	clean_text(before_at)
}

fn format_packages(info: &SystemInfo) -> Option<String> {
	if info.packages.is_empty() {
		return None;
	}

	Some(
		info.packages
			.iter()
			.map(|packages| format!("{} ({})", packages.count(), packages.manager()))
			.collect::<Vec<_>>()
			.join(", "),
	)
}

fn format_swap(info: &SystemInfo) -> String {
	if !info.ram.is_swap_enabled() {
		return "None".to_string();
	}

	format!(
		"{:.2} GiB / {:.2} GiB ({:.0}%)",
		info.ram.swap_used_gb(),
		info.ram.swap_total_gb(),
		info.ram.swap_used_pct()
	)
}

fn format_battery_state_for_display(battery: &BatteryInfo) -> &'static str {
	match battery.state() {
		BatteryState::Charging => "Charging",
		BatteryState::Discharging => "Discharging",
		BatteryState::Empty => "Empty",
		BatteryState::Full => "Full",
		BatteryState::Unknown if battery.charge_pct() >= 95.0 => "AC Connected",
		BatteryState::Unknown => "Unknown",
	}
}

fn should_show_ip(interface: &str, address: &str) -> bool {
	if is_ignored_interface(interface) {
		return false;
	}

	address
		.parse::<IpAddr>()
		.map(|address| address.is_ipv4())
		.unwrap_or(false)
}

fn is_ignored_interface(interface: &str) -> bool {
	interface == "lo"
		|| interface.starts_with("docker")
		|| interface.starts_with("br-")
		|| interface.starts_with("veth")
		|| interface.starts_with("virbr")
}

fn clean_text(text: &str) -> String { text.split_whitespace().collect::<Vec<_>>().join(" ") }

fn render_plain(entries: &[(String, String)], config: Config) {
	for (label, value) in entries {
		print_entry(label, value, config);
	}
}

fn render_with_logo(logo: &[&str], entries: &[(String, String)], config: Config) {
	let height = logo.len().max(entries.len());

	for index in 0..height {
		let logo_line = logo.get(index).copied().unwrap_or("");
		let entry = entries.get(index);

		match entry {
			Some((label, value)) => {
				let info_line = format_entry(label, value, config);

				if config.color {
					println!("{}    {info_line}", format!("{logo_line:<38}").bright_black());
				} else {
					println!("{logo_line:<38}    {info_line}");
				}
			}
			None => {
				if config.color {
					println!("{}", logo_line.bright_black());
				} else {
					println!("{logo_line}");
				}
			}
		}
	}
}

fn format_entry(label: &str, value: &str, config: Config) -> String {
	match label {
		"Header" => {
			if config.color {
				value.bold().to_string()
			} else {
				value.to_string()
			}
		}
		"Separator" => value.to_string(),
		_ => format!("{} {value}", format_label(label, config)),
	}
}

fn print_entry(label: &str, value: &str, config: Config) {
	println!("{}", format_entry(label, value, config));
}

fn format_label(label: &str, config: Config) -> String {
	let label = format!("{label:<14}");

	if config.color {
		label.cyan().bold().to_string()
	} else {
		label
	}
}

fn render_json(info: &SystemInfo, config: Config) {
	let root_disk = syspeek::disk::root_disk(&info.disks);
	let local_ip = info
		.local_ips
		.iter()
		.find(|ip| should_show_ip(ip.interface(), ip.address()));

	let output = json!({
		"name": "Zero Fetch",
		"version": env!("CARGO_PKG_VERSION"),
		"os": {
			"long_version": info.os.long_version(),
			"hostname": info.os.hostname(),
			"kernel": info.os.kernel_version(),
			"uptime": info.os.uptime_formatted(),
		},
		"environment": {
			"shell": info.env.shell(),
			"desktop": info.env.desktop(),
			"session": info.env.session_type(),
			"terminal": info.env.terminal(),
			"locale": info.env.locale(),
		},
		"cpu": {
			"name": clean_cpu_name(info.cpu.name()),
			"logical_cores": info.cpu.cores(),
			"physical_cores": info.cpu.physical_cores(),
			"frequency_ghz": info.cpu.frequency_ghz(),
			"arch": info.cpu.arch(),
			"usage_pct": if config.verbose { Some(info.cpu.usage()) } else { None },
			"temperature_c": if config.verbose { info.cpu.temperature() } else { None },
			"core_usages_pct": if config.verbose { Some(info.cpu.core_usages()) } else { None },
		},
		"gpu": info.gpu.as_ref().map(|gpu| {
			json!({
				"name": gpu.name(),
				"vendor": gpu.vendor(),
				"family": gpu.family(),
				"device_id": gpu.device_id(),
				"vram_total_gib": gpu.vram_total_gb(),
				"vram_used_gib": gpu.vram_used_gb(),
				"vram_available_gib": gpu.vram_available_gb(),
				"vram_usage_pct": gpu.vram_usage_pct(),
				"load_pct": gpu.load(),
				"temperature_c": gpu.temperature(),
			})
		}),
		"memory": {
			"total_gib": info.ram.total_gb(),
			"used_gib": info.ram.used_gb(),
			"available_gib": info.ram.available_gb(),
			"free_gib": info.ram.free_gb(),
			"used_pct": info.ram.used_pct(),
			"available_pct": info.ram.available_pct(),
			"free_pct": info.ram.free_pct(),
		},
		"swap": {
			"enabled": info.ram.is_swap_enabled(),
			"total_gib": info.ram.swap_total_gb(),
			"used_gib": info.ram.swap_used_gb(),
			"available_gib": info.ram.swap_available_gb(),
			"used_pct": info.ram.swap_used_pct(),
			"available_pct": info.ram.swap_available_pct(),
		},
		"disk": root_disk.map(|disk| {
			json!({
				"mount": disk.mount_point().display().to_string(),
				"filesystem": disk.file_system(),
				"used_gib": disk.used_gb(),
				"total_gib": disk.total_gb(),
				"usage_pct": disk.usage_pct(),
			})
		}),
		"network": local_ip.map(|ip| {
			json!({
				"interface": ip.interface(),
				"address": ip.address(),
			})
		}),
		"battery": info.batteries.iter().map(|battery| {
			json!({
				"name": clean_text(&battery.display_name()),
				"charge_pct": battery.charge_pct(),
				"state": format_battery_state_for_display(battery),
			})
		}).collect::<Vec<_>>(),
		"packages": info.packages.iter().map(|packages| {
			json!({
				"manager": packages.manager(),
				"count": packages.count(),
			})
		}).collect::<Vec<_>>(),
	});

	println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_help() {
	println!("Zero Fetch v{}", env!("CARGO_PKG_VERSION"));
	println!();
	println!("Usage:");
	println!("  zerofetch [options]");
	println!();
	println!("Options:");
	println!("  -v, --verbose   Show detailed CPU, GPU, and memory information");
	println!("      --json      Print system information as JSON");
	println!("      --no-logo   Hide the Zero logo");
	println!("      --no-color  Disable colored output");
	println!("  -h, --help      Show this help message");
}
