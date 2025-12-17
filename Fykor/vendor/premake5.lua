--==============================
-- FrameLog
--==============================
include "FrameLog/include/FrameLog/premake5.lua"
--==============================
-- GLFW
--==============================
project "GLFW"
    location "GLFW"
    kind "SharedLib"
    language "C"

    targetdir("../../build/%{cfg.buildcfg}")
    objdir("../../build/obj/%{cfg.buildcfg}")

    files {
        "GLFW/src/**.cpp",
        "GLFW/src/**.c"
}

    includedirs {
        "GLFW/include",
        "GLFW/"
    }

    filter "system:linux"
        defines {
            "_GLFW_X11"
        }
    filter "configurations:Debug"
        runtime "Debug"
        symbols "On"
    filter "configurations:Release"
        runtime "Release"
        symbols "Off"
        optimize "on"
--==============================
-- Glad
--==============================
project "Glad"
    kind "StaticLib"
    language "C"
    pic "on"

    targetdir("../../build/%{cfg.buildcfg}")
    objdir("../../build/obj/%{cfg.buildcfg}")

    files {
        "Glad/include/glad/glad.h",
        "Glad/include/KHR/khrplatform.h",
        "Glad/src/glad.c"
    }

    includedirs {
        "Glad/include"
    }

    filter "configurations:Debug"
        runtime "Debug"
        symbols "On"
    filter "configurations:Release"
        runtime "Release"
        symbols "Off"
        optimize "on"
--==============================
-- ImGui
--==============================
project "ImGui"
    location "ImGui"
    kind "StaticLib"
    language "C++"
    pic "on"

    targetdir("../../build/%{cfg.buildcfg}")
    objdir("../../build/obj/%{cfg.buildcfg}")

    files {
        "ImGui/imgui.cpp",
        "ImGui/imgui_draw.cpp",
        "ImGui/imgui_tables.cpp",
        "ImGui/imgui_widgets.cpp",
        "ImGui/imgui_demo.cpp",

        "ImGui/backends/imgui_impl_glfw.cpp",
        "ImGui/backends/imgui_impl_opengl3.cpp",
    }

    includedirs {
        "ImGui",
        "ImGui/backends",
        "GLFW//include",
        "Glad/include"
    }

    filter "configurations:Debug"
        runtime "Debug"
        symbols "On"
    filter "configurations:Release"
        runtime "Release"
        symbols "Off"
        optimize "on"
