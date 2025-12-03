#pragma once

#include "Events/AppEvent.h"
#include "Events/KeyEvent.h"
#include "Events/MouseEvent.h"
#include "Layer.h"

namespace Fykor::Layers {
    class FYKOR_API ImGuiLayer : public Layers::Layer {
    public:
        ImGuiLayer();
        ~ImGuiLayer();

        void OnAttach() override;
        void OnDetach() override;
        void OnUpdate() override;
        void OnEvent(Events::Event& event) override;
    private:
        bool OnMouseButtonPressedEvent(Events::MouseButtonPressedEvent& event);
        bool OnMouseButtonReleasedEvent(Events::MouseButtonReleasedEvent& event);
        bool OnMouseMovedEvent(Events::MouseMovedEvent& event);
        bool OnMouseScrolledEvent(Events::MouseScrolledEvent& event);
    private:
        bool OnKeyPressedEvent(Events::KeyPressedEvent& event);
        bool OnKeyReleasedEvent(Events::KeyReleasedEvent& event);
        bool OnKeyTypedEvent(Events::KeyTypedEvent& event);
    private:
        bool OnWindowResizeEvent(Events::WindowResizeEvent& event);
    private:
        float m_Time = 0.0f;
    };
}