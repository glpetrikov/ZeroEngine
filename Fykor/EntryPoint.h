#pragma once
#include "Common.h"

#include "Vectors/Vector2.h"
#include "Vectors/Vector3.h"

#include "App.h"
#include <FrameLog.hpp>

#ifdef FR_PLATFORM_LINUX
int main(int argc, char **argv) {
    FR_INFO("Engine Started!\n");

    auto *app = Fykor::CreateApp();
    app->Run();
    delete app;
}
#endif