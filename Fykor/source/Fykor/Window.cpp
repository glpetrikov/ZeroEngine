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

#include <GLFW/glfw3.h>
#include "Common.h"

#include "Events/AppEvent.h"
#include "Events/KeyEvent.h"
#include "Events/MouseEvent.h"
#include "Renderer/OpenGL/OpenGLContext.h"

namespace Fykor::Window
{
	static bool s_GLFWInitilized = false;

	static void glfwErrorCallback(int error, const char* message)
	{
		FR_CORE_ERROR("GLFW Error: ({0}): {1}", error, message);
	}

	Window* Window::Create(const WindowData& data) { return new Window(data); }

	Window::Window(const WindowData& data) { Init(data); }

	Window::~Window()
	{
		delete m_Context;
		ShutDown();
	}

	void Window::Init(const WindowData& data)
	{
		Data.Name = data.Name;
		Data.Width = data.Width;
		Data.Height = data.Height;

		FR_CORE_INFO("Creating window {0} {1} {2}", data.Name, data.Width, data.Height);


		if (!s_GLFWInitilized)
		{
			glfwSetErrorCallback(glfwErrorCallback);

			if (!glfwInit())
			{
				FR_CORE_ERROR("Could not initialize GLFW!");
				return;
			}

			s_GLFWInitilized = true;
		}

		++s_WindowCount;
		m_Window = glfwCreateWindow((int)data.Width, (int)data.Height, Data.Name.c_str(), nullptr, nullptr);
		int w, h;
		glfwGetFramebufferSize(m_Window, &w, &h);

		Data.Width = (unsigned int)w;
		Data.Height = (unsigned int)h;
		glfwSetWindowUserPointer(m_Window, &Data);

		m_Context = new OpenGLContext(m_Window);
		m_Context->Init();

		SetVSync(true);

		// Set GLFW CallBack's

		glfwSetFramebufferSizeCallback(m_Window,
									   [](GLFWwindow* window, int width, int height)
									   {
										   WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

										   data.Width = width;
										   data.Height = height;

										   Events::WindowResizeEvent event(width, height);
										   data.EventCallback(event);
									   });

		glfwSetWindowCloseCallback(m_Window,
								   [](GLFWwindow* window)
								   {
									   WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);
									   Events::WindowCloseEvent event;
									   data.EventCallback(event);
								   });

		glfwSetKeyCallback(m_Window,
						   [](GLFWwindow* window, int key, int scancode, int action, int mods)
						   {
							   WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

							   switch (action)
							   {
							   case GLFW_PRESS:
								   {
									   Events::KeyPressedEvent event(key, 0);
									   data.EventCallback(event);
									   break;
								   }
							   case GLFW_RELEASE:
								   {
									   Events::KeyReleasedEvent event(key);
									   data.EventCallback(event);
									   break;
								   }
							   case GLFW_REPEAT:
								   {
									   Events::KeyPressedEvent event(key, 1);
									   data.EventCallback(event);
									   break;
								   }
							   }
						   });

		glfwSetCharCallback(m_Window,
							[](GLFWwindow* window, unsigned int keycode)
							{
								WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);
								Events::KeyTypedEvent event(keycode);
								data.EventCallback(event);
							});

		glfwSetMouseButtonCallback(m_Window,
								   [](GLFWwindow* window, int button, int action, int mods)
								   {
									   WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

									   switch (action)
									   {
									   case GLFW_PRESS:
										   {
											   Events::MouseButtonPressedEvent event(button);
											   data.EventCallback(event);
											   break;
										   }
									   case GLFW_RELEASE:
										   {
											   Events::MouseButtonReleasedEvent event(button);
											   data.EventCallback(event);
											   break;
										   }
									   }
								   });

		glfwSetScrollCallback(m_Window,
							  [](GLFWwindow* window, double xoffset, double yoffset)
							  {
								  WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

								  Events::MouseScrolledEvent event((float)xoffset, (float)yoffset);
								  data.EventCallback(event);
							  });

		glfwSetCursorPosCallback(m_Window,
								 [](GLFWwindow* window, double xPos, double yPos)
								 {
									 WindowData& data = *(WindowData*)glfwGetWindowUserPointer(window);

									 Events::MouseMovedEvent event((float)xPos, (float)yPos);
									 data.EventCallback(event);
								 });
	}

	void Window::ShutDown()
	{
		glfwDestroyWindow(m_Window);
		--s_WindowCount;

		if (s_WindowCount == 0)
		{
			glfwTerminate();
			s_GLFWInitilized = false;
		}
	}

	void Window::OnUpdate()
	{
		glfwPollEvents();
		m_Context->SwapBuffers();
	}

	void Window::SetVSync(bool enable)
	{
		if (m_Context == nullptr)
		{
			FR_CORE_FATAL("To set vertical immobility, the context must be initialized (in Window.cpp)")
			exit(1);
		}
		else
		{
			m_Context->SetVSync(enable);
			Data.VSync = enable;
		}
	}

	bool Window::IsVSync() const { return Data.VSync; }
} // namespace Fykor::Window
