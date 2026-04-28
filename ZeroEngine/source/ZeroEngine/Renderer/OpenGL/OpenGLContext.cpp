/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * OpenGLContext.cpp
 * ─────────────────────────────────────────────────
 * OpenGL Context implementation
 * =================================================
 */

#include "../../Common.h"
#include "../../Core.h"
#include "OpenGLContext.h"

#include <GLFW/glfw3.h>
#include <glad/glad.h>

namespace ZeroEngine {

OpenGLContext::OpenGLContext(GLFWwindow* windowHandle) : m_WindowHandle(windowHandle) {
	ZE_CORE_ASSERT(windowHandle, "Window handle is null!");
}

void OpenGLContext::Init() {
	glfwMakeContextCurrent(m_WindowHandle);
	int status = gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);
	ZE_CORE_ASSERT(status, "Failed to Init Glad!");

	ZE_CORE_INFO("OpenGL Info:");
	ZE_CORE_INFO("  Vendor: {0}", reinterpret_cast<const char*>(glGetString(GL_VENDOR)));
	ZE_CORE_INFO("  Renderer: {0}", reinterpret_cast<const char*>(glGetString(GL_RENDERER)));
	ZE_CORE_INFO("  Version: {0}", reinterpret_cast<const char*>(glGetString(GL_VERSION)));
}

void OpenGLContext::SwapBuffers() {
	glfwSwapBuffers(m_WindowHandle);
}

void OpenGLContext::SetVSync(bool enable) {
	glfwSwapInterval(enable ? 1 : 0);
}

} // namespace ZeroEngine
