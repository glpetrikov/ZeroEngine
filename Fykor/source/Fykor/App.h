/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * App.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.26
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Application class
 * =================================================
 */

#pragma once

#include "Core.h"
#include "Events/AppEvent.h"
#include "Events/Event.h"
#include "Events/KeyEvent.h"
#include "Events/MouseEvent.h"
#include "ImGui/ImGuiLayer.h"
#include "Layers/LayerStack.h"
#include "Window.h"

namespace Fykor
{
	class FYKOR_API App
	{
	public:
		App();
		virtual ~App();

		void PushLayer(Layers::Layer* layer);
		void PushOverlay(Layers::Layer* overlay);

		inline static App& Get() { return *s_Instance; }

		inline Window::Window& GetWindow() { return *m_Window; }

		inline unsigned int GetWindowWidth() { return m_Window->GetWidth(); }

		inline unsigned int GetWindowHeight() { return m_Window->GetHeight(); }

		void OnEvent(Events::Event& event);

		virtual void Run();

	private:
		bool OnWindowClose(Events::WindowCloseEvent& event);

		ImGuiLayer* m_ImGuiLayer;
		std::unique_ptr<Window::Window> m_Window;
		bool IsRunning = true;
		Layers::LayerStack m_LayerStack;

		static App* s_Instance;
	};

	// defined is User
	App* CreateApp();
} // namespace Fykor
