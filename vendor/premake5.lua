--==============================
-- FrameLog
--==============================
project "FrameLog"
    location "FrameLog"
    kind "SharedLib"
    language "C++"
    cppdialect "C++20"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files {
        "FrameLog/source/**.h",
        "FrameLog/source/**.hpp",
        "FrameLog/source/**.cpp"
    }

    includedirs {
        "FrameLog/source/"
    }
--==============================
-- GLFW
--==============================
project "GLFW"
    location "GLFW"
    kind "SharedLib"
    language "C"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

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
--==============================
-- Glad
--==============================
project "Glad"
    kind "StaticLib"
    language "C"
    pic "On"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

    files {
        "Glad/include/glad/glad.h",
        "Glad/include/KHR/khrplatform.h",
        "Glad/src/glad.c"
    }

    includedirs {
        "Glad/include"
    }
--==============================
-- ImGui
--==============================
project "ImGui"
    location "ImGui"
    kind "StaticLib"
    language "C++"
    pic "On"

    targetdir("build/%{cfg.buildcfg}")
    objdir("build/obj/%{cfg.buildcfg}")

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