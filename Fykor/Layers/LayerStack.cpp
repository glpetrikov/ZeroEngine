/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * LayerStack.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.26
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * LayerStack class
 * =================================================
 */
#include "LayerStack.h"
#include "Common.h"

namespace Fykor::Layers
{
	LayerStack::LayerStack() { m_LayerInsert = m_Layers.begin(); }

	LayerStack::~LayerStack()
	{
		for (Layer* layer : m_Layers)
		{
			delete layer;
		}
	}

	void LayerStack::PushLayer(Layer* layer)
	{
		m_LayerInsert = m_Layers.emplace(m_LayerInsert, layer);
		layer->OnAttach();
	}

	void LayerStack::PushOverlay(Layer* overlay)
	{
		m_Layers.emplace_back(overlay);
		overlay->OnAttach();
	}

	void LayerStack::PopLayer(Layer* layer)
	{
		auto it = std::find(m_Layers.begin(), m_Layers.end(), layer);
		if (it != m_Layers.end())
		{
			m_Layers.erase(it);
			m_LayerInsert--;
		}
	}

	void LayerStack::PopOverlay(Layer* overlay)
	{
		auto it = std::find(m_Layers.begin(), m_Layers.end(), overlay);
		if (it != m_Layers.end())
		{
			m_Layers.erase(it);
		}
	}

	std::vector<Layer*>::iterator LayerStack::begin() { return m_Layers.begin(); }

	std::vector<Layer*>::iterator LayerStack::end() { return m_Layers.end(); }
} // namespace Fykor::Layers
