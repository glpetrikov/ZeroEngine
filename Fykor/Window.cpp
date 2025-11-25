/* =================================================
* Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Window.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * source file of Window
 * =================================================
 */

#include "Window.h"

#include "Common.h"
#include <GLFW/glfw3.h>

#include "Events/AppEvent.h"
#include "Events/KeyEvent.h"
#include "Events/MouseEvent.h"

namespace Fykor::Window {
    static bool s_GLFWInitilized = false;

    static void glfwErrorCallback(int error, const char* message) {
        FR_CORE_ERROR("GLFW Error: ({0}): {1}", error, message);
    }

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

        FR_CORE_INFO("Creating window {0} {1} {2}",
        data.Name,
        data.Width,
        data.Height);


        if (!s_GLFWInitilized) {
            glfwSetErrorCallback(glfwErrorCallback);
            int success = glfwInit();

            if (!success) {
                FR_CORE_ERROR("Could not Initialize GLFW!");
                return;
            }

            FR_CORE_INFO("GLFW Initialized successfully!");
            s_GLFWInitilized = true;
        }
        window = glfwCreateWindow((int)data.Width, (int)data.Height, Data.Name.c_str(), nullptr, nullptr);
        glfwMakeContextCurrent(window);
        glfwSetWindowUserPointer(window, &Data);
        SetVSync(true);

        // Set GLFW CallBack's

        glfwSetWindowSizeCallback(window, [](GLFWwindow* window, int width, int height) {
            WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

            data.Width = width;
            data.Height = height;

            Events::WindowResizeEvent event(width, height);
            data.EventCallback(event);
        });

        glfwSetWindowCloseCallback(window, [](GLFWwindow* window) {
            WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);
            Events::WindowCloseEvent event;
            data.EventCallback(event);
        });

        glfwSetKeyCallback(window, [](GLFWwindow* window, int key, int scancode, int action, int mods) {
            WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

            switch (action) {
                case GLFW_PRESS: {
                    Events::KeyPressedEvent event(key, 0);
                    data.EventCallback(event);
                    break;
                }
                case GLFW_RELEASE: {
                    Events::KeyReleasedEvent event(key);
                    data.EventCallback(event);
                    break;
                }
                case GLFW_REPEAT: {
                    Events::KeyPressedEvent event(key, 1);
                    data.EventCallback(event);
                    break;
                }
            }
        });

        glfwSetMouseButtonCallback(window, [](GLFWwindow* window, int button, int action, int mods) {
            WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

            switch (action) {
                case GLFW_PRESS: {
                    Events::MouseButtonPressedEvent event(button);
                    data.EventCallback(event);
                    break;
                }
                case GLFW_RELEASE: {
                    Events::MouseButtonReleasedEvent event(button);
                    data.EventCallback(event);
                    break;
                }
            }
        });

        glfwSetScrollCallback(window, [](GLFWwindow* window, double xoffset, double yoffset) {
            WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

            Events::MouseScrolledEvent event((float)xoffset, (float)yoffset);
            data.EventCallback(event);
        });

        glfwSetCursorPosCallback(window, [](GLFWwindow* window, double xPos, double yPos) {
            WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

            Events::MouseMovedEvent event((float)xPos, (float)yPos);
            data.EventCallback(event);
        });

        if (Data.EventCallback) {
            FR_CORE_INFO("EventCallback is SET!");
        } else {
            FR_CORE_ERROR("EventCallback is NULL!");
        }
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