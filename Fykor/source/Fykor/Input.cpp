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

namespace Fykor
{

	bool Input::IsKeyPressedImpl(int keycode)
	{
		auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
		auto state = glfwGetKey(window, keycode);
		return state == GLFW_PRESS || state == GLFW_REPEAT;
	}

	bool Input::IsMouseButtonPressedImpl(int button)
	{
		auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
		auto state = glfwGetMouseButton(window, button);
		return state == GLFW_PRESS;
	}

	Vector2 Input::GetMousePositionImpl()
	{
		auto window = static_cast<GLFWwindow*>(App::Get().GetWindow().GetNativeWindow());
		double xpos, ypos;
		glfwGetCursorPos(window, &xpos, &ypos);
		return Vector2((float)xpos, (float)ypos);
	}

	float Input::GetMouseXImpl()
	{
		Vector2 v = GetMousePosition();
		return v.x;
	}

	float Input::GetMouseYImpl()
	{
		Vector2 v = GetMousePosition();
		return v.y;
	}

} // namespace Fykor
