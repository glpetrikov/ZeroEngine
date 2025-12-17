/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * OpenGLContext.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.17
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Graphics Context
 * =================================================
 */
#include "OpenGLContext.h"
#include "../../Common.h"
#include "../../Core.h"

#include <GLFW/glfw3.h>
#include <glad/glad.h>

namespace Fykor
{

	OpenGLContext::OpenGLContext(GLFWwindow* windowHandle) : m_WindowHandle(windowHandle)
	{
		FR_CORE_ASSERT(windowHandle, "Window handle is null!");
	}

	void OpenGLContext::Init()
	{
		glfwMakeContextCurrent(m_WindowHandle);
		int status = gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);
		FR_CORE_ASSERT(status, "Failed to Init Glad!");
	}

	void OpenGLContext::SwapBuffers() { glfwSwapBuffers(m_WindowHandle); }

	void OpenGLContext::SetVSync(bool enable) { glfwSwapInterval(enable ? 1 : 0); }


} // namespace Fykor
