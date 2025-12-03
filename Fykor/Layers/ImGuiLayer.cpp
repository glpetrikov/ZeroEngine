#include "Common.h"
#include "ImGuiLayer.h"

#include "Common.h"
#include "Events/AppEvent.h"
#include "Events/Event.h"
#include "Events/KeyEvent.h"
#include "Events/MouseEvent.h"

#include "App.h"
#include <backends/imgui_impl_opengl3.h>
#include <imgui.h>

#define IMGUI_IMPL_OPENGL_LOADER_GLAD

// TEMPORARY
#include "GLFW/glfw3.h"
#include <glad/glad.h>

namespace Fykor::Layers {

    static ImGuiKey GLFWKeyToImGuiKey(int glfwKey)
    {
        switch (glfwKey)
        {
            case GLFW_KEY_TAB: return ImGuiKey_Tab;
            case GLFW_KEY_LEFT: return ImGuiKey_LeftArrow;
            case GLFW_KEY_RIGHT: return ImGuiKey_RightArrow;
            case GLFW_KEY_UP: return ImGuiKey_UpArrow;
            case GLFW_KEY_DOWN: return ImGuiKey_DownArrow;
            case GLFW_KEY_PAGE_UP: return ImGuiKey_PageUp;
            case GLFW_KEY_PAGE_DOWN: return ImGuiKey_PageDown;
            case GLFW_KEY_HOME: return ImGuiKey_Home;
            case GLFW_KEY_END: return ImGuiKey_End;
            case GLFW_KEY_INSERT: return ImGuiKey_Insert;
            case GLFW_KEY_DELETE: return ImGuiKey_Delete;
            case GLFW_KEY_BACKSPACE: return ImGuiKey_Backspace;
            case GLFW_KEY_SPACE: return ImGuiKey_Space;
            case GLFW_KEY_ENTER: return ImGuiKey_Enter;
            case GLFW_KEY_ESCAPE: return ImGuiKey_Escape;
            case GLFW_KEY_LEFT_CONTROL: return ImGuiKey_LeftCtrl;
            case GLFW_KEY_LEFT_SHIFT: return ImGuiKey_LeftShift;
            case GLFW_KEY_LEFT_ALT: return ImGuiKey_LeftAlt;
            case GLFW_KEY_LEFT_SUPER: return ImGuiKey_LeftSuper;
            case GLFW_KEY_RIGHT_CONTROL: return ImGuiKey_RightCtrl;
            case GLFW_KEY_RIGHT_SHIFT: return ImGuiKey_RightShift;
            case GLFW_KEY_RIGHT_ALT: return ImGuiKey_RightAlt;
            case GLFW_KEY_RIGHT_SUPER: return ImGuiKey_RightSuper;
            case GLFW_KEY_CAPS_LOCK: return ImGuiKey_CapsLock;
            case GLFW_KEY_SCROLL_LOCK: return ImGuiKey_ScrollLock;
            case GLFW_KEY_NUM_LOCK: return ImGuiKey_NumLock;
            case GLFW_KEY_PRINT_SCREEN: return ImGuiKey_PrintScreen;
            case GLFW_KEY_PAUSE: return ImGuiKey_Pause;
            default: return ImGuiKey_None;
        }
    }

    ImGuiLayer::ImGuiLayer() : Layer("ImGuiLayer") {
    }

    ImGuiLayer::~ImGuiLayer() {
    }

    void ImGuiLayer::OnAttach() {
        ImGui::CreateContext();
        ImGui::StyleColorsDark();

        ImGuiIO &io = ImGui::GetIO();
        io.BackendFlags |= ImGuiBackendFlags_HasMouseCursors;
        io.BackendFlags |= ImGuiBackendFlags_HasSetMousePos;

        ImGui_ImplOpenGL3_Init("#version 410");
    }

    void ImGuiLayer::OnDetach() {
    }

