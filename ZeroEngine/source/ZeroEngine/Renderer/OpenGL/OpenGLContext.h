/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * OpenGLContext.h
 * ─────────────────────────────────────────────────
 * Graphics Context for OpenGL
 * =================================================
 */

#pragma once
#include "../GraphicsContext.h"

struct GLFWwindow;

namespace ZeroEngine {
class OpenGLContext : public GraphicsContext {
  public:
	OpenGLContext(GLFWwindow* windowHandle);
	virtual void Init() override;
	virtual void SwapBuffers() override;
	virtual void SetVSync(bool enable) override;

  private:
	GLFWwindow* m_WindowHandle;
};
} // namespace ZeroEngine
