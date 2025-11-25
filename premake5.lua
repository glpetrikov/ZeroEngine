workspace "Fykor"
    architecture "x64"
    location "build"
    startproject "Sandbox"

    configurations{
        "Debug",
        "Release"
    }

    vendor = {}
    vendor["GLFW"] = "vendor/GLFW/include/"
    vendor["FrameLog"] = "vendor/FrameLog/source/"

include "vendor/premake5.lua"

project "Fykor"
    location "build"
    kind "SharedLib"
    language "C++"
    cppdialect "C++20"
    staticruntime "Off"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files{
        "Fykor/**.h", 
        "Fykor/**.hpp", 
        "Fykor/**.cpp"
    }

    includedirs{
        "Fykor/",
        "%{vendor.FrameLog}",
        "%{vendor.GLFW}"
    }

    libdirs {
        "vendor/build/%{cfg.buildcfg}"
    }

    links{
        "FrameLog",
        "GLFW",
    }

    filter "system:windows"
        cppdialect "C++20"
        staticruntime "On"
        systemversion "latest"

        defines{
            "FR_BUILD_DLL"
        }

    filter "system:linux"
        cppdialect "C++20"
        links {"GL", "X11", "pthread", "dl" }
        defines {
            "FR_BUILD_SO"
        }

    filter "configurations:Debug"
        defines "FR_DEBUG"
        symbols "On"
    filter "configurations:Release"
        defines "FR_RELEASE"
        symbols "Off"
        optimize "On"


project "Sandbox"
    location "build"
    kind "ConsoleApp"
    language "C++"
    cppdialect "C++20"
    staticruntime "Off"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files{
        "Sandbox/source/**.h", 
        "Sandbox/source/**.hpp", 
        "Sandbox/source/**.cpp"
    }

    includedirs{
        "Fykor/",
        "%{vendor.FrameLog}",
        "%{vendor.GLFW}"
    }

    libdirs {
        "vendor/build/%{cfg.buildcfg}"
    }

    links{
        "Fykor",
        "FrameLog",
    }