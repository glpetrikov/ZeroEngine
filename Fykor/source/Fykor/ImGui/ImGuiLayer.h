/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * ImGuiLayer.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.17
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * ImGuiLayer class
 * =================================================
 */

#pragma once

#include "../Layers/Layer.h"

namespace Fykor
{
	class FYKOR_API ImGuiLayer : public Layers::Layer
	{
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
} // namespace Fykor
