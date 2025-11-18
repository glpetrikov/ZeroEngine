#pragma once

#include "App.hpp"
#include <iostream>

#ifdef FK_PLATFORM_LINUX
extern "C" int main(int argc, char** argv) {
    std::cout << "[Fykor] Engine Started!" << '\n';
    auto* app = Fykor::CreateApp();
    app->Run();
    delete app;
}
#endif