    void ImGuiLayer::OnUpdate() {
        ImGuiIO &io = ImGui::GetIO();
        App &app = App::Get();

        io.DisplaySize = ImVec2((float)app.GetWindow().GetWidth(), (float)app.GetWindow().GetHeight());

        float time = (float)glfwGetTime();
        io.DeltaTime = m_Time > 0.0f ? (time - m_Time) : (1.0f / 60.0f);
        m_Time = time;

        ImGui_ImplOpenGL3_NewFrame();
        ImGui::NewFrame();

        static bool show = true;
        ImGui::ShowDemoWindow(&show);

        ImGui::Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
    }

    void ImGuiLayer::OnEvent(Events::Event &event) {
        Events::EventDispatcher dispatcher(event);
        dispatcher.Dispatch<Events::MouseButtonPressedEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnMouseButtonPressedEvent));
        dispatcher.Dispatch<Events::MouseButtonReleasedEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnMouseButtonReleasedEvent));
        dispatcher.Dispatch<Events::MouseMovedEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnMouseMovedEvent));
        dispatcher.Dispatch<Events::MouseScrolledEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnMouseScrolledEvent));
        dispatcher.Dispatch<Events::KeyPressedEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnKeyPressedEvent));
        dispatcher.Dispatch<Events::KeyReleasedEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnKeyReleasedEvent));
        dispatcher.Dispatch<Events::KeyTypedEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnKeyTypedEvent));
        dispatcher.Dispatch<Events::WindowResizeEvent>(HZ_BIND_EVENT_FN(ImGuiLayer::OnWindowResizeEvent));

    }

    bool ImGuiLayer::OnMouseButtonPressedEvent(Events::MouseButtonPressedEvent& event) {
        ImGuiIO & io = ImGui::GetIO();
        io.MouseDown[event.GetMouseButton()] = true;

        return false;
    }
    bool ImGuiLayer::OnMouseButtonReleasedEvent(Events::MouseButtonReleasedEvent& event) {
        ImGuiIO & io = ImGui::GetIO();
        io.MouseDown[event.GetMouseButton()] = false;

        return false;
    }
    bool ImGuiLayer::OnMouseMovedEvent(Events::MouseMovedEvent& event) {
        ImGuiIO & io = ImGui::GetIO();
        io.MousePos = ImVec2(event.GetX(), event.GetY());

        return false;
    }
    bool ImGuiLayer::OnMouseScrolledEvent(Events::MouseScrolledEvent& event) {
        ImGuiIO & io = ImGui::GetIO();
        io.MouseWheelH += event.GetOffsetX();
        io.MouseWheel += event.GetOffsetY();

        return false;
    }
    bool ImGuiLayer::OnKeyPressedEvent(Events::KeyPressedEvent& event) {
        ImGuiKey key = GLFWKeyToImGuiKey(event.GetKeyCode());
        if (key != ImGuiKey_None) {
            ImGui::GetIO().AddKeyEvent(key, true);
        }
        return false;
    }

    bool ImGuiLayer::OnKeyReleasedEvent(Events::KeyReleasedEvent& event) {
        ImGuiKey key = GLFWKeyToImGuiKey(event.GetKeyCode());
        if (key != ImGuiKey_None) {
            ImGui::GetIO().AddKeyEvent(key, false);
        }
        return false;
    }
    bool ImGuiLayer::OnKeyTypedEvent(Events::KeyTypedEvent& event) {
        ImGuiIO & io = ImGui::GetIO();
        int keycode = event.GetKeyCode();
        if (keycode > 0 && keycode < 0x10000) {
            io.AddInputCharacter((unsigned short)keycode);
        }
        return false;
    }
    bool ImGuiLayer::OnWindowResizeEvent(Events::WindowResizeEvent& event) {
        ImGuiIO & io = ImGui::GetIO();
        io.DisplaySize = ImVec2(event.GetWidth(), event.GetHeight());
        io.DisplayFramebufferScale = ImVec2(1.0f, 1.0f);
        glViewport(0, 0, event.GetWidth(), event.GetHeight());
        return false;
    }
}