use battery_crate::{
	Manager, State,
	units::{ratio::percent, time::second},
};

#[derive(Debug, Clone)]
pub struct BatteryInfo {
	vendor: Option<String>,
	model: Option<String>,
	state: BatteryState,
	charge_pct: f32,
	health_pct: f32,
	time_to_full_secs: Option<f32>,
	time_to_empty_secs: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
	Charging,
	Discharging,
	Empty,
	Full,
	Unknown,
}

impl BatteryInfo {
	pub fn vendor(&self) -> Option<&str> { self.vendor.as_deref() }
	pub fn model(&self) -> Option<&str> { self.model.as_deref() }
	pub fn state(&self) -> BatteryState { self.state }
	pub fn charge_pct(&self) -> f32 { self.charge_pct }
	pub fn health_pct(&self) -> f32 { self.health_pct }
	pub fn time_to_full_secs(&self) -> Option<f32> { self.time_to_full_secs }
	pub fn time_to_empty_secs(&self) -> Option<f32> { self.time_to_empty_secs }

	pub fn display_name(&self) -> String {
		match (self.vendor(), self.model()) {
			(Some(vendor), Some(model)) => format!("{vendor} {model}"),
			(None, Some(model)) => model.to_string(),
			(Some(vendor), None) => vendor.to_string(),
			(None, None) => "Battery".to_string(),
		}
	}
}

pub fn refresh_batteries() -> Vec<BatteryInfo> {
	let Ok(manager) = Manager::new() else {
		return Vec::new();
	};

	let Ok(batteries) = manager.batteries() else {
		return Vec::new();
	};

	batteries
		.filter_map(Result::ok)
		.map(|battery| BatteryInfo {
			vendor: battery.vendor().map(str::to_string),
			model: battery.model().map(str::to_string),
			state: convert_state(battery.state()),
			charge_pct: battery.state_of_charge().get::<percent>(),
			health_pct: battery.state_of_health().get::<percent>(),
			time_to_full_secs: battery.time_to_full().map(|time| time.get::<second>()),
			time_to_empty_secs: battery.time_to_empty().map(|time| time.get::<second>()),
		})
		.collect()
}

fn convert_state(state: State) -> BatteryState {
	match state {
		State::Charging => BatteryState::Charging,
		State::Discharging => BatteryState::Discharging,
		State::Empty => BatteryState::Empty,
		State::Full => BatteryState::Full,
		_ => BatteryState::Unknown,
	}
}
