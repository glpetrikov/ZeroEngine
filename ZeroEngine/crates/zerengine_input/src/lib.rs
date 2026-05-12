#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZKeyCode {
	Escape = 0,
	Space,
	Q,
	W,
	E,
	R,
	T,
	Y,
	U,
	I,
	O,
	P,
	A,
	S,
	D,
	F,
	G,
	H,
	J,
	K,
	L,
	Z,
	X,
	C,
	V,
	B,
	N,
	M,
	Enter,
	Ctrl,
	K1,
	K2,
	K3,
	K4,
	K5,
	K6,
	K7,
	K8,
	K9,
	K0,
	KF1,
	KF2,
	KF3,
	KF4,
	KF5,
	KF6,
	KF7,
	KF8,
	KF9,
	KF10,
	KF11,
	KF12,
	Unknown = 511,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZMouseCode {
	Left = 0,
	Right = 1,
	Middle = 2,
	Back = 3,
	Forward = 4,
	Other = 7,
}

pub struct Input {
	current_keys: [bool; 512],
	previous_keys: [bool; 512],

	current_mouse: [bool; 8],
	previous_mouse: [bool; 8],

	pub mouse_pos: (f32, f32),
}

impl Input {
	pub fn new() -> Self { Self::default() }

	// --- Updaters ---

	pub fn set_key(&mut self, code: ZKeyCode, state: bool) { self.current_keys[code as usize] = state; }

	pub fn set_mouse_button(&mut self, button: ZMouseCode, state: bool) { self.current_mouse[button as usize] = state; }

	pub fn late_update(&mut self) {
		self.previous_keys = self.current_keys;
		self.previous_mouse = self.current_mouse;
	}

	// --- Main Methods ---

	// Keyboard

	fn is_key_pressed(&self, code: ZKeyCode) -> bool { self.current_keys[code as usize] }

	fn is_key_just_pressed(&self, code: ZKeyCode) -> bool {
		self.current_keys[code as usize] && !self.previous_keys[code as usize]
	}

	fn is_key_released(&self, key_code: ZKeyCode) -> bool { !self.current_keys[key_code as usize] }

	fn is_key_just_released(&self, key_code: ZKeyCode) -> bool {
		!self.is_key_pressed(key_code) && self.previous_keys[key_code as usize]
	}

	// Mouse

	fn is_button_pressed(&self, code: ZMouseCode) -> bool { self.current_mouse[code as usize] }

	fn is_button_just_pressed(&self, code: ZMouseCode) -> bool {
		self.current_mouse[code as usize] && !self.previous_mouse[code as usize]
	}

	fn is_button_released(&self, mouse_code: ZMouseCode) -> bool { !self.current_mouse[mouse_code as usize] }

	fn is_button_just_released(&self, mouse_code: ZMouseCode) -> bool {
		!self.is_button_pressed(mouse_code) && self.previous_mouse[mouse_code as usize]
	}
}

// --- Global Access ---

impl Input {
	// Keyboard

	pub fn key_pressed(key: ZKeyCode) -> bool { Self::global().lock().unwrap().is_key_pressed(key) }

	pub fn key_just_pressed(key: ZKeyCode) -> bool { Self::global().lock().unwrap().is_key_just_pressed(key) }

	pub fn key_released(key: ZKeyCode) -> bool { Self::global().lock().unwrap().is_key_released(key) }

	pub fn key_just_released(key: ZKeyCode) -> bool { Self::global().lock().unwrap().is_key_just_released(key) }

	// Mouse

	pub fn get_mouse_pos() -> (f32, f32) { Self::global().lock().unwrap().mouse_pos }

	pub fn mouse_pressed(button: ZMouseCode) -> bool { Self::global().lock().unwrap().is_button_pressed(button) }

	pub fn mouse_just_pressed(button: ZMouseCode) -> bool {
		Self::global().lock().unwrap().is_button_just_pressed(button)
	}

	pub fn mouse_released(button: ZMouseCode) -> bool { Self::global().lock().unwrap().is_button_released(button) }

	pub fn mouse_just_released(button: ZMouseCode) -> bool {
		Self::global().lock().unwrap().is_button_just_released(button)
	}
}

impl Default for Input {
	fn default() -> Self {
		Self {
			current_keys: [false; 512],
			previous_keys: [false; 512],
			current_mouse: [false; 8],
			previous_mouse: [false; 8],
			mouse_pos: (0.0, 0.0),
		}
	}
}

