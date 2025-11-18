#pragma once

// ---------- Platform detection ----------
#ifdef _WIN32
    #define FK_PLATFORM_WINDOWS
#elif __linux__
    #define FK_PLATFORM_LINUX
#elif __APPLE__
    #error Apple not support
#else
    #error Unknown platform
#endif

// ---------- DLL / .SO ----------
#ifdef FK_PLATFORM_WINDOWS
    #ifdef FK_BUILD_DLL
        #define FYKOR_API __declspec(dllexport)
    #else
        #define FYKOR_API __declspec(dllimport)
    #endif
#elif defined(FK_PLATFORM_LINUX)
    #ifdef FK_BUILD_SO
        #define FYKOR_API __attribute__((visibility("default")))
    #else
        #define FYKOR_API
    #endif
#endif