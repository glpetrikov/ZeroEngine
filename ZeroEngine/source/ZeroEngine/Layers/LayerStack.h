/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * App.h
 * ─────────────────────────────────────────────────
 * Application for the ZeroEngine
 * =================================================
 */

#pragma once

#include "../Core.h"
#include "Layer.h"

#include <vector>

namespace ZeroEngine::Layers {
class LayerStack {
  public:
	LayerStack();
	~LayerStack();

	void PushLayer(Layer* layer);
	void PushOverlay(Layer* overlay);
	void PopLayer(Layer* layer);
	void PopOverlay(Layer* overlay);

	std::vector<Layer*>::iterator begin();
	std::vector<Layer*>::iterator end();

  private:
	std::vector<Layer*> m_Layers;
	unsigned int m_LayerInsertIndex = 0;
};
} // namespace ZeroEngine::Layers
