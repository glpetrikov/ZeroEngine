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
#include "Events/AppEvent.h"
#include "Common.h"

namespace Fykor {
#define BIND_EVENT_FN(x) std::bind(&x, this, std::placeholders::_1)

    App::App() {
        window = std::unique_ptr<Window::Window>(Window::Window::Create());
        window->SetEventCallback(BIND_EVENT_FN(App::OnEvent));
    }

    App::~App() {
    }

    void App::PushLayer(Layers::Layer* layer) {
        m_LayerStack.PushLayer(layer);
    }
    void App::PushOverlay(Layers::Layer* overlay) {
        m_LayerStack.PushOverlay(overlay);
    }

    void App::OnEvent(Events::Event& event) {
        Events::EventDispatcher dispatcher(event);

        dispatcher.Dispatch<Events::WindowCloseEvent>(BIND_EVENT_FN(App::OnWindowClose));

        for (auto it = m_LayerStack.end(); it != m_LayerStack.begin(); ) {
            (*--it)->OnEvent(event);
            if (event.GetHandler()) {
                break;
            }
        }
    }

    void App::Run() {
        while (IsRunning) {
            for (Layers::Layer* layer : m_LayerStack) {
                layer->OnUpdate();
            }
            window->OnUpdate();
            Fykor::Debug::FykorLogger.Flush();
            Fykor::Debug::Logger.Flush();
        }
    }

    bool App::OnWindowClose(Events::WindowCloseEvent &event) {
        IsRunning = false;
        return true;
    }
}