/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * ImGuiLayer.h
 * ─────────────────────────────────────────────────
 * ImGuiLayer class
 * =================================================
 */

#pragma once

#include "../Layers/Layer.h"

namespace ZeroEngine {
class ZE_API ImGuiLayer : public Layers::Layer {
  public:
	ImGuiLayer();
	~ImGuiLayer();

	virtual void OnAttach() override;
	virtual void OnDetach() override;
	virtual void OnImGuiRender() override;

	void Begin();
	void End();

  private:
	float m_Time = 0.0f;
};
} // namespace ZeroEngine
