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

project "FrameLog"
    location "vendor/FrameLog"
    kind "SharedLib"
    language "C++"
    cppdialect "C++20"

    targetdir("vendor/FrameLog/source/build/%{cfg.buildcfg}")
    objdir("vendor/FrameLog/source/build/obj/%{cfg.buildcfg}")

    files {
        "vendor/FrameLog/source/**.h",
        "vendor/FrameLog/source/**.hpp",
        "vendor/FrameLog/source/**.cpp"
    }


project "Fykor"
    location ""
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
        "vendor/FrameLog/source/",
    }

    libdirs {
        "vendor/GLFW/build/src"
    }

    links{
        "FrameLog",
        "glfw",
        "dl",
        "X11",
        "pthread",
        "GL"
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
        staticruntime "On"
        systemversion "latest"

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
        "vendor/FrameLog/source/",
    }

    libdirs {
        "vendor/GLFW/build/src"
    }

    links{
        "Fykor",
        "FrameLog",
        "glfw",
        "dl",
        "X11",
        "pthread",
        "GL"
    }