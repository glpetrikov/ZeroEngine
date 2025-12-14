/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Core.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Engine Core
 * =================================================
 */

#pragma once

#include "Platform.h"

#include "Common.h"

#ifdef FR_PLATFORM_WINDOWS
#	define FR_DEBUGBREAK() __debugbreak()
#else
#	define FR_DEBUGBREAK() __builtin_trap()
#endif

#ifdef FR_ENABLE_ASSERTS
#	define FR_ASSERT(x, ...)                                                                                          \
		if (!(x))                                                                                                      \
		{                                                                                                              \
			FykorLogger.Error(FykorLogger.Format("Assertion Failed: {0}", __VA_ARGS__));                               \
			FR_DEBUGBREAK();                                                                                           \
		}

#	define FR_CORE_ASSERT(x, ...)                                                                                     \
		if (!(x))                                                                                                      \
		{                                                                                                              \
			FykorLogger.Error(FykorLogger.Format("Assertion Failed: {0}", __VA_ARGS__));                               \
			FR_DEBUGBREAK();                                                                                           \
		}
#else
#	define FR_ASSERT(x, ...)
#	define FR_CORE_ASSERT(x, ...)
#endif

#define BIT(x) (1 << x)

#define HZ_BIND_EVENT_FN(fn) std::bind(&fn, this, std::placeholders::_1)

namespace Fykor::Core
{

}
