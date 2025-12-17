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

#include <FrameLog/FrameLog.h>
#include "Platform.h"

namespace Fykor::Debug
{
	inline FrameLog::Logger FykorLogger("Fykor");
	inline FrameLog::Logger Logger("App");

#define FR_TRACE(...) Fykor::Debug::Logger.TraceLine(__VA_ARGS__);
#define FR_INFO(...) Fykor::Debug::Logger.InfoLine(__VA_ARGS__);
#define FR_WARN(...) Fykor::Debug::Logger.WarnLine(__VA_ARGS__);
#define FR_ERROR(...) Fykor::Debug::Logger.ErrorLine(__VA_ARGS__);
#define FR_FATAL(...) Fykor::Debug::Logger.FatalLine(__VA_ARGS__);

#define FR_CORE_TRACE(...) Fykor::Debug::FykorLogger.TraceLine(__VA_ARGS__);
#define FR_CORE_INFO(...) Fykor::Debug::FykorLogger.InfoLine(__VA_ARGS__);
#define FR_CORE_WARN(...) Fykor::Debug::FykorLogger.WarnLine(__VA_ARGS__);
#define FR_CORE_ERROR(...) Fykor::Debug::FykorLogger.ErrorLine(__VA_ARGS__);
#define FR_CORE_FATAL(...) Fykor::Debug::FykorLogger.FatalLine(__VA_ARGS__);

} // namespace Fykor::Debug
