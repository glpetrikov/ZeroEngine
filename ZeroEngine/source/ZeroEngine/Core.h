/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Core.h
 * ─────────────────────────────────────────────────
 * Engine Core
 * =================================================
 */

#pragma once

#include "Platform.h" // IWYU pragma: export

#include "Common.h" // IWYU pragma: export

#ifdef ZE_PLATFORM_WINDOWS
#define ZE_DEBUGBREAK() __debugbreak()
#else
#define ZE_DEBUGBREAK() __builtin_trap()
#endif

#ifdef ZE_DEBUG
#define ZE_ENABLE_ASSERTS
#endif

#ifdef ZE_ENABLE_ASSERTS
#define ZE_ASSERT(x, ...)                                                                                              \
	if (!(x)) {                                                                                                        \
			ZeroEngine::Debug::ZeroEngineLogger.Error(("Assertion Failed: {0}", __VA_ARGS__);                               \
			ZE_DEBUGBREAK();                                                                                           \
	}

#define ZE_CORE_ASSERT(x, ...)                                                                                         \
	if (!(x)) {                                                                                                        \
		ZeroEngine::Debug::ZeroEngineLogger.Error("Assertion Failed: {0}", __VA_ARGS__);                               \
		ZE_DEBUGBREAK();                                                                                               \
	}
#else
#define ZE_ASSERT(x, ...)
#define ZE_CORE_ASSERT(x, ...)
#endif

#define BIT(x) (1 << x)

#define HZ_BIND_EVENT_FN(fn) std::bind(&fn, this, std::placeholders::_1)

namespace ZeroEngine::Core {}
