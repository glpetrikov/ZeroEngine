/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * KeyEvent.h
 * ─────────────────────────────────────────────────
 * Key Events
 * =================================================
 */

#pragma once

#include "../Input.h"
#include "Event.h"

namespace ZeroEngine::Events {
class ZE_API KeyEvent : public Event {
  public:
	inline Input::Keycode GetKeyCode() const { return m_KeyCode; }

	virtual int GetCategoryFlags() const override {
		return EventCategory::CategoryInput | EventCategory::CategoryKeyboard;
	}

  protected:
	KeyEvent(Input::Keycode keycode) : m_KeyCode(keycode) {}

	Input::Keycode m_KeyCode;
};

class ZE_API KeyPressedEvent : public KeyEvent {
  public:
	KeyPressedEvent(Input::Keycode keycode, int repeatCount) : KeyEvent(keycode), m_RepeatCount(repeatCount) {}

	inline int GetRepeatCount() const { return m_RepeatCount; }

	std::string ToString() const override {
		std::stringstream ss;
		ss << "KeyPressedEvent: " << Input::Keycode2Int(m_KeyCode) << "(" << m_RepeatCount << " repeats)";
		return ss.str();
	}

	static EventType GetType() { return EventType::KeyPressed; }

	virtual EventType GetEventType() const override { return GetType(); }

	virtual std::string GetName() const override { return "KeyPressed"; }

  private:
	int m_RepeatCount;
};

class ZE_API KeyReleasedEvent : public KeyEvent {
  public:
	KeyReleasedEvent(Input::Keycode keycode) : KeyEvent(keycode) {}

	std::string ToString() const override {
		std::stringstream ss;
		ss << "KeyReleasedEvent: " << Input::Keycode2Int(m_KeyCode);
		return ss.str();
	}

	static EventType GetType() { return EventType::KeyReleased; }

	virtual EventType GetEventType() const override { return GetType(); }

	virtual std::string GetName() const override { return "KeyReleased"; }
};

class KeyTypedEvent : public KeyEvent {
  public:
	KeyTypedEvent(const Input::Keycode keycode) : KeyEvent(keycode) {}

	std::string ToString() const override {
		std::stringstream ss;
		ss << "KeyTypedEvent: " << Input::Keycode2Int(m_KeyCode);
		return ss.str();
	}

	static EventType GetType() { return EventType::KeyTyped; }

	virtual EventType GetEventType() const override { return GetType(); }

	virtual std::string GetName() const override { return "KeyTyped"; }
};
} // namespace ZeroEngine::Events
