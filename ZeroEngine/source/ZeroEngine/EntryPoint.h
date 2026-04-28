/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * EntryPoint.h
 * ─────────────────────────────────────────────────
 * Entry point for the ZeroEngine application.
 * =================================================
 */

#pragma once
#include "App.h"
#include "Common.h" // IWYU pragma: export

int main(int argc, char** argv) {
	ZE_CORE_INFO("Engine Started!\n");

	auto* app = ZeroEngine::CreateApp();
	app->Run();
	delete app;
}
