/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Layer.h
 * ─────────────────────────────────────────────────
 * Layer class declaration
 * =================================================
 */
#pragma once

#include "../Core.h"
#include "../Events/Event.h"

namespace ZeroEngine::Layers {
class ZE_API Layer {
  public:
	Layer(const std::string& debugname = "Default");
	virtual ~Layer();

	virtual void OnAttach() {}

	virtual void OnDetach() {}

	virtual void OnUpdate() {}

	virtual void OnImGuiRender() {}

	virtual void OnEvent(Events::Event& event) {}

	inline const std::string& GetName() const { return m_DebugName; }

  protected:
	std::string m_DebugName;
};
} // namespace ZeroEngine::Layers
