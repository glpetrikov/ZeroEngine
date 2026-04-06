/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Window.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * header of Window
 * =================================================
 */

#pragma once

#include "Common.h"
#include "Core.h"
#include "Events/Event.h"

#include "Renderer/GraphicsContext.h"

struct GLFWwindow;

namespace Fykor::Window
{
	inline static uint32_t s_WindowCount = 0;

	struct WindowData
	{
		using EventCallbackFn = std::function<void(Events::Event&)>;

		EventCallbackFn EventCallback = [](Events::Event& event) {};

		std::string Name;
		unsigned int Width;
		unsigned int Height;

		bool VSync;

		inline WindowData(const std::string& name = "Fykor Engine", unsigned int width = 1280,
						  unsigned int height = 720) : Name(name)
		{
			Width = width;
			Height = height;
		}
	};

	class FYKOR_API Window
	{

	public:
		using EventCallbackFn = std::function<void(Events::Event&)>;

		Window(const WindowData& data);
		~Window();

		void OnUpdate();

		unsigned int GetWidth() const { return Data.Width; }

		unsigned int GetHeight() const { return Data.Height; }

		void SetEventCallback(const EventCallbackFn& callback) { Data.EventCallback = callback; }

		void SetVSync(bool enable);
		bool IsVSync() const;

		static Window* Create(const WindowData& data = WindowData());

		void* GetNativeWindow() const { return m_Window; }

	private:
		void Init(const WindowData& data);
		void ShutDown();

		GLFWwindow* m_Window;

		WindowData Data;

		GraphicsContext* m_Context;
	};
} // namespace Fykor::Window
