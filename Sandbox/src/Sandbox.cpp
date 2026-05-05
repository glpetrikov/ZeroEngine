// #include <ZeroEngine.h>
// #include <imgui.h>

// using namespace ZeroEngine;

// class ExampleLayer : public Layers::Layer {
// public:
//   ExampleLayer() : Layer("Example") {}

//   void OnUpdate() {}

//   void OnEvent(Events::Event &event) {
//     if (event.GetEventType() == Events::EventType::KeyPressed) {
//       Events::KeyPressedEvent &e = (Events::KeyPressedEvent &)event;
//       ZE_INFO("Key pressed: {0}", (char)e.GetKeyCode());
//     }
//   }

//   void OnImGuiRender() {
//     ImGui::Begin("Sandbox");
//     ImGui::Text("Hello ZEom Sandbox!");
//     ImGui::End();
//   }
// };

// class Sandbox : public ZeroEngine::App {
// public:
//   Sandbox() { PushLayer(new ExampleLayer()); }

//   ~Sandbox() {}
// };

// ZeroEngine::App *ZeroEngine::CreateApp() { return new Sandbox; }
