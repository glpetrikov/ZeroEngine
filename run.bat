@echo off
REM Stop on error manually
setlocal enabledelayedexpansion

cd /d Fykor\vendor\ImGui || exit /b 1
git checkout docking || exit /b 1

cd ..\GLFW || exit /b 1
git checkout master || exit /b 1
git pull || exit /b 1

cd ..\FrameLog || exit /b 1
git checkout main || exit /b 1
git pull || exit /b 1

cd ..\GLFW || exit /b 1
git checkout master || exit /b 1
git pull || exit /b 1

cd ..\..\.. || exit /b 1

REM mkdir -p build
if not exist build mkdir build

premake5 gmake2 || exit /b 1

cd build || exit /b 1
make -j6 || exit /b 1

cd Debug || exit /b 1
echo Running...
Sandbox.exe

endlocal
