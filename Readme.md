# Fykor Engine
or just Fykor.
Fykor is C++ Game Engine

## info
![Platforms](https://img.shields.io/badge/platforms-cross--platform-lightgrey.svg)
![Language](https://img.shields.io/badge/language-C%2B%2B-blue.svg)
![Status](https://img.shields.io/badge/status-closed.alpha-orange.svg)

[![License](https://img.shields.io/badge/License-Apache2.0-blue.svg)](LICENSE)
[![C++](https://img.shields.io/badge/C%2B%2B-20%2B-blue.svg)]()


![GitHub Repo stars](https://img.shields.io/github/stars/glpetrikov/FykorEngine?style=social)
![GitHub forks](https://img.shields.io/github/forks/glpetrikov/FykorEngine?style=social)
![GitHub issues](https://img.shields.io/github/issues/glpetrikov/FykorEngine)

## features
- Cross-platform Windowing
The window is created via GLFW and runs on Linux/Windows/macOS.

- Event System (basic)
Basic events: resize, close, key/mouse input.
(If you have them, add them; if you're unsure, I'll help.)

- LayerStack Architecture
Minimal layer system: you can push layers and overlays, call OnUpdate() and OnImGuiRender().

- ImGui Integration
It has its own ImGuiLayer, which renders on top of everything. Convenient for debugging.

- Logging System via FrameLog
A beautifully structured logger as a separate submodule.

- Premake5 Build System
Project generation for Linux/Windows/Mac. Easy to extend.

- Vendor modularity
Vendors are included as submodules (GLFW, FrameLog, ImGui).
Easy to update and maintain.

## License
FrameLog is distributed under the **Apache 2.0 License**.  
See [LICENSE](LICENSE) for details.

## Supported Platforms
- Windows11
- Fedora
- Ubuntu
- MacOS
