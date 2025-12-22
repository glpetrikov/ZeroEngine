/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Input.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.22
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Input
 * =================================================
 */

#pragma once

#include "Core.h"

#include "Types.h"

#include <GLFW/glfw3.h>

namespace Fykor::Input
{
	enum class Keycode : int
	{
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


	enum class MouseButton : int
	{
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

	FYKOR_API constexpr int Keycode2Int(Keycode key) { return static_cast<int>(key); }

	FYKOR_API constexpr int MouseButton2Int(MouseButton button) { return static_cast<int>(button); }

	FYKOR_API constexpr Keycode Int2Keycode(int key) { return static_cast<Keycode>(key); }

	FYKOR_API constexpr MouseButton Int2MouseButton(int button) { return static_cast<MouseButton>(button); }

	// main Input Functions

	// Keys Functions

	FYKOR_API bool IsKeyPressed(Keycode keycode);

	FYKOR_API inline bool IsPressed(Keycode key) { return IsKeyPressed(key); }

	// note: this function just checks "is the key up?" not "is the key up in the current frame?", can be called at any
	// time
	FYKOR_API bool IsKeyUp(Keycode keycode);
	// note: this function just checks "is the key down?" not "is the key down in the current frame?", can be called at
	// any time
	FYKOR_API bool IsKeyDown(Keycode keycode);

	// Mouse Functions

	// can be called at any time
	FYKOR_API bool IsMouseButtonPressed(MouseButton button);

	// can be called at any time
	FYKOR_API inline bool IsPressed(MouseButton button) { return IsMouseButtonPressed(button); }

	// returns the screen position in pixels, can be called at any time
	FYKOR_API Vector2 GetMousePosition();
	// returns the screen position in pixels, can be called at any time
	FYKOR_API float GetMouseX();
	// returns the screen position in pixels, can be called at any time
	FYKOR_API float GetMouseY();

	// Key Defines

	inline constexpr auto FR_MOUSE_BUTTON_1 = ::Fykor::Input::MouseButton::Button1;
	inline constexpr auto FR_MOUSE_BUTTON_2 = ::Fykor::Input::MouseButton::Button2;
	inline constexpr auto FR_MOUSE_BUTTON_3 = ::Fykor::Input::MouseButton::Button3;
	inline constexpr auto FR_MOUSE_BUTTON_4 = ::Fykor::Input::MouseButton::Button4;
	inline constexpr auto FR_MOUSE_BUTTON_5 = ::Fykor::Input::MouseButton::Button5;
	inline constexpr auto FR_MOUSE_BUTTON_6 = ::Fykor::Input::MouseButton::Button6;
	inline constexpr auto FR_MOUSE_BUTTON_7 = ::Fykor::Input::MouseButton::Button7;
	inline constexpr auto FR_MOUSE_BUTTON_8 = ::Fykor::Input::MouseButton::Button8;
	inline constexpr auto FR_MOUSE_BUTTON_LAST = ::Fykor::Input::MouseButton::ButtonLast;
	inline constexpr auto FR_MOUSE_BUTTON_LEFT = ::Fykor::Input::MouseButton::ButtonLeft;
	inline constexpr auto FR_MOUSE_BUTTON_RIGHT = ::Fykor::Input::MouseButton::ButtonRight;
	inline constexpr auto FR_MOUSE_BUTTON_MIDDLE = ::Fykor::Input::MouseButton::ButtonMiddle;

