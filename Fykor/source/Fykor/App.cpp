/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * App.cpp
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

#include "App.h"
#include <glad/glad.h>
#include "Common.h"
#include "Events/AppEvent.h"

namespace Fykor
{
#define BIND_EVENT_FN(x) std::bind(&x, this, std::placeholders::_1)

	App* App::s_Instance = nullptr;

	App::App()
	{
		FR_CORE_ASSERT(!s_Instance, "Application already exists!");
		s_Instance = this;

		window = std::unique_ptr<Window::Window>(Window::Window::Create());
		window->SetEventCallback(BIND_EVENT_FN(App::OnEvent));
	}

	App::~App() {}

	void App::PushLayer(Layers::Layer* layer) { m_LayerStack.PushLayer(layer); }

	void App::PushOverlay(Layers::Layer* overlay) { m_LayerStack.PushOverlay(overlay); }

	void App::OnEvent(Events::Event& event)
	{
		Events::EventDispatcher dispatcher(event);

		dispatcher.Dispatch<Events::WindowCloseEvent>(BIND_EVENT_FN(App::OnWindowClose));

		for (auto it = m_LayerStack.end(); it != m_LayerStack.begin();)
		{
			--it;
			(*it)->OnEvent(event);
			if (event.GetHandler())
			{
				break;
			}
		}
	}

	void App::Run()
	{
		while (IsRunning)
		{
			glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
			glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
			for (Layers::Layer* layer : m_LayerStack)
			{
				layer->OnUpdate();
			}
			window->OnUpdate();
			Fykor::Debug::FykorLogger.Flush();
			Fykor::Debug::Logger.Flush();
		}
	}

	bool App::OnWindowClose(Events::WindowCloseEvent& event)
	{
		IsRunning = false;
		return true;
	}
} // namespace Fykor
