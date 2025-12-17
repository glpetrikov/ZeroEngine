/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Sandbox
 * Sandbox.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.14
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Test Project of Fykor
 * =================================================
 */

#include <Fykor.h>

using namespace Fykor;

class ExampleLayer : public Layers::Layer
{
public:
	ExampleLayer() : Layer("Example") {}

	void OnUpdate() {}

	void OnEvent(Events::Event& event)
	{
		if (event.GetEventType() == Events::EventType::KeyPressed)
		{
			Events::KeyPressedEvent& e = (Events::KeyPressedEvent&)event;
			FR_INFO("Key pressed: {0}", (char)e.GetKeyCode());
		}
	}
};

class Sandbox : public Fykor::App
{
public:
	Sandbox()
	{
		PushLayer(new ExampleLayer());
		PushOverlay(new Fykor::Layers::ImGuiLayer());
	}

	~Sandbox() {}
};

Fykor::App* Fykor::CreateApp() { return new Sandbox; }
