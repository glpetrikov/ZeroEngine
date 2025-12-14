/* =================================================
 * Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Platform.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Define Platform
 * =================================================
 */

#pragma once

// ---------- Platform detection ----------
#ifdef _WIN32
#	define FR_PLATFORM_WINDOWS
#elif __linux__
#	define FR_PLATFORM_LINUX
#elif __APPLE__
#	define FR_PLATFORM_MACOS
#else
#	error Unknown platform
#endif

// ---------- DLL / .SO ----------
#ifdef FR_PLATFORM_WINDOWS
#	ifdef FR_BUILD_DLL
#		define FYKOR_API __declspec(dllexport)
#	else
#		define FYKOR_API __declspec(dllimport)
#	endif
#elif defined(FR_PLATFORM_LINUX)
#	ifdef FR_BUILD_SO
#		define FYKOR_API __attribute__((visibility("default")))
#	else
#		define FYKOR_API
#	endif
#elif defined(FR_PLATFORM_MACOS)
#	ifdef FR_BUILD_SO
#		define FYKOR_API __attribute__((visibility("default")))
#	else
#		define FYKOR_API
#	endif
#endif
