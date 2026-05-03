#[allow(dead_code)] // TODO: TEMP
pub struct Input {
	current_keys: [bool; 512],
	previous_keys: [bool; 512],

	current_mouse: [bool; 8],
	previous_mouse: [bool; 8],

	pub mouse_pos: (f32, f32),
}
