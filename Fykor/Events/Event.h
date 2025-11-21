#pragma once

#include "../Common.h"
#include "../Core.h"

namespace Fykor::Events {
    enum class EventType {
        None = 0,

        // Window Events
        WindowClose,
        WindowResize,
        WindowFocus,
        WindowLostFocus,
        WindowMoved,
        // App Events
        AppTick,
        AppUpdate,
        AppRender,
        // Key Events
        KeyPressed,
        KeyReleased,
        KeyTyped,
        // Mouse Events
        MouseButtonPressed,
        MouseButtonReleased,
        MouseMoved,
        MouseScrolled,
        // Game Entity Events
        EntityMoved,
        EntityDestroy,
        EntityCreated,
        EntityAddComponent
    };

    enum EventCategory {
        None = 0,
        CategoryApplication = BIT(0),
        CategoryInput = BIT(1),
        CategoryKeyboard = BIT(2),
        CategoryMouse = BIT(3),
        CategoryMouseButton = BIT(4)
    };

    class FYKOR_API Event {
    public:
        virtual ~Event() = default;

        bool GetHandler() { return m_Handler; }
        void SetHandler(bool is) {
            m_Handler = is;
            return;
        }

        virtual EventType GetEventType() const = 0;
        virtual std::string GetName() const = 0;

        virtual int GetCategoryFlags() const = 0;

        virtual std::string ToString() const { return GetName(); }

        inline bool IsInCategory(EventCategory Category) {
            return GetCategoryFlags() & Category;
        }

    protected:
        bool m_Handler = false;
    };

    class EventDispatcher {
    public:
        EventDispatcher(Event& event) : m_Event(event) {
        }

        template <typename T, typename F>
        bool Dispatch(const F& func) {
            if (m_Event.GetEventType() == T::GetType()) {
                m_Event.GetHandler() |= func(static_cast<T&>(m_Event));
                return true;
            }
            return false;
        }

    private:
        Event& m_Event;
    };
}