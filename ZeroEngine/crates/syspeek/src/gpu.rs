use gfxinfo::active_gpu;

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

		let vram_total = {
			let v = info.total_vram();
			if v == 0 { None } else { Some(v) }
		};

		let vram_used = {
			let v = info.used_vram();
			if v == 0 { None } else { Some(v) }
		};

		let load: Option<f32> = {
			let v = info.load_pct() as f32;
			if v == 0.0 { None } else { Some(v) }
		};

		let temperature = {
			let v = info.temperature();
			if v == 0 { None } else { Some(v as f32 / 1000.0) }
		};

		Some(Self {
			name: gpu.model().to_string(),
			vendor: gpu.vendor().to_string(),
			family: gpu.family().to_string(),
			device_id: *gpu.device_id(),
			vram_total_bytes: vram_total,
			vram_used_bytes: vram_used,
			load,
			temperature,
		})
	}

	/// Returns the GPU model name
	pub fn name(&self) -> &str { &self.name }

	/// Returns the GPU vendor (NVIDIA, AMD, Intel, etc.)
	pub fn vendor(&self) -> &str { &self.vendor }

	/// Returns the GPU family
	pub fn family(&self) -> &str { &self.family }

	/// Returns the GPU device ID
	pub fn device_id(&self) -> u32 { self.device_id }

	// ── VRAM Total ───────────────────────────────────────────────────────────

	/// Returns total VRAM in bytes
	pub fn vram_total_bytes(&self) -> Option<u64> { self.vram_total_bytes }

	/// Returns total VRAM in KB
	pub fn vram_total_kb(&self) -> Option<u64> { self.vram_total_bytes.map(|v| v / 1024) }

	/// Returns total VRAM in MB
	pub fn vram_total_mb(&self) -> Option<u64> { self.vram_total_bytes.map(|v| v / 1024 / 1024) }

	/// Returns total VRAM in GB
	pub fn vram_total_gb(&self) -> Option<f32> { self.vram_total_bytes.map(|v| v as f32 / 1024.0 / 1024.0 / 1024.0) }

	// ── VRAM Used ────────────────────────────────────────────────────────────

	/// Returns used VRAM in bytes
	pub fn vram_used_bytes(&self) -> Option<u64> { self.vram_used_bytes }

	/// Returns used VRAM in MB
	pub fn vram_used_mb(&self) -> Option<u64> { self.vram_used_bytes.map(|v| v / 1024 / 1024) }

	/// Returns used VRAM in GB
	pub fn vram_used_gb(&self) -> Option<f32> { self.vram_used_bytes.map(|v| v as f32 / 1024.0 / 1024.0 / 1024.0) }

	// ── VRAM Available ───────────────────────────────────────────────────────

	/// Returns available VRAM in bytes
	pub fn vram_available_bytes(&self) -> Option<u64> {
		match (self.vram_total_bytes, self.vram_used_bytes) {
			(Some(total), Some(used)) => Some(total.saturating_sub(used)),
			_ => None,
		}
	}

	/// Returns available VRAM in MB
	pub fn vram_available_mb(&self) -> Option<u64> { self.vram_available_bytes().map(|v| v / 1024 / 1024) }

	// ── VRAM Usage % ─────────────────────────────────────────────────────────

	/// Returns VRAM usage as percentage (0.0 - 100.0)
	pub fn vram_usage_pct(&self) -> Option<f32> {
		match (self.vram_total_bytes, self.vram_used_bytes) {
			(Some(total), Some(used)) if total > 0 => Some(used as f32 / total as f32 * 100.0),
			_ => None,
		}
	}

	// ── Load & Temp ──────────────────────────────────────────────────────────

	/// Returns GPU load percentage (0.0 - 100.0) if available
	pub fn load(&self) -> Option<f32> { self.load }

	/// Returns GPU temperature in Celsius if available
	pub fn temperature(&self) -> Option<f32> { self.temperature }
}
