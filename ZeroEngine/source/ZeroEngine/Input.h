/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Input.h
 * ─────────────────────────────────────────────────
 * Input handling
 * =================================================
 */

#pragma once

#include "Platform.h"
#include "Types.h"
#include <GLFW/glfw3.h>

namespace ZeroEngine::Input {
enum class Keycode : int {
	Space = GLFW_KEY_SPACE,
	Apostrophe = GLFW_KEY_APOSTROPHE,
	Comma = GLFW_KEY_COMMA,
	Minus = GLFW_KEY_MINUS,
	Period = GLFW_KEY_PERIOD,
	Slash = GLFW_KEY_SLASH,
	K0 = GLFW_KEY_0,
	K1 = GLFW_KEY_1,
	K2 = GLFW_KEY_2,
	K3 = GLFW_KEY_3,
	K4 = GLFW_KEY_4,
	K5 = GLFW_KEY_5,
	K6 = GLFW_KEY_6,
	K7 = GLFW_KEY_7,
	K8 = GLFW_KEY_8,
	K9 = GLFW_KEY_9,
	Semicolon = GLFW_KEY_SEMICOLON,
	Equal = GLFW_KEY_EQUAL,
	A = GLFW_KEY_A,
	B = GLFW_KEY_B,
	C = GLFW_KEY_C,
	D = GLFW_KEY_D,
	E = GLFW_KEY_E,
	F = GLFW_KEY_F,
	G = GLFW_KEY_G,
	H = GLFW_KEY_H,
	I = GLFW_KEY_I,
	J = GLFW_KEY_J,
	K = GLFW_KEY_K,
	L = GLFW_KEY_L,
	M = GLFW_KEY_M,
	N = GLFW_KEY_N,
	O = GLFW_KEY_O,
	P = GLFW_KEY_P,
	Q = GLFW_KEY_Q,
	R = GLFW_KEY_R,
	S = GLFW_KEY_S,
	T = GLFW_KEY_T,
	U = GLFW_KEY_U,
	V = GLFW_KEY_V,
	W = GLFW_KEY_W,
	X = GLFW_KEY_X,
	Y = GLFW_KEY_Y,
	Z = GLFW_KEY_Z,
	LeftBracket = GLFW_KEY_LEFT_BRACKET,
	Backslash = GLFW_KEY_BACKSLASH,
	RightBracket = GLFW_KEY_RIGHT_BRACKET,
	GraveAccent = GLFW_KEY_GRAVE_ACCENT,
	World1 = GLFW_KEY_WORLD_1,
	World2 = GLFW_KEY_WORLD_2,

	Escape = GLFW_KEY_ESCAPE,
	Enter = GLFW_KEY_ENTER,
	Tab = GLFW_KEY_TAB,
	Backspace = GLFW_KEY_BACKSPACE,
	Insert = GLFW_KEY_INSERT,
	Delete = GLFW_KEY_DELETE,
	RightArrow = GLFW_KEY_RIGHT,
	LeftArrow = GLFW_KEY_LEFT,
	DownArrow = GLFW_KEY_DOWN,
	UpArrow = GLFW_KEY_UP,
	PageUp = GLFW_KEY_PAGE_UP,
	PageDown = GLFW_KEY_PAGE_DOWN,
	Home = GLFW_KEY_HOME,
	End = GLFW_KEY_END,
	CapsLock = GLFW_KEY_CAPS_LOCK,
	ScrollLock = GLFW_KEY_SCROLL_LOCK,
	NumLock = GLFW_KEY_NUM_LOCK,
	PrintScreen = GLFW_KEY_PRINT_SCREEN,
	Pause = GLFW_KEY_PAUSE,