use std::sync::{Mutex, OnceLock};

static INPUT_INSTANCE: OnceLock<Mutex<Input>> = OnceLock::new();

impl Input {
	pub fn global() -> &'static Mutex<Input> { INPUT_INSTANCE.get_or_init(|| Mutex::new(Input::new())) }

	pub fn update_globally<F>(f: F)
	where
		F: FnOnce(&mut Input),
	{
		let mut input = Self::global().lock().unwrap();
		f(&mut input);
	}
}

#[cfg(feature = "winit")]
macro_rules! impl_from_winit_keycode {
    ($($winit:ident => $zero:ident),* $(,)?) => {
        #[cfg(feature = "winit")]
        impl From<winit::keyboard::KeyCode> for ZKeyCode {
            fn from(key: winit::keyboard::KeyCode) -> Self {
                match key {
                    $(winit::keyboard::KeyCode::$winit => ZKeyCode::$zero,)*
                    _ => ZKeyCode::Unknown,
                }
            }
        }
    };
}
#[cfg(feature = "winit")]
impl_from_winit_keycode! {
	Escape => Escape,
	Space => Space,
	KeyQ => Q,
	KeyW => W,
	KeyE => E,
	KeyR => R,
	KeyT => T,
	KeyY => Y,
	KeyU => U,
	KeyI => I,
	KeyO => O,
	KeyP => P,
	KeyA => A,
	KeyS => S,
	KeyD => D,
	KeyF => F,
	KeyG => G,
	KeyH => H,
	KeyJ => J,
	KeyK => K,
	KeyL => L,
	KeyZ => Z,
	KeyX => X,
	KeyC => C,
	KeyV => V,
	KeyB => B,
	KeyN => N,
	KeyM => M,
	Enter => Enter,
	ControlLeft => Ctrl,
	Digit1 => K1,
	Digit2 => K2,
	Digit3 => K3,
	Digit4 => K4,
	Digit5 => K5,
	Digit6 => K6,
	Digit7 => K7,
	Digit8 => K8,
	Digit9 => K9,
	Digit0 => K0,
	F1 => KF1,
	F2 => KF2,
	F3 => KF3,
	F4 => KF4,
	F5 => KF5,
	F6 => KF6,
	F7 => KF7,
	F8 => KF8,
	F9 => KF9,
	F10 => KF10,
	F11 => KF11,
	F12 => KF12,
}

#[cfg(feature = "winit")]
impl From<winit::event::MouseButton> for ZMouseCode {
	fn from(button: winit::event::MouseButton) -> Self {
		match button {
			winit::event::MouseButton::Left => ZMouseCode::Left,
			winit::event::MouseButton::Right => ZMouseCode::Right,
			winit::event::MouseButton::Middle => ZMouseCode::Middle,
			winit::event::MouseButton::Back => ZMouseCode::Back,
			winit::event::MouseButton::Forward => ZMouseCode::Forward,
			winit::event::MouseButton::Other(_) => ZMouseCode::Other,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn make_input() -> Input { Input::new() }

	#[test]
	fn test_key_just_pressed() {
		let mut input = make_input();
		input.set_key(ZKeyCode::Space, true);
		assert!(input.is_key_just_pressed(ZKeyCode::Space));
		assert!(input.is_key_pressed(ZKeyCode::Space));
	}

	#[test]
	fn test_key_just_released() {
		let mut input = make_input();
		input.set_key(ZKeyCode::Space, true);
		input.late_update();
		input.set_key(ZKeyCode::Space, false);
		assert!(input.is_key_just_released(ZKeyCode::Space));
		assert!(!input.is_key_pressed(ZKeyCode::Space));
	}

	#[test]
	fn test_key_held() {
		let mut input = make_input();
		input.set_key(ZKeyCode::W, true);
		input.late_update();
		// Held - pressed but not just pressed
		assert!(input.is_key_pressed(ZKeyCode::W));
		assert!(!input.is_key_just_pressed(ZKeyCode::W));
	}

	#[test]
	fn test_mouse_just_pressed() {
		let mut input = make_input();
		input.set_mouse_button(ZMouseCode::Left, true);
		assert!(input.is_button_just_pressed(ZMouseCode::Left));
	}

	#[test]
	fn test_late_update_clears_just_pressed() {
		let mut input = make_input();
		input.set_key(ZKeyCode::Enter, true);
		input.late_update();
		assert!(!input.is_key_just_pressed(ZKeyCode::Enter));
	}
}
