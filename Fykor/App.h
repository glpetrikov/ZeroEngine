#pragma once

#include "Core.h"
#include "Events/Event.h"
#include "Events/AppEvent.h"
#include "Events/KeyEvent.h"
#include "Events/MouseEvent.h"

#include "Window.h"

namespace Fykor {
    class FYKOR_API App {
    public:
        App();
        virtual ~App();

        void OnEvent(Events::Event& event);

        virtual void Run();
    private:
        bool OnWindowClose(Events::WindowCloseEvent& event);

        std::unique_ptr<Window::Window> window;
        bool IsRunning = true;
    };

    // defined is User
    App *CreateApp();
}