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
		bool IsKeyPressedImpl(Keycode keycode)
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			auto state = glfwGetKey(window, Keycode2Int(keycode));
			return state == GLFW_PRESS || state == GLFW_REPEAT;
		}

		bool IsKeyUpImpl(Keycode keycode)
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			auto state = glfwGetKey(window, Keycode2Int(keycode));
			return state == GLFW_RELEASE;
		}

		bool IsKeyDownImpl(Keycode keycode)
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			auto state = glfwGetKey(window, Keycode2Int(keycode));
			return state == GLFW_PRESS;
		}

		bool IsMouseButtonPressedImpl(MouseButton button)
		{
			auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
			auto state = glfwGetMouseButton(window, MouseButton2Int(button));
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

	bool IsKeyPressed(Keycode keycode) { return IsKeyPressedImpl(keycode); }

	bool IsKeyUp(Keycode keycode) { return IsKeyUpImpl(keycode); }

	bool IsKeyDown(Keycode keycode) { return IsKeyDownImpl(keycode); }

	// Mouse Functions

	bool IsMouseButtonPressed(MouseButton button) { return IsMouseButtonPressedImpl(button); }

	Vector2 GetMousePosition() { return GetMousePositionImpl(); }

	float GetMouseX() { return GetMouseXImpl(); }

	float GetMouseY() { return GetMouseYImpl(); }

} // namespace Fykor::Input
