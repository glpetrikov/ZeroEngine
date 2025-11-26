/* =================================================
* Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Sandbox
 * Sandbox.cpp
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
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

class ExampleLayer : public Layers::Layer {
public:
    ExampleLayer() : Layer("Example") {
    }

    void OnUpdate() {
        //FR_INFO("ExampleLayer::Update");
    }

    void OnEvent(Events::Event& event) {
        FR_INFO("Event: {0}", event.ToString());
    }
};

class Sandbox : public Fykor::App {
public:
    Sandbox() {
        PushLayer(new ExampleLayer());
    }

    ~Sandbox() {
    }
};

Fykor::App *Fykor::CreateApp() {
    return new Sandbox;
}