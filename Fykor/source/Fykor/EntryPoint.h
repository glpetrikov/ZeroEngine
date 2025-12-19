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
#include "App.h"
#include "Common.h"

int main(int argc, char** argv)
{
	FR_CORE_INFO("Engine Started!\n");

	auto* app = Fykor::CreateApp();
	app->Run();
	delete app;
}
