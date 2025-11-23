#include <Fykor.h>

using namespace Fykor;

class Sandbox : public Fykor::App {
public:
    Sandbox() {
    }

    ~Sandbox() {
    }
};

Fykor::App *Fykor::CreateApp() {
    return new Sandbox;
}