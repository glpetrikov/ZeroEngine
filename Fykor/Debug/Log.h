/* =================================================
* Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Log.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Debug
 * =================================================
 */

#pragma once

#include <FrameLog.hpp>

#include "Platform.h"

namespace Fykor::Debug {
    inline FrameLog::Logger FykorLogger("Fykor");
    inline FrameLog::Logger Logger("App");

#define FR_TRACE(...) Fykor::Debug::Logger.Trace(Fykor::Debug::Logger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_INFO(...) Fykor::Debug::Logger.Info(Fykor::Debug::Logger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_WARN(...) Fykor::Debug::Logger.Warn(Fykor::Debug::Logger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_ERROR(...) Fykor::Debug::Logger.Error(Fykor::Debug::Logger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_FATAL(...) Fykor::Debug::Logger.Fatal(Fykor::Debug::Logger.Format(__VA_ARGS__) + std::string("\n"));

#define FR_CORE_TRACE(...) Fykor::Debug::FykorLogger.Trace(Fykor::Debug::FykorLogger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_CORE_INFO(...) Fykor::Debug::FykorLogger.Info(Fykor::Debug::FykorLogger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_CORE_WARN(...) Fykor::Debug::FykorLogger.Warn(Fykor::Debug::FykorLogger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_CORE_ERROR(...) Fykor::Debug::FykorLogger.Error(Fykor::Debug::FykorLogger.Format(__VA_ARGS__) + std::string("\n"));
#define FR_CORE_FATAL(...) Fykor::Debug::FykorLogger.Fatal(Fykor::Debug::FykorLogger.Format(__VA_ARGS__) + std::string("\n"));

}