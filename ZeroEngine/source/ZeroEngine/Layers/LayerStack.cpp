/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * LayerStack.cpp
 * ─────────────────────────────────────────────────
 * LayerStack class implementation
 * =================================================
 */

#include "Common.h"
#include "LayerStack.h"

namespace ZeroEngine::Layers {
LayerStack::LayerStack() {
	m_LayerInsertIndex = 0;
}

LayerStack::~LayerStack() {
	for (Layer* layer : m_Layers) {
		delete layer;
	}
}

void LayerStack::PushLayer(Layer* layer) {
	m_Layers.emplace(m_Layers.begin() + m_LayerInsertIndex, layer);
	m_LayerInsertIndex++;
	layer->OnAttach();
}

void LayerStack::PushOverlay(Layer* overlay) {
	m_Layers.emplace_back(overlay);
	overlay->OnAttach();
}

void LayerStack::PopLayer(Layer* layer) {
	auto it = std::find(m_Layers.begin(), m_Layers.end(), layer);
	if (it != m_Layers.end()) {
		m_Layers.erase(it);
		m_LayerInsertIndex--;
	}
}

void LayerStack::PopOverlay(Layer* overlay) {
	auto it = std::find(m_Layers.begin(), m_Layers.end(), overlay);
	if (it != m_Layers.end()) {
		m_Layers.erase(it);
	}
}

std::vector<Layer*>::iterator LayerStack::begin() {
	return m_Layers.begin();
}

std::vector<Layer*>::iterator LayerStack::end() {
	return m_Layers.end();
}
} // namespace ZeroEngine::Layers
