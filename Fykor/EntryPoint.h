/* =================================================
* Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * EntryPoint.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * EntryPoint
 * =================================================
 */

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