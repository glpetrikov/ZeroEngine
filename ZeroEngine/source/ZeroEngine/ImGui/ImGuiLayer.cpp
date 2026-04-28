/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * ImGuiLayer.cpp
 * ─────────────────────────────────────────────────
 * ImGuiLayer class implementation
 * =================================================
 */

#include "ImGuiLayer.h"

#include "App.h"
#include <backends/imgui_impl_glfw.h>
#include <backends/imgui_impl_opengl3.h>
#include <imgui.h>

#define IMGUI_IMPL_OPENGL_LOADER_GLAD

// TEMPORARY
#include "GLFW/glfw3.h"
#include <glad/glad.h>

namespace ZeroEngine {

static ImGuiKey GLFWKeyToImGuiKey(int glfwKey) {
	switch (glfwKey) {
	case GLFW_KEY_TAB:
		return ImGuiKey_Tab;
	case GLFW_KEY_LEFT:
		return ImGuiKey_LeftArrow;
	case GLFW_KEY_RIGHT:
		return ImGuiKey_RightArrow;
	case GLFW_KEY_UP:
		return ImGuiKey_UpArrow;
	case GLFW_KEY_DOWN:
		return ImGuiKey_DownArrow;
	case GLFW_KEY_PAGE_UP:
		return ImGuiKey_PageUp;
	case GLFW_KEY_PAGE_DOWN:
		return ImGuiKey_PageDown;
	case GLFW_KEY_HOME:
		return ImGuiKey_Home;
	case GLFW_KEY_END:
		return ImGuiKey_End;
	case GLFW_KEY_INSERT:
		return ImGuiKey_Insert;
	case GLFW_KEY_DELETE:
		return ImGuiKey_Delete;
	case GLFW_KEY_BACKSPACE:
		return ImGuiKey_Backspace;
	case GLFW_KEY_SPACE:
		return ImGuiKey_Space;
	case GLFW_KEY_ENTER:
		return ImGuiKey_Enter;
	case GLFW_KEY_ESCAPE:
		return ImGuiKey_Escape;
	case GLFW_KEY_LEFT_CONTROL:
		return ImGuiKey_LeftCtrl;
	case GLFW_KEY_LEFT_SHIFT:
		return ImGuiKey_LeftShift;
	case GLFW_KEY_LEFT_ALT:
		return ImGuiKey_LeftAlt;
	case GLFW_KEY_LEFT_SUPER:
		return ImGuiKey_LeftSuper;
	case GLFW_KEY_RIGHT_CONTROL:
		return ImGuiKey_RightCtrl;
	case GLFW_KEY_RIGHT_SHIFT:
		return ImGuiKey_RightShift;
	case GLFW_KEY_RIGHT_ALT:
		return ImGuiKey_RightAlt;
	case GLFW_KEY_RIGHT_SUPER:
		return ImGuiKey_RightSuper;
	case GLFW_KEY_CAPS_LOCK:
		return ImGuiKey_CapsLock;
	case GLFW_KEY_SCROLL_LOCK:
		return ImGuiKey_ScrollLock;
	case GLFW_KEY_NUM_LOCK:
		return ImGuiKey_NumLock;
	case GLFW_KEY_PRINT_SCREEN:
		return ImGuiKey_PrintScreen;
	case GLFW_KEY_PAUSE:
		return ImGuiKey_Pause;
	default:
		return ImGuiKey_None;
	}
}

ImGuiLayer::ImGuiLayer() : Layers::Layer("ImGuiLayer") {}

ImGuiLayer::~ImGuiLayer() = default;

void ImGuiLayer::OnAttach() {
	IMGUI_CHECKVERSION();
	ImGui::CreateContext();
	ImGuiIO& io = ImGui::GetIO();
	io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
	// io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;
	io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;
	io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable;

	ImGui::StyleColorsDark();
	// ImGui::StyleColorsClassic();

	ImGuiStyle& style = ImGui::GetStyle();
	if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable) {
		style.WindowRounding = 0.0f;
		style.Colors[ImGuiCol_WindowBg].w = 1.0f;
	}

	App& app = App::Get();
	GLFWwindow* window = static_cast<GLFWwindow*>(app.GetWindow().GetNativeWindow());

	ImGui_ImplGlfw_InitForOpenGL(window, true);
	ImGui_ImplOpenGL3_Init("#version 410");
}

void ImGuiLayer::OnDetach() {
	ImGui_ImplOpenGL3_Shutdown();
	ImGui_ImplGlfw_Shutdown();
	ImGui::DestroyContext();
}

void ImGuiLayer::Begin() {
	ImGui_ImplGlfw_NewFrame();
	ImGui_ImplOpenGL3_NewFrame();
	ImGui::NewFrame();
}

void ImGuiLayer::End() {
	ImGuiIO& io = ImGui::GetIO();
	App& app = App::Get();
	io.DisplaySize = ImVec2((float)app.GetWindow().GetWidth(), (float)app.GetWindow().GetHeight());
	ImGui::Render();
	ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
	if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable) {
		GLFWwindow* backup_current_context = static_cast<GLFWwindow*>(app.GetWindow().GetNativeWindow());
		ImGui::UpdatePlatformWindows();
		ImGui::RenderPlatformWindowsDefault();
		glfwMakeContextCurrent(backup_current_context);
	}
}

void ImGuiLayer::OnImGuiRender() {
	static bool show = true;
	ImGui::ShowDemoWindow(&show);
}
} // namespace ZeroEngine