	// F keys
	F1 = GLFW_KEY_F1,
	F2 = GLFW_KEY_F2,
	F3 = GLFW_KEY_F3,
	F4 = GLFW_KEY_F4,
	F5 = GLFW_KEY_F5,
	F6 = GLFW_KEY_F6,
	F7 = GLFW_KEY_F7,
	F8 = GLFW_KEY_F8,
	F9 = GLFW_KEY_F9,
	F10 = GLFW_KEY_F10,
	F11 = GLFW_KEY_F11,
	F12 = GLFW_KEY_F12,
	F13 = GLFW_KEY_F13,
	F14 = GLFW_KEY_F14,
	F15 = GLFW_KEY_F15,
	F16 = GLFW_KEY_F16,
	F17 = GLFW_KEY_F17,
	F18 = GLFW_KEY_F18,
	F19 = GLFW_KEY_F19,
	F20 = GLFW_KEY_F20,
	F21 = GLFW_KEY_F21,
	F22 = GLFW_KEY_F22,
	F23 = GLFW_KEY_F23,
	F24 = GLFW_KEY_F24,
	F25 = GLFW_KEY_F25,

	// Keypad
	KP0 = GLFW_KEY_KP_0,
	KP1 = GLFW_KEY_KP_1,
	KP2 = GLFW_KEY_KP_2,
	KP3 = GLFW_KEY_KP_3,
	KP4 = GLFW_KEY_KP_4,
	KP5 = GLFW_KEY_KP_5,
	KP6 = GLFW_KEY_KP_6,
	KP7 = GLFW_KEY_KP_7,
	KP8 = GLFW_KEY_KP_8,
	KP9 = GLFW_KEY_KP_9,
	KPDecimal = GLFW_KEY_KP_DECIMAL,
	KPDivide = GLFW_KEY_KP_DIVIDE,
	KPMultiply = GLFW_KEY_KP_MULTIPLY,
	KPSubtract = GLFW_KEY_KP_SUBTRACT,
	KPAdd = GLFW_KEY_KP_ADD,
	KPEnter = GLFW_KEY_KP_ENTER,
	KPEqual = GLFW_KEY_KP_EQUAL,

