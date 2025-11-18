#include <Fykor.hpp>

class Sandbox : public Fykor::App {
public:
    Sandbox() {
    }
    ~Sandbox() {
    }
};

Fykor::App* Fykor::CreateApp() {
    return new Sandbox;
}