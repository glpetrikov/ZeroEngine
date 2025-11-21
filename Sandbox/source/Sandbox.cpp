#include <Fykor.h>

using namespace Fykor;

class Sandbox : public Fykor::App {
    std::unique_ptr<Window::Window> window;
    bool m_Running = true;

public:
    Sandbox() {
        window = std::unique_ptr<Window::Window>(Window::Window::Create());
        while (true) {
            window->OnUpdate();
        }
    }

    ~Sandbox() {
    }
};

Fykor::App *Fykor::CreateApp() {
    return new Sandbox;
}