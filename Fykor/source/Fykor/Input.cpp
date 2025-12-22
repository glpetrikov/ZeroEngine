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

#include "Input.h"
#include "Common.h"

#include "App.h"
#include "GLFW/glfw3.h"

namespace Fykor::Input
{
	namespace
	{
		bool IsKeyPressedImpl(int keycode)
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			auto state = glfwGetKey(window, keycode);
			return state == GLFW_PRESS || state == GLFW_REPEAT;
		}

		bool IsMouseButtonPressedImpl(int button)
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			auto state = glfwGetMouseButton(window, button);
			return state == GLFW_PRESS;
		}

		Vector2 GetMousePositionImpl()
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			double xpos, ypos;
			glfwGetCursorPos(window, &xpos, &ypos);
			return Vector2(static_cast<float>(xpos), static_cast<float>(ypos));
		}

		float GetMouseXImpl() { return GetMousePositionImpl().x; }

		float GetMouseYImpl() { return GetMousePositionImpl().y; }
	} // namespace

	bool IsKeyPressed(int keycode) { return IsKeyPressedImpl(keycode); }

	bool IsMouseButtonPressed(int button) { return IsMouseButtonPressedImpl(button); }

	Vector2 GetMousePosition() { return GetMousePositionImpl(); }

	float GetMouseX() { return GetMouseXImpl(); }

	float GetMouseY() { return GetMouseYImpl(); }

} // namespace Fykor::Input
