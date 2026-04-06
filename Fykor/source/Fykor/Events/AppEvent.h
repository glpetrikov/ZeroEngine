/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * AppEvent.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Application Events
 * =================================================
 */
#pragma once

#include "../Common.h"
#include "../Core.h"

#include "Event.h"

namespace Fykor::Events
{
	class FYKOR_API WindowResizeEvent : public Event
	{
	public:
		WindowResizeEvent(unsigned int width, unsigned int height) : m_Width(width), m_Height(height) {}

		inline unsigned int GetWidth() const { return m_Width; }

		inline unsigned int GetHeight() const { return m_Height; }

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "WindowResizeEvent: " << m_Width << ", " << m_Height;
			return ss.str();
		}

		static EventType GetType() { return EventType::WindowResize; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "WindowResize"; }

		virtual int GetCategoryFlags() const override { return EventCategory::CategoryApplication; }

	private:
		unsigned int m_Width;
		unsigned int m_Height;
	};

	class WindowCloseEvent : public Event
	{
	public:
		WindowCloseEvent() = default;

		static EventType GetType() { return EventType::WindowClose; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "WindowClose"; }

		virtual int GetCategoryFlags() const override { return EventCategory::CategoryApplication; }
	};

	class AppTickEvent : public Event
	{
		AppTickEvent() = default;

		static EventType GetType() { return EventType::AppTick; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "AppTick"; }

		virtual int GetCategoryFlags() const override { return EventCategory::CategoryApplication; }
	};

	class AppUpdateEvent : public Event
	{
	public:
		AppUpdateEvent() = default;

		static EventType GetType() { return EventType::AppUpdate; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "AppUpdate"; }

		virtual int GetCategoryFlags() const override { return EventCategory::CategoryApplication; }
	};

	class AppRenderEvent : public Event
	{
	public:
		AppRenderEvent() = default;

		static EventType GetType() { return EventType::AppRender; }

		virtual EventType GetEventType() const override { return GetType(); }

		virtual std::string GetName() const override { return "AppRender"; }

		virtual int GetCategoryFlags() const override { return EventCategory::CategoryApplication; }
	};
} // namespace Fykor::Events
