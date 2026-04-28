/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Log.h
 * ─────────────────────────────────────────────────
 * Debug logging
 * =================================================
 */

#pragma once

#include <FrameLog/FrameLog.h>

namespace ZeroEngine::Debug {
inline FrameLog::Logger ZeroEngineLogger("ZeroEngine");
inline FrameLog::Logger Logger("App");

#define ZE_TRACE(...) ZeroEngine::Debug::Logger.TraceLine(__VA_ARGS__);
#define ZE_INFO(...) ZeroEngine::Debug::Logger.InfoLine(__VA_ARGS__);
#define ZE_WARN(...) ZeroEngine::Debug::Logger.WarnLine(__VA_ARGS__);
#define ZE_ERROR(...) ZeroEngine::Debug::Logger.ErrorLine(__VA_ARGS__);
#define ZE_FATAL(...) ZeroEngine::Debug::Logger.FatalLine(__VA_ARGS__);

#define ZE_CORE_TRACE(...) ZeroEngine::Debug::ZeroEngineLogger.TraceLine(__VA_ARGS__);
#define ZE_CORE_INFO(...) ZeroEngine::Debug::ZeroEngineLogger.InfoLine(__VA_ARGS__);
#define ZE_CORE_WARN(...) ZeroEngine::Debug::ZeroEngineLogger.WarnLine(__VA_ARGS__);
#define ZE_CORE_ERROR(...) ZeroEngine::Debug::ZeroEngineLogger.ErrorLine(__VA_ARGS__);
#define ZE_CORE_FATAL(...) ZeroEngine::Debug::ZeroEngineLogger.FatalLine(__VA_ARGS__);

} // namespace ZeroEngine::Debug