	LeftShift = GLFW_KEY_LEFT_SHIFT,
	LeftControl = GLFW_KEY_LEFT_CONTROL,
	LeftAlt = GLFW_KEY_LEFT_ALT,
	LeftSuper = GLFW_KEY_LEFT_SUPER,
	RightShift = GLFW_KEY_RIGHT_SHIFT,
	RightControl = GLFW_KEY_RIGHT_CONTROL,
	RightAlt = GLFW_KEY_RIGHT_ALT,
	RightSuper = GLFW_KEY_RIGHT_SUPER,
	Menu = GLFW_KEY_MENU,
	LastKey = GLFW_KEY_LAST
};

enum class MouseButton : int {
	ButtonLeft = GLFW_MOUSE_BUTTON_LEFT,
	ButtonRight = GLFW_MOUSE_BUTTON_RIGHT,
	ButtonMiddle = GLFW_MOUSE_BUTTON_MIDDLE,
	Button1 = GLFW_MOUSE_BUTTON_1,
	Button2 = GLFW_MOUSE_BUTTON_2,
	Button3 = GLFW_MOUSE_BUTTON_3,
	Button4 = GLFW_MOUSE_BUTTON_4,
	Button5 = GLFW_MOUSE_BUTTON_5,
	Button6 = GLFW_MOUSE_BUTTON_6,
	Button7 = GLFW_MOUSE_BUTTON_7,
	Button8 = GLFW_MOUSE_BUTTON_8,
	ButtonLast = GLFW_MOUSE_BUTTON_LAST,
};

ZE_API constexpr int Keycode2Int(Keycode key) {
	return static_cast<int>(key);
}

ZE_API constexpr int MouseButton2Int(MouseButton button) {
	return static_cast<int>(button);
}

ZE_API constexpr Keycode Int2Keycode(int key) {
	return static_cast<Keycode>(key);
}

ZE_API constexpr MouseButton Int2MouseButton(int button) {
	return static_cast<MouseButton>(button);
}

// main Input Functions

// Keys Functions

ZE_API bool IsKeyPressed(Keycode keycode);

ZE_API inline bool IsPressed(Keycode key) {
	return IsKeyPressed(key);
}

// note: this function just checks "is the key up?" not "is the key up in the current ZEame?", can be called at any
// time
ZE_API bool IsKeyUp(Keycode keycode);
// note: this function just checks "is the key down?" not "is the key down in the current ZEame?", can be called at
// any time
ZE_API bool IsKeyDown(Keycode keycode);

// Mouse Functions

// can be called at any time
ZE_API bool IsMouseButtonPressed(MouseButton button);

// can be called at any time
ZE_API inline bool IsPressed(MouseButton button) {
	return IsMouseButtonPressed(button);
}

// returns the screen position in pixels, can be called at any time
ZE_API Vector2 GetMousePosition();
// returns the screen position in pixels, can be called at any time
ZE_API float GetMouseX();
// returns the screen position in pixels, can be called at any time
ZE_API float GetMouseY();

// Key Defines

inline constexpr auto ZE_MOUSE_BUTTON_1 = ::ZeroEngine::Input::MouseButton::Button1;
inline constexpr auto ZE_MOUSE_BUTTON_2 = ::ZeroEngine::Input::MouseButton::Button2;
inline constexpr auto ZE_MOUSE_BUTTON_3 = ::ZeroEngine::Input::MouseButton::Button3;
inline constexpr auto ZE_MOUSE_BUTTON_4 = ::ZeroEngine::Input::MouseButton::Button4;
inline constexpr auto ZE_MOUSE_BUTTON_5 = ::ZeroEngine::Input::MouseButton::Button5;
inline constexpr auto ZE_MOUSE_BUTTON_6 = ::ZeroEngine::Input::MouseButton::Button6;
inline constexpr auto ZE_MOUSE_BUTTON_7 = ::ZeroEngine::Input::MouseButton::Button7;
inline constexpr auto ZE_MOUSE_BUTTON_8 = ::ZeroEngine::Input::MouseButton::Button8;
inline constexpr auto ZE_MOUSE_BUTTON_LAST = ::ZeroEngine::Input::MouseButton::ButtonLast;
inline constexpr auto ZE_MOUSE_BUTTON_LEFT = ::ZeroEngine::Input::MouseButton::ButtonLeft;
inline constexpr auto ZE_MOUSE_BUTTON_RIGHT = ::ZeroEngine::Input::MouseButton::ButtonRight;
inline constexpr auto ZE_MOUSE_BUTTON_MIDDLE = ::ZeroEngine::Input::MouseButton::ButtonMiddle;

// Keys
inline constexpr auto ZE_KEY_SPACE = ::ZeroEngine::Input::Keycode::Space;
inline constexpr auto ZE_KEY_APOSTROPHE = ::ZeroEngine::Input::Keycode::Apostrophe;
inline constexpr auto ZE_KEY_COMMA = ::ZeroEngine::Input::Keycode::Comma;
inline constexpr auto ZE_KEY_MINUS = ::ZeroEngine::Input::Keycode::Minus;
inline constexpr auto ZE_KEY_PERIOD = ::ZeroEngine::Input::Keycode::Period;
inline constexpr auto ZE_KEY_SLASH = ::ZeroEngine::Input::Keycode::Slash;
inline constexpr auto ZE_KEY_0 = ::ZeroEngine::Input::Keycode::K0;
inline constexpr auto ZE_KEY_1 = ::ZeroEngine::Input::Keycode::K1;
inline constexpr auto ZE_KEY_2 = ::ZeroEngine::Input::Keycode::K2;
inline constexpr auto ZE_KEY_3 = ::ZeroEngine::Input::Keycode::K3;
inline constexpr auto ZE_KEY_4 = ::ZeroEngine::Input::Keycode::K4;
inline constexpr auto ZE_KEY_5 = ::ZeroEngine::Input::Keycode::K5;
inline constexpr auto ZE_KEY_6 = ::ZeroEngine::Input::Keycode::K6;
inline constexpr auto ZE_KEY_7 = ::ZeroEngine::Input::Keycode::K7;
inline constexpr auto ZE_KEY_8 = ::ZeroEngine::Input::Keycode::K8;
inline constexpr auto ZE_KEY_9 = ::ZeroEngine::Input::Keycode::K9;
inline constexpr auto ZE_KEY_SEMICOLON = ::ZeroEngine::Input::Keycode::Semicolon;
inline constexpr auto ZE_KEY_EQUAL = ::ZeroEngine::Input::Keycode::Equal;
inline constexpr auto ZE_KEY_A = ::ZeroEngine::Input::Keycode::A;
inline constexpr auto ZE_KEY_B = ::ZeroEngine::Input::Keycode::B;
inline constexpr auto ZE_KEY_C = ::ZeroEngine::Input::Keycode::C;
inline constexpr auto ZE_KEY_D = ::ZeroEngine::Input::Keycode::D;
inline constexpr auto ZE_KEY_E = ::ZeroEngine::Input::Keycode::E;
inline constexpr auto ZE_KEY_F = ::ZeroEngine::Input::Keycode::F;
inline constexpr auto ZE_KEY_G = ::ZeroEngine::Input::Keycode::G;
inline constexpr auto ZE_KEY_H = ::ZeroEngine::Input::Keycode::H;
inline constexpr auto ZE_KEY_I = ::ZeroEngine::Input::Keycode::I;
inline constexpr auto ZE_KEY_J = ::ZeroEngine::Input::Keycode::J;
inline constexpr auto ZE_KEY_K = ::ZeroEngine::Input::Keycode::K;
inline constexpr auto ZE_KEY_L = ::ZeroEngine::Input::Keycode::L;
inline constexpr auto ZE_KEY_M = ::ZeroEngine::Input::Keycode::M;
inline constexpr auto ZE_KEY_N = ::ZeroEngine::Input::Keycode::N;
inline constexpr auto ZE_KEY_O = ::ZeroEngine::Input::Keycode::O;
inline constexpr auto ZE_KEY_P = ::ZeroEngine::Input::Keycode::P;
inline constexpr auto ZE_KEY_Q = ::ZeroEngine::Input::Keycode::Q;
inline constexpr auto ZE_KEY_R = ::ZeroEngine::Input::Keycode::R;
inline constexpr auto ZE_KEY_S = ::ZeroEngine::Input::Keycode::S;
inline constexpr auto ZE_KEY_T = ::ZeroEngine::Input::Keycode::T;
inline constexpr auto ZE_KEY_U = ::ZeroEngine::Input::Keycode::U;
inline constexpr auto ZE_KEY_V = ::ZeroEngine::Input::Keycode::V;
inline constexpr auto ZE_KEY_W = ::ZeroEngine::Input::Keycode::W;
inline constexpr auto ZE_KEY_X = ::ZeroEngine::Input::Keycode::X;
inline constexpr auto ZE_KEY_Y = ::ZeroEngine::Input::Keycode::Y;
inline constexpr auto ZE_KEY_Z = ::ZeroEngine::Input::Keycode::Z;
inline constexpr auto ZE_KEY_LEFT_BRACKET = ::ZeroEngine::Input::Keycode::LeftBracket;
inline constexpr auto ZE_KEY_BACKSLASH = ::ZeroEngine::Input::Keycode::Backslash;
inline constexpr auto ZE_KEY_RIGHT_BRACKET = ::ZeroEngine::Input::Keycode::RightBracket;
inline constexpr auto ZE_KEY_GRAVE_ACCENT = ::ZeroEngine::Input::Keycode::GraveAccent;
inline constexpr auto ZE_KEY_WORLD_1 = ::ZeroEngine::Input::Keycode::World1;
inline constexpr auto ZE_KEY_WORLD_2 = ::ZeroEngine::Input::Keycode::World2;
inline constexpr auto ZE_KEY_ESCAPE = ::ZeroEngine::Input::Keycode::Escape;
inline constexpr auto ZE_KEY_ENTER = ::ZeroEngine::Input::Keycode::Enter;
inline constexpr auto ZE_KEY_TAB = ::ZeroEngine::Input::Keycode::Tab;
inline constexpr auto ZE_KEY_BACKSPACE = ::ZeroEngine::Input::Keycode::Backspace;
inline constexpr auto ZE_KEY_INSERT = ::ZeroEngine::Input::Keycode::Insert;
inline constexpr auto ZE_KEY_DELETE = ::ZeroEngine::Input::Keycode::Delete;
inline constexpr auto ZE_KEY_RIGHT = ::ZeroEngine::Input::Keycode::RightArrow;
inline constexpr auto ZE_KEY_LEFT = ::ZeroEngine::Input::Keycode::LeftArrow;
inline constexpr auto ZE_KEY_DOWN = ::ZeroEngine::Input::Keycode::DownArrow;
inline constexpr auto ZE_KEY_UP = ::ZeroEngine::Input::Keycode::UpArrow;
inline constexpr auto ZE_KEY_PAGE_UP = ::ZeroEngine::Input::Keycode::PageUp;
inline constexpr auto ZE_KEY_PAGE_DOWN = ::ZeroEngine::Input::Keycode::PageDown;
inline constexpr auto ZE_KEY_HOME = ::ZeroEngine::Input::Keycode::Home;
inline constexpr auto ZE_KEY_END = ::ZeroEngine::Input::Keycode::End;
inline constexpr auto ZE_KEY_CAPS_LOCK = ::ZeroEngine::Input::Keycode::CapsLock;
inline constexpr auto ZE_KEY_SCROLL_LOCK = ::ZeroEngine::Input::Keycode::ScrollLock;
inline constexpr auto ZE_KEY_NUM_LOCK = ::ZeroEngine::Input::Keycode::NumLock;
inline constexpr auto ZE_KEY_PRINT_SCREEN = ::ZeroEngine::Input::Keycode::PrintScreen;
inline constexpr auto ZE_KEY_PAUSE = ::ZeroEngine::Input::Keycode::Pause;

// F keys
inline constexpr auto ZE_KEY_F1 = ::ZeroEngine::Input::Keycode::F1;
inline constexpr auto ZE_KEY_F2 = ::ZeroEngine::Input::Keycode::F2;
inline constexpr auto ZE_KEY_F3 = ::ZeroEngine::Input::Keycode::F3;
inline constexpr auto ZE_KEY_F4 = ::ZeroEngine::Input::Keycode::F4;
inline constexpr auto ZE_KEY_F5 = ::ZeroEngine::Input::Keycode::F5;
inline constexpr auto ZE_KEY_F6 = ::ZeroEngine::Input::Keycode::F6;
inline constexpr auto ZE_KEY_F7 = ::ZeroEngine::Input::Keycode::F7;
inline constexpr auto ZE_KEY_F8 = ::ZeroEngine::Input::Keycode::F8;
inline constexpr auto ZE_KEY_F9 = ::ZeroEngine::Input::Keycode::F9;
inline constexpr auto ZE_KEY_F10 = ::ZeroEngine::Input::Keycode::F10;
inline constexpr auto ZE_KEY_F11 = ::ZeroEngine::Input::Keycode::F11;
inline constexpr auto ZE_KEY_F12 = ::ZeroEngine::Input::Keycode::F12;
inline constexpr auto ZE_KEY_F13 = ::ZeroEngine::Input::Keycode::F13;
inline constexpr auto ZE_KEY_F14 = ::ZeroEngine::Input::Keycode::F14;
inline constexpr auto ZE_KEY_F15 = ::ZeroEngine::Input::Keycode::F15;
inline constexpr auto ZE_KEY_F16 = ::ZeroEngine::Input::Keycode::F16;
inline constexpr auto ZE_KEY_F17 = ::ZeroEngine::Input::Keycode::F17;
inline constexpr auto ZE_KEY_F18 = ::ZeroEngine::Input::Keycode::F18;
inline constexpr auto ZE_KEY_F19 = ::ZeroEngine::Input::Keycode::F19;
inline constexpr auto ZE_KEY_F20 = ::ZeroEngine::Input::Keycode::F20;
inline constexpr auto ZE_KEY_F21 = ::ZeroEngine::Input::Keycode::F21;
inline constexpr auto ZE_KEY_F22 = ::ZeroEngine::Input::Keycode::F22;
inline constexpr auto ZE_KEY_F23 = ::ZeroEngine::Input::Keycode::F23;
inline constexpr auto ZE_KEY_F24 = ::ZeroEngine::Input::Keycode::F24;
inline constexpr auto ZE_KEY_F25 = ::ZeroEngine::Input::Keycode::F25;

// Keypad
inline constexpr auto ZE_KEY_KP_0 = ::ZeroEngine::Input::Keycode::KP0;
inline constexpr auto ZE_KEY_KP_1 = ::ZeroEngine::Input::Keycode::KP1;
inline constexpr auto ZE_KEY_KP_2 = ::ZeroEngine::Input::Keycode::KP2;
inline constexpr auto ZE_KEY_KP_3 = ::ZeroEngine::Input::Keycode::KP3;
inline constexpr auto ZE_KEY_KP_4 = ::ZeroEngine::Input::Keycode::KP4;
inline constexpr auto ZE_KEY_KP_5 = ::ZeroEngine::Input::Keycode::KP5;
inline constexpr auto ZE_KEY_KP_6 = ::ZeroEngine::Input::Keycode::KP6;
inline constexpr auto ZE_KEY_KP_7 = ::ZeroEngine::Input::Keycode::KP7;
inline constexpr auto ZE_KEY_KP_8 = ::ZeroEngine::Input::Keycode::KP8;
inline constexpr auto ZE_KEY_KP_9 = ::ZeroEngine::Input::Keycode::KP9;
inline constexpr auto ZE_KEY_KP_DECIMAL = ::ZeroEngine::Input::Keycode::KPDecimal;
inline constexpr auto ZE_KEY_KP_DIVIDE = ::ZeroEngine::Input::Keycode::KPDivide;
inline constexpr auto ZE_KEY_KP_MULTIPLY = ::ZeroEngine::Input::Keycode::KPMultiply;
inline constexpr auto ZE_KEY_KP_SUBTRACT = ::ZeroEngine::Input::Keycode::KPSubtract;
inline constexpr auto ZE_KEY_KP_ADD = ::ZeroEngine::Input::Keycode::KPAdd;
inline constexpr auto ZE_KEY_KP_ENTER = ::ZeroEngine::Input::Keycode::KPEnter;
inline constexpr auto ZE_KEY_KP_EQUAL = ::ZeroEngine::Input::Keycode::KPEqual;

// Modifiers
inline constexpr auto ZE_KEY_LEFT_SHIFT = ::ZeroEngine::Input::Keycode::LeftShift;
inline constexpr auto ZE_KEY_LEFT_CONTROL = ::ZeroEngine::Input::Keycode::LeftControl;
inline constexpr auto ZE_KEY_LEFT_ALT = ::ZeroEngine::Input::Keycode::LeftAlt;
inline constexpr auto ZE_KEY_LEFT_SUPER = ::ZeroEngine::Input::Keycode::LeftSuper;
inline constexpr auto ZE_KEY_RIGHT_SHIFT = ::ZeroEngine::Input::Keycode::RightShift;
inline constexpr auto ZE_KEY_RIGHT_CONTROL = ::ZeroEngine::Input::Keycode::RightControl;
inline constexpr auto ZE_KEY_RIGHT_ALT = ::ZeroEngine::Input::Keycode::RightAlt;
inline constexpr auto ZE_KEY_RIGHT_SUPER = ::ZeroEngine::Input::Keycode::RightSuper;
inline constexpr auto ZE_KEY_MENU = ::ZeroEngine::Input::Keycode::Menu;

inline constexpr auto ZE_KEY_LAST = ::ZeroEngine::Input::Keycode::LastKey;
} // namespace ZeroEngine::Input
