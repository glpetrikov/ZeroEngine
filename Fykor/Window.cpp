#include "Window.h"
#include "Core.h"
#include "Debug/Log.h"

namespace Fykor::Window {
    Window *Window::Create(const WindowData &data) {
        return new Window(data);
    }

    Window::Window(const WindowData &data) {
        Init(data);
    }

    Window::~Window() {
        ShutDown();
    }

    void Window::Init(const WindowData &data) {
        Data.Name = data.Name;
        Data.Width = data.Width;
        Data.Height = data.Height;

        Debug::FykorLogger.InfoLine(Debug::FykorLogger.Format("Creating window {0} {1} {2}",
                                                              data.Name,
                                                              data.Width,
                                                              data.Height));
        if (!s_GLFWInitilized) {
            int success = glfwInit();
            FR_CORE_ASSERT(success, "Could not Initilize GLFW!")

            s_GLFWInitilized = true;
        }
        window = glfwCreateWindow((int)data.Width, (int)data.Height, Data.Name.c_str(), nullptr, nullptr);
        glfwMakeContextCurrent(window);
        glfwSetWindowUserPointer(window, &Data);
        SetVSync(true);
    }

    void Window::ShutDown() {
        glfwDestroyWindow(window);
    }

    void Window::OnUpdate() {
        glfwPollEvents();
        glfwSwapBuffers(window);
    }

    void Window::SetVSync(bool enable) {
        if (enable) {
            glfwSwapInterval(1);
        } else {
            glfwSwapInterval(0);
        }

        Data.VSync = enable;
    }

    bool Window::IsVSync() const {
        return Data.VSync;
    }
}