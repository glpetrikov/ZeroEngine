workspace "ZeroEngine"
    architecture "x64"
    location "build"
    startproject "Sandbox"

    configurations{
        "Debug",
        "Release"
    }

    vendor = {}
    vendor["GLFW"] = "ZeroEngine/vendor/GLFW"
    vendor["FrameLog"] = "ZeroEngine/vendor/FrameLog/include"
    vendor["Glad"] = "ZeroEngine/vendor/Glad"
    vendor["ImGui"] = "ZeroEngine/vendor/ImGui"
    vendor["glm"] = "ZeroEngine/vendor/glm"

include "ZeroEngine/vendor/premake5.lua"

project "ZeroEngine"
    location "ZeroEngine"
    kind "StaticLib"
    language "C++"
    cppdialect "C++20"
    staticruntime "Off"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files {
        "ZeroEngine/source/**.cpp",
    }

    includedirs{
        "ZeroEngine/source/",
        "ZeroEngine/source/ZeroEngine/",
        "%{vendor.FrameLog}",
        "%{vendor.GLFW}/include",
        "%{vendor.Glad}/include",
        "%{vendor.ImGui}",
        "%{vendor.ImGui}/backends",
        "%{vendor.glm}",
    }

    defines {
        "GLFW_INCLUDE_NONE"
    }

    libdirs {
        "vendor/build/%{cfg.buildcfg}"
    }

    links{
        "FrameLog",
        "GLFW",
        "Glad",
        "ImGui"
    }

    filter "system:windows"
        cppdialect "C++20"
        staticruntime "on"
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
        runtime "Debug"
        symbols "On"
    filter "configurations:Release"
        defines "FR_RELEASE"
        runtime "Release"
        symbols "Off"
        optimize "on"


project "Sandbox"
    location "Sandbox"
    kind "ConsoleApp"
    language "C++"
    cppdialect "C++20"
    staticruntime "Off"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files{
        "Sandbox/source/**.hpp",
        "Sandbox/source/**.cpp"
    }

    includedirs{
        "Sandbox/source/",
        "ZeroEngine/source/",
        "%{vendor.FrameLog}",
        "%{vendor.GLFW}/include",
        "%{vendor.ImGui}",
        "%{vendor.ImGui}/backends",
        "%{vendor.glm}",
    }

    libdirs {
        "vendor/build/%{cfg.buildcfg}"
    }

    links {
        "ZeroEngine",
        "FrameLog",
        "GLFW",
        "Glad",
        "ImGui",
    }

    filter "system:linux"
        links { "GL", "X11", "pthread", "dl" }
    filter "system:windows"
        links { "opengl32", "gdi32", "user32", "shell32" }

    filter "configurations:Debug"
        defines "FR_DEBUG"
        runtime "Debug"
        symbols "On"
    filter "configurations:Release"
        defines "FR_RELEASE"
        runtime "Release"
        symbols "Off"
        optimize "on"
