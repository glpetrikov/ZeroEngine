/* =================================================
* Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * App.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
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

    void App::OnEvent(Events::Event& event) {
        Events::EventDispatcher dispatcher(event);

        dispatcher.Dispatch<Events::WindowCloseEvent>(BIND_EVENT_FN(App::OnWindowClose));

        FR_CORE_INFO("{0}", event.ToString());
    }

    void App::Run() {
        while (IsRunning) {
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