	// Keys
	inline constexpr auto FR_KEY_SPACE = ::Fykor::Input::Keycode::Space;
	inline constexpr auto FR_KEY_APOSTROPHE = ::Fykor::Input::Keycode::Apostrophe;
	inline constexpr auto FR_KEY_COMMA = ::Fykor::Input::Keycode::Comma;
	inline constexpr auto FR_KEY_MINUS = ::Fykor::Input::Keycode::Minus;
	inline constexpr auto FR_KEY_PERIOD = ::Fykor::Input::Keycode::Period;
	inline constexpr auto FR_KEY_SLASH = ::Fykor::Input::Keycode::Slash;
	inline constexpr auto FR_KEY_0 = ::Fykor::Input::Keycode::K0;
	inline constexpr auto FR_KEY_1 = ::Fykor::Input::Keycode::K1;
	inline constexpr auto FR_KEY_2 = ::Fykor::Input::Keycode::K2;
	inline constexpr auto FR_KEY_3 = ::Fykor::Input::Keycode::K3;
	inline constexpr auto FR_KEY_4 = ::Fykor::Input::Keycode::K4;
	inline constexpr auto FR_KEY_5 = ::Fykor::Input::Keycode::K5;
	inline constexpr auto FR_KEY_6 = ::Fykor::Input::Keycode::K6;
	inline constexpr auto FR_KEY_7 = ::Fykor::Input::Keycode::K7;
	inline constexpr auto FR_KEY_8 = ::Fykor::Input::Keycode::K8;
	inline constexpr auto FR_KEY_9 = ::Fykor::Input::Keycode::K9;
	inline constexpr auto FR_KEY_SEMICOLON = ::Fykor::Input::Keycode::Semicolon;
	inline constexpr auto FR_KEY_EQUAL = ::Fykor::Input::Keycode::Equal;
	inline constexpr auto FR_KEY_A = ::Fykor::Input::Keycode::A;
	inline constexpr auto FR_KEY_B = ::Fykor::Input::Keycode::B;
	inline constexpr auto FR_KEY_C = ::Fykor::Input::Keycode::C;
	inline constexpr auto FR_KEY_D = ::Fykor::Input::Keycode::D;
	inline constexpr auto FR_KEY_E = ::Fykor::Input::Keycode::E;
	inline constexpr auto FR_KEY_F = ::Fykor::Input::Keycode::F;
	inline constexpr auto FR_KEY_G = ::Fykor::Input::Keycode::G;
	inline constexpr auto FR_KEY_H = ::Fykor::Input::Keycode::H;
	inline constexpr auto FR_KEY_I = ::Fykor::Input::Keycode::I;
	inline constexpr auto FR_KEY_J = ::Fykor::Input::Keycode::J;
	inline constexpr auto FR_KEY_K = ::Fykor::Input::Keycode::K;
	inline constexpr auto FR_KEY_L = ::Fykor::Input::Keycode::L;
	inline constexpr auto FR_KEY_M = ::Fykor::Input::Keycode::M;
	inline constexpr auto FR_KEY_N = ::Fykor::Input::Keycode::N;
	inline constexpr auto FR_KEY_O = ::Fykor::Input::Keycode::O;
	inline constexpr auto FR_KEY_P = ::Fykor::Input::Keycode::P;
	inline constexpr auto FR_KEY_Q = ::Fykor::Input::Keycode::Q;
	inline constexpr auto FR_KEY_R = ::Fykor::Input::Keycode::R;
	inline constexpr auto FR_KEY_S = ::Fykor::Input::Keycode::S;
	inline constexpr auto FR_KEY_T = ::Fykor::Input::Keycode::T;
	inline constexpr auto FR_KEY_U = ::Fykor::Input::Keycode::U;
	inline constexpr auto FR_KEY_V = ::Fykor::Input::Keycode::V;
	inline constexpr auto FR_KEY_W = ::Fykor::Input::Keycode::W;
	inline constexpr auto FR_KEY_X = ::Fykor::Input::Keycode::X;
	inline constexpr auto FR_KEY_Y = ::Fykor::Input::Keycode::Y;
	inline constexpr auto FR_KEY_Z = ::Fykor::Input::Keycode::Z;
	inline constexpr auto FR_KEY_LEFT_BRACKET = ::Fykor::Input::Keycode::LeftBracket;
	inline constexpr auto FR_KEY_BACKSLASH = ::Fykor::Input::Keycode::Backslash;
	inline constexpr auto FR_KEY_RIGHT_BRACKET = ::Fykor::Input::Keycode::RightBracket;
	inline constexpr auto FR_KEY_GRAVE_ACCENT = ::Fykor::Input::Keycode::GraveAccent;
	inline constexpr auto FR_KEY_WORLD_1 = ::Fykor::Input::Keycode::World1;
	inline constexpr auto FR_KEY_WORLD_2 = ::Fykor::Input::Keycode::World2;
	inline constexpr auto FR_KEY_ESCAPE = ::Fykor::Input::Keycode::Escape;
	inline constexpr auto FR_KEY_ENTER = ::Fykor::Input::Keycode::Enter;
	inline constexpr auto FR_KEY_TAB = ::Fykor::Input::Keycode::Tab;
	inline constexpr auto FR_KEY_BACKSPACE = ::Fykor::Input::Keycode::Backspace;
	inline constexpr auto FR_KEY_INSERT = ::Fykor::Input::Keycode::Insert;
	inline constexpr auto FR_KEY_DELETE = ::Fykor::Input::Keycode::Delete;
	inline constexpr auto FR_KEY_RIGHT = ::Fykor::Input::Keycode::RightArrow;
	inline constexpr auto FR_KEY_LEFT = ::Fykor::Input::Keycode::LeftArrow;
	inline constexpr auto FR_KEY_DOWN = ::Fykor::Input::Keycode::DownArrow;
	inline constexpr auto FR_KEY_UP = ::Fykor::Input::Keycode::UpArrow;
	inline constexpr auto FR_KEY_PAGE_UP = ::Fykor::Input::Keycode::PageUp;
	inline constexpr auto FR_KEY_PAGE_DOWN = ::Fykor::Input::Keycode::PageDown;
	inline constexpr auto FR_KEY_HOME = ::Fykor::Input::Keycode::Home;
	inline constexpr auto FR_KEY_END = ::Fykor::Input::Keycode::End;
	inline constexpr auto FR_KEY_CAPS_LOCK = ::Fykor::Input::Keycode::CapsLock;
	inline constexpr auto FR_KEY_SCROLL_LOCK = ::Fykor::Input::Keycode::ScrollLock;
	inline constexpr auto FR_KEY_NUM_LOCK = ::Fykor::Input::Keycode::NumLock;
	inline constexpr auto FR_KEY_PRINT_SCREEN = ::Fykor::Input::Keycode::PrintScreen;
	inline constexpr auto FR_KEY_PAUSE = ::Fykor::Input::Keycode::Pause;

