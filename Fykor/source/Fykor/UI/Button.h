/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Fykor
 * Button.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.14
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Button class
 * =================================================
 */
#pragma once
#include <imgui.h>
#include "../Common.h"

namespace Fykor::UI
{
	class Button
	{
	public:
		Button(const std::string& label, const ImVec2& size = ImVec2(0, 0)) : m_Label(label), m_Size(size) {}

		void SetCallback(const std::function<void()>& callback) { m_Callback = callback; }

		void Draw()
		{
			if (ImGui::Button(m_Label.c_str(), m_Size))
			{
				if (m_Callback)
					m_Callback();
			}
		}

	private:
		std::string m_Label;
		ImVec2 m_Size;
		std::function<void()> m_Callback;
	};
} // namespace Fykor::UI
