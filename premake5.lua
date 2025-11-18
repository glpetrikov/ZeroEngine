workspace "Fykor"
    architecture "x64"
    location "build"
    startproject "Sandbox"

    configurations{
        "Debug",
        "Release"
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
        "vendor/"
    }

    filter "system:windows"
        cppdialect "C++17"
        staticruntime "On"
        systemversion "latest"

        defines{
            "FK_BUILD_DLL"
        }

    filter "system:linux"
        cppdialect "C++17"
        staticruntime "On"
        systemversion "latest"

        defines {
            "FK_BUILD_SO"
        }

    filter "configurations:Debug"
        defines "FK_DEBUG"
        symbols "On"
    filter "configurations:Release"
        defines "FK_RELEASE"
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
        "vendor/"
    }

    links{
        "Fykor"
    }