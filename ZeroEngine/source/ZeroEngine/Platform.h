/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Platform.h
 * ─────────────────────────────────────────────────
 * Platform detection and DLL/SO definition
 * =================================================
 */

#pragma once

// ---------- Platform detection ----------
#ifdef _WIN32
#define FR_PLATFORM_WINDOWS
#elif __linux__
#define FR_PLATFORM_LINUX
#elif __APPLE__
#define FR_PLATFORM_MACOS
#else
#error Unknown platform
#endif

// ---------- DLL / .SO ----------
#ifdef FR_BUILD_DINAMIC
#ifdef FR_PLATFORM_WINDOWS
#ifdef FR_BUILD_DLL
#define ZE_API __declspec(dllexport)
#else
#define ZE_API __declspec(dllimport)
#endif
#elif defined(FR_PLATFORM_LINUX)
#ifdef FR_BUILD_SO
#define ZE_API __attribute__((visibility("default")))
#else
#define ZE_API
#endif
#elif defined(FR_PLATFORM_MACOS)
#ifdef FR_BUILD_SO
#define ZE_API __attribute__((visibility("default")))
#else
#define ZE_API
#endif
#endif
#endif

#define ZE_API