	// F keys
	inline constexpr auto FR_KEY_F1 = ::Fykor::Input::Keycode::F1;
	inline constexpr auto FR_KEY_F2 = ::Fykor::Input::Keycode::F2;
	inline constexpr auto FR_KEY_F3 = ::Fykor::Input::Keycode::F3;
	inline constexpr auto FR_KEY_F4 = ::Fykor::Input::Keycode::F4;
	inline constexpr auto FR_KEY_F5 = ::Fykor::Input::Keycode::F5;
	inline constexpr auto FR_KEY_F6 = ::Fykor::Input::Keycode::F6;
	inline constexpr auto FR_KEY_F7 = ::Fykor::Input::Keycode::F7;
	inline constexpr auto FR_KEY_F8 = ::Fykor::Input::Keycode::F8;
	inline constexpr auto FR_KEY_F9 = ::Fykor::Input::Keycode::F9;
	inline constexpr auto FR_KEY_F10 = ::Fykor::Input::Keycode::F10;
	inline constexpr auto FR_KEY_F11 = ::Fykor::Input::Keycode::F11;
	inline constexpr auto FR_KEY_F12 = ::Fykor::Input::Keycode::F12;
	inline constexpr auto FR_KEY_F13 = ::Fykor::Input::Keycode::F13;
	inline constexpr auto FR_KEY_F14 = ::Fykor::Input::Keycode::F14;
	inline constexpr auto FR_KEY_F15 = ::Fykor::Input::Keycode::F15;
	inline constexpr auto FR_KEY_F16 = ::Fykor::Input::Keycode::F16;
	inline constexpr auto FR_KEY_F17 = ::Fykor::Input::Keycode::F17;
	inline constexpr auto FR_KEY_F18 = ::Fykor::Input::Keycode::F18;
	inline constexpr auto FR_KEY_F19 = ::Fykor::Input::Keycode::F19;
	inline constexpr auto FR_KEY_F20 = ::Fykor::Input::Keycode::F20;
	inline constexpr auto FR_KEY_F21 = ::Fykor::Input::Keycode::F21;
	inline constexpr auto FR_KEY_F22 = ::Fykor::Input::Keycode::F22;
	inline constexpr auto FR_KEY_F23 = ::Fykor::Input::Keycode::F23;
	inline constexpr auto FR_KEY_F24 = ::Fykor::Input::Keycode::F24;
	inline constexpr auto FR_KEY_F25 = ::Fykor::Input::Keycode::F25;

