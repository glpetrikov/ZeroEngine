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
#include "Input.h"
#include "imgui.h"

namespace Fykor
{
#define BIND_EVENT_FN(x) std::bind(&x, this, std::placeholders::_1)

	App* App::s_Instance = nullptr;

	App::App()
	{
		FR_CORE_ASSERT(!s_Instance, "Application already exists!");
		s_Instance = this;

		m_Window = std::unique_ptr<Window::Window>(Window::Window::Create());
		m_Window->SetEventCallback(BIND_EVENT_FN(App::OnEvent));

		m_ImGuiLayer = new ImGuiLayer();
		PushOverlay(m_ImGuiLayer);

		glGenVertexArrays(1, &m_VertexArray);
		glBindVertexArray(m_VertexArray);

		glGenBuffers(1, &m_VertexBuffer);
		glBindBuffer(GL_ARRAY_BUFFER, m_VertexBuffer);

		float vertices[3 * 3] = {-0.5f, -0.5f, 0.0f, 0.5f, -0.5f, 0.0f, 0.0f, 0.5f, 0.0f};

		glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

		glEnableVertexAttribArray(0);
		glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), nullptr);

		glGenBuffers(1, &m_IndexBuffer);
		glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_IndexBuffer);

		unsigned int indices[3] = {0, 1, 2};
		glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);
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
			glClear(GL_COLOR_BUFFER_BIT);

			glBindVertexArray(m_VertexArray);
			glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, nullptr);

			for (Layers::Layer* layer : m_LayerStack)
			{
				layer->OnUpdate();
			}

			m_ImGuiLayer->Begin();
			for (Layers::Layer* layer : m_LayerStack)
			{
				layer->OnImGuiRender();
			}
			m_ImGuiLayer->End();

			m_Window->OnUpdate();
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
