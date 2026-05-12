use gfxinfo::active_gpu;

#[derive(Debug, Clone)]
pub struct GpuInfo {
	name: String,
	vendor: String,
	family: String,
	device_id: u32,
	vram_total_bytes: Option<u64>,
	vram_used_bytes: Option<u64>,
	load: Option<f32>,
	temperature: Option<f32>,
}

impl GpuInfo {
	pub fn refresh() -> Option<Self> {
		let gpu = active_gpu().ok()?;
		let info = gpu.info();

		let vram_total_bytes = non_zero(info.total_vram());
		let vram_used_bytes = non_zero(info.used_vram());
		let load = Some(info.load_pct() as f32);
		let temperature = non_zero(info.temperature() as u64).map(|temperature| temperature as f32 / 1000.0);

		Some(Self {
			name: gpu.model().to_string(),
			vendor: gpu.vendor().to_string(),
			family: gpu.family().to_string(),
			device_id: *gpu.device_id(),
			vram_total_bytes,
			vram_used_bytes,
			load,
			temperature,
		})
	}

	pub fn name(&self) -> &str { &self.name }
	pub fn vendor(&self) -> &str { &self.vendor }
	pub fn family(&self) -> &str { &self.family }
	pub fn device_id(&self) -> u32 { self.device_id }

	pub fn vram_total_bytes(&self) -> Option<u64> { self.vram_total_bytes }
	pub fn vram_total_kb(&self) -> Option<u64> { self.vram_total_bytes.map(|bytes| bytes / 1024) }
	pub fn vram_total_mb(&self) -> Option<u64> { self.vram_total_bytes.map(|bytes| bytes / 1024 / 1024) }
	pub fn vram_total_gb(&self) -> Option<f32> { self.vram_total_bytes.map(bytes_to_gib) }

	pub fn vram_used_bytes(&self) -> Option<u64> { self.vram_used_bytes }
	pub fn vram_used_mb(&self) -> Option<u64> { self.vram_used_bytes.map(|bytes| bytes / 1024 / 1024) }
	pub fn vram_used_gb(&self) -> Option<f32> { self.vram_used_bytes.map(bytes_to_gib) }

	pub fn vram_available_bytes(&self) -> Option<u64> {
		match (self.vram_total_bytes, self.vram_used_bytes) {
			(Some(total), Some(used)) => Some(total.saturating_sub(used)),
			_ => None,
		}
	}

	pub fn vram_available_mb(&self) -> Option<u64> { self.vram_available_bytes().map(|bytes| bytes / 1024 / 1024) }

	pub fn vram_available_gb(&self) -> Option<f32> { self.vram_available_bytes().map(bytes_to_gib) }

	pub fn vram_usage_pct(&self) -> Option<f32> {
		match (self.vram_total_bytes, self.vram_used_bytes) {
			(Some(total), Some(used)) if total > 0 => Some(used as f32 / total as f32 * 100.0),
			_ => None,
		}
	}

	pub fn load(&self) -> Option<f32> { self.load }
	pub fn temperature(&self) -> Option<f32> { self.temperature }
}

fn non_zero(value: u64) -> Option<u64> { if value == 0 { None } else { Some(value) } }

fn bytes_to_gib(bytes: u64) -> f32 { bytes as f32 / 1024.0 / 1024.0 / 1024.0 }
