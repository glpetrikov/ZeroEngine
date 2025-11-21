#pragma once
#include "Common.h"

#include "Vectors/Vector2.h"
#include "Vectors/Vector3.h"

#include "App.h"
#include <FrameLog.hpp>

#ifdef FR_PLATFORM_LINUX
int main(int argc, char **argv) {

    Fykor::Vectors::Vector2<float> vector(1, 1);

    using namespace Fykor::Debug;

    FykorLogger.InfoLine("Engine Started!");
    FykorLogger.Flush();

    std::cout << vector << '\n';

    auto *app = Fykor::CreateApp();
    app->Run();
    delete app;
}
#endif