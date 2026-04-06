/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * OpenGLContext.h
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
#pragma once
#include "../GraphicsContext.h"

struct GLFWwindow;

namespace Fykor
{
	class OpenGLContext : public GraphicsContext
	{
	public:
		OpenGLContext(GLFWwindow* windowHandle);
		virtual void Init() override;
		virtual void SwapBuffers() override;
		virtual void SetVSync(bool enable) override;

	private:
		GLFWwindow* m_WindowHandle;
	};
} // namespace Fykor
