/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Input.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.17
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

namespace Fykor
{
	class FYKOR_API Input
	{
	public:
		inline static bool IsKeyPressed(int keycode) { return IsKeyPressedImpl(keycode); }

		inline static bool IsMouseButtonPressed(int button) { return IsMouseButtonPressedImpl(button); }

		inline static Vector2 GetMousePosition() { return GetMousePositionImpl(); }

		inline static float GetMouseX() { return GetMouseX(); }

		inline static float GetMouseY() { return GetMouseY(); }

	private:
		bool static IsKeyPressedImpl(int keycode);
		bool static IsMouseButtonPressedImpl(int button);
		static Vector2 GetMousePositionImpl();
		static float GetMouseXImpl();
		static float GetMouseYImpl();
	};
} // namespace Fykor