	// Keypad
	inline constexpr auto FR_KEY_KP_0 = ::Fykor::Input::Keycode::KP0;
	inline constexpr auto FR_KEY_KP_1 = ::Fykor::Input::Keycode::KP1;
	inline constexpr auto FR_KEY_KP_2 = ::Fykor::Input::Keycode::KP2;
	inline constexpr auto FR_KEY_KP_3 = ::Fykor::Input::Keycode::KP3;
	inline constexpr auto FR_KEY_KP_4 = ::Fykor::Input::Keycode::KP4;
	inline constexpr auto FR_KEY_KP_5 = ::Fykor::Input::Keycode::KP5;
	inline constexpr auto FR_KEY_KP_6 = ::Fykor::Input::Keycode::KP6;
	inline constexpr auto FR_KEY_KP_7 = ::Fykor::Input::Keycode::KP7;
	inline constexpr auto FR_KEY_KP_8 = ::Fykor::Input::Keycode::KP8;
	inline constexpr auto FR_KEY_KP_9 = ::Fykor::Input::Keycode::KP9;
	inline constexpr auto FR_KEY_KP_DECIMAL = ::Fykor::Input::Keycode::KPDecimal;
	inline constexpr auto FR_KEY_KP_DIVIDE = ::Fykor::Input::Keycode::KPDivide;
	inline constexpr auto FR_KEY_KP_MULTIPLY = ::Fykor::Input::Keycode::KPMultiply;
	inline constexpr auto FR_KEY_KP_SUBTRACT = ::Fykor::Input::Keycode::KPSubtract;
	inline constexpr auto FR_KEY_KP_ADD = ::Fykor::Input::Keycode::KPAdd;
	inline constexpr auto FR_KEY_KP_ENTER = ::Fykor::Input::Keycode::KPEnter;
	inline constexpr auto FR_KEY_KP_EQUAL = ::Fykor::Input::Keycode::KPEqual;

	// Modifiers
	inline constexpr auto FR_KEY_LEFT_SHIFT = ::Fykor::Input::Keycode::LeftShift;
	inline constexpr auto FR_KEY_LEFT_CONTROL = ::Fykor::Input::Keycode::LeftControl;
	inline constexpr auto FR_KEY_LEFT_ALT = ::Fykor::Input::Keycode::LeftAlt;
	inline constexpr auto FR_KEY_LEFT_SUPER = ::Fykor::Input::Keycode::LeftSuper;
	inline constexpr auto FR_KEY_RIGHT_SHIFT = ::Fykor::Input::Keycode::RightShift;
	inline constexpr auto FR_KEY_RIGHT_CONTROL = ::Fykor::Input::Keycode::RightControl;
	inline constexpr auto FR_KEY_RIGHT_ALT = ::Fykor::Input::Keycode::RightAlt;
	inline constexpr auto FR_KEY_RIGHT_SUPER = ::Fykor::Input::Keycode::RightSuper;
	inline constexpr auto FR_KEY_MENU = ::Fykor::Input::Keycode::Menu;

	inline constexpr auto FR_KEY_LAST = ::Fykor::Input::Keycode::LastKey;
} // namespace Fykor::Input
