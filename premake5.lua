workspace "Fykor"
    architecture "x64"
    location "build"
    startproject "Sandbox"

    configurations{
        "Debug",
        "Release"
    }

    vendor = {}
    vendor["GLFW"] = "Fykor/vendor/GLFW"
    vendor["FrameLog"] = "Fykor/vendor/FrameLog/include"
    vendor["Glad"] = "Fykor/vendor/Glad"
    vendor["ImGui"] = "Fykor/vendor/ImGui"
    vendor["glm"] = "Fykor/vendor/glm"

include "Fykor/vendor/premake5.lua"

project "Fykor"
    location "Fykor"
    kind "StaticLib"
    language "C++"
    cppdialect "C++20"
    staticruntime "Off"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files {
        "Fykor/source/**.h",
        "Fykor/source/**.hpp",
        "Fykor/source/**.cpp"
    }

    includedirs{
        "Fykor/source/",
        "Fykor/source/Fykor/",
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
    links {
            "Glad",
            "glfw",
            "ImGui",
            "GL",
            "X11",
            "pthread",
            "dl"
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
        "Sandbox/source/**.h", 
        "Sandbox/source/**.hpp", 
        "Sandbox/source/**.cpp"
    }

    includedirs{
        "Fykor/source/",
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
        "Fykor",
        "FrameLog",
        "GLFW",
        "Glad",
        "ImGui",
    }

    filter "system:linux"
        links { "GL", "X11", "pthread", "dl" }
    filter "system:windows"
        links { "Glad", "GLFW", "opengl32", "gdi32", "user32", "shell32" }

    filter "configurations:Debug"
        defines "FR_DEBUG"
        runtime "Debug"
        symbols "On"
    filter "configurations:Release"
        defines "FR_RELEASE"
        runtime "Release"
        symbols "Off"
        optimize "on"
