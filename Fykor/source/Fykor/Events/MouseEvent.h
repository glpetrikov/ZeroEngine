/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * MouseEvent.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Mouse Events
 * =================================================
 */

#pragma once

#include "../Common.h"
#include "Event.h"

namespace Fykor::Events
{

	class MouseMovedEvent : public Event
	{

	public:
		MouseMovedEvent(const float x, const float y)
		{
			vec.x = x;
			vec.y = y;
		}

		float GetX() const { return vec.x; }

		float GetY() const { return vec.y; }

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseMovedEvent: " << vec.x << ", " << vec.y;
			return ss.str();
		}

		virtual int GetCategoryFlags() const override
		{
			return EventCategory::CategoryInput | EventCategory::CategoryMouse;
		}

		static EventType GetType() { return EventType::MouseMoved; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "MouseMoved"; }

	private:
		Vector2 vec;
	};

	class MouseScrolledEvent : public Event
	{
	public:
		MouseScrolledEvent(const float xOffset, const float yOffset)
		{
			vec.x = xOffset;
			vec.y = yOffset;
		}

		float GetOffsetX() const { return vec.x; }

		float GetOffsetY() const { return vec.y; }

		Vector2 GetOffset() const { return vec; }

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseScrolledEvent: " << vec.x << ", " << vec.y;
			return ss.str();
		}

		virtual int GetCategoryFlags() const override
		{
			return EventCategory::CategoryInput | EventCategory::CategoryMouse;
		}

		static EventType GetType() { return EventType::MouseScrolled; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "MouseScrolled"; }

	private:
		Vector2 vec;
	};

	class MouseButtonEvent : public Event
	{
	public:
		inline int GetMouseButton() const { return m_Button; }

		virtual int GetCategoryFlags() const override
		{
			return EventCategory::CategoryInput | EventCategory::CategoryMouse;
		}

	protected:
		MouseButtonEvent(const int button) : m_Button(button) {}

		int m_Button;
	};

	class MouseButtonPressedEvent : public MouseButtonEvent
	{
	public:
		MouseButtonPressedEvent(const int button) : MouseButtonEvent(button) {}

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseButtonPressedEvent: " << m_Button;
			return ss.str();
		}

		static EventType GetType() { return EventType::MouseButtonPressed; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "MouseButtonPressed"; }
	};

	class MouseButtonReleasedEvent : public MouseButtonEvent
	{
	public:
		MouseButtonReleasedEvent(const int button) : MouseButtonEvent(button) {}

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseButtonReleasedEvent: " << m_Button;
			return ss.str();
		}

		static EventType GetType() { return EventType::MouseButtonReleased; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "MouseButtonReleased"; }
	};
} // namespace Fykor::Events
