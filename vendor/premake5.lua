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
        "GLFW/src/**.h",
        "GLFW/src/**.hpp",
        "GLFW/src/**.cpp",
        "GLFW/src/**.c"
    }

    filter "system:linux"
        defines {
            "_GLFW_X11"
        }

