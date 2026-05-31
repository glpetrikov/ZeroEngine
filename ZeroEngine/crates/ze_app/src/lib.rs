use std::{collections::HashMap, sync::Arc, time::Instant};

use winit::{
	application::ApplicationHandler,
	event::{MouseScrollDelta, WindowEvent},
	event_loop::ActiveEventLoop,
	keyboard::{KeyCode, PhysicalKey},
	window::{Window, WindowId},
};
use ze_core::{ResourceManager, Result, bail};
use ze_ecs::{Scene, System, registry};
use ze_input::{Input, ZKeyCode, ZMouseCode};
use ze_physics::PhysicsSystem;
use ze_renderer::{EditorCameraSystem, RenderSystem, register_renderer_components};
use ze_scripting_cs::{ScriptingSystem, register_scripting_components};

const DEFAULT_SCENE_NAME: &str = "main";

#[derive(Debug)]
pub enum CustomEvents {
	Shutdown,
}

pub struct App {
	runtime: tokio::runtime::Runtime,
	pub window: Option<Arc<Window>>,
	renderer: Option<ze_renderer::Renderer>,
	focused: bool,
	occluded: bool,
	minimized: bool,
	scenes: HashMap<String, Scene>,
	active_scene: String,
	last_frame_time: Instant,
	resources: ResourceManager,
}
impl Default for App {
	fn default() -> Self { Self::new().expect("failed to initialize app") }
}

impl App {
	pub fn new() -> Result<Self> {
		let resources = ResourceManager::for_runtime("assets");
		let scene = load_main_scene(&resources)?;

		if let Err(error) = resources.compile_game_shaders() {
			ze_log::error!("Failed to compile game shaders: {error:?}");
		}

		let active_scene = scene.name.clone();
		let mut scenes = HashMap::new();
		scenes.insert(scene.name.clone(), scene);

		let runtime = match tokio::runtime::Runtime::new() {
			Ok(r) => r,
			Err(e) => {
				ze_log::error!("Cannon create tokio runtime! error: {}", e);
				std::process::exit(1);
			}
		};

		Ok(Self {
			runtime,
			window: None,
			renderer: None,
			focused: true,
			occluded: false,
			minimized: false,
			scenes,
			active_scene,
			last_frame_time: Instant::now(),
			resources,
		})
	}

	pub fn add_scene(&mut self, scene: Scene) -> Result<()> {
		let name = scene.name.clone();
		if self.scenes.contains_key(&name) {
			bail!("scene `{name}` already exists");
		}

		self.scenes.insert(name, scene);
		Ok(())
	}

	pub fn set_active_scene(&mut self, name: impl Into<String>) -> Result<()> {
		let name = name.into();
		if !self.scenes.contains_key(&name) {
			bail!("scene `{name}` does not exist");
		}

		self.active_scene = name;
		Ok(())
	}

	pub fn active_scene(&self) -> Option<&Scene> { self.scenes.get(&self.active_scene) }

	pub fn active_scene_mut(&mut self) -> Option<&mut Scene> { self.scenes.get_mut(&self.active_scene) }

	pub fn add_system<S>(&mut self, system: S)
	where
		S: System + 'static,
	{
		let Some(scene) = self.active_scene_mut() else {
			return;
		};

		scene.add_system(system);
	}

	pub fn update_active_scene_systems(&mut self, dt: f32) -> Result<()> {
		let Some(scene) = self.active_scene_mut() else {
			return Ok(());
		};

		scene.update_systems(dt)
	}
}

pub fn load_main_scene(resources: &ResourceManager) -> Result<Scene> {
	let mut registry = registry::ComponentRegistry::new();

	Scene::register_defaults(&mut registry);
	register_renderer_components(&mut registry);
	register_scripting_components(&mut registry);

	let mut scene = Scene::from_path_with_registry(
		&resources
			.game_assets_root()
			.join(format!("scenes/{DEFAULT_SCENE_NAME}.zescene.json")),
		registry,
	)
	.map_err(|error| {
		ze_core::anyhow!(
			"failed to load main scene `{DEFAULT_SCENE_NAME}` from assets/scenes/{DEFAULT_SCENE_NAME}.zescene.json: {error:?}"
		)
	})?;
	scene.add_system(EditorCameraSystem::new());
	let scripting_system = ScriptingSystem::new()?;
	let scripting_runtime = scripting_system.runtime();
	scene.add_system(scripting_system);
	scene.add_system(PhysicsSystem::with_scripting(scripting_runtime));
	scene.add_system(RenderSystem::new());
	Ok(scene)
}

impl ApplicationHandler<CustomEvents> for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		ze_log::trace!("App resumed");

		let attrs = Window::default_attributes()
			.with_title("ZeroEngine")
			.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

		// TODO: add asset manager and icon

		let window = match event_loop.create_window(attrs) {
			Ok(window) => Arc::new(window),
			Err(error) => {
				ze_log::error!("Failed to create window: {error}");
				event_loop.exit();
				return;
			}
		};

		let _ = window
			.set_cursor_grab(winit::window::CursorGrabMode::Locked)
			.or_else(|_| window.set_cursor_grab(winit::window::CursorGrabMode::Confined));

		window.set_cursor_visible(false);

		let renderer = self.runtime.block_on(ze_renderer::Renderer::new(window.clone()));

		match renderer {
			Ok(renderer) => {
				self.renderer = Some(renderer);
			}
			Err(error) => {
				ze_log::error!("Failed to create renderer: {error:?}");
				event_loop.exit();
				return;
			}
		}

		self.window = Some(window);

		self.last_frame_time = Instant::now();
	}
	fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
		ze_log::trace!("App update");

		event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

		let now = Instant::now();
		let dt = now.duration_since(self.last_frame_time).as_secs_f32().min(0.05);
		self.last_frame_time = now;

		if Input::is_key_just_pressed(ZKeyCode::Escape) {
			ze_log::info!("Exiting...");
			event_loop.exit(); // TODO: TEMP
		}

		let window = self.window.clone();
		if let Some(window) = window
			&& !self.occluded
			&& !self.minimized
		{
			if let Err(error) = self.update_active_scene_systems(dt) {
				ze_log::error!("System update failed: {error:?}");
			}
			window.request_redraw();
		}
		Input::update_globally(Input::late_update);
	}

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvents) {
		match event {
			CustomEvents::Shutdown => {
				event_loop.exit();
			}
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		ze_log::trace!("window event");
		match event {
			// === WINDOW STATE =============
			WindowEvent::Resized(size) => {
				self.minimized = size.width == 0 || size.height == 0;

				if let Some(renderer) = &mut self.renderer
					&& !self.minimized
				{
					renderer.resize(size);
				}
			}
			WindowEvent::Focused(focused) => {
				self.focused = focused;

				if !focused {
					Input::update_globally(Input::reset);
				}
			}
			WindowEvent::Occluded(occluded) => {
				self.occluded = occluded;
			}
			WindowEvent::CloseRequested => {
				ze_log::debug!("Exiting...");
				event_loop.exit();
			}
			// === INPUT ==============
			WindowEvent::KeyboardInput { event, .. } if event.physical_key == PhysicalKey::Code(KeyCode::Escape) => {
				event_loop.exit();
			}
			WindowEvent::KeyboardInput { event: key_event, .. } => {
				if let PhysicalKey::Code(code) = key_event.physical_key {
					Input::update_globally(|i| {
						i.set_key(ZKeyCode::from(code), key_event.state.is_pressed());
					});
				}
			}
			WindowEvent::MouseInput { state, button, .. } => {
				Input::update_globally(|i| {
					i.set_mouse_button(ZMouseCode::from(button), state.is_pressed());
				});
			}
			WindowEvent::MouseWheel { delta, .. } => {
				let wheel_delta = match delta {
					MouseScrollDelta::LineDelta(_, y) => y,
					MouseScrollDelta::PixelDelta(position) => position.y as f32 / 120.0,
				};
				Input::update_globally(|i| {
					i.add_mouse_wheel_delta(wheel_delta);
				});
			}
			// === RENDER ==============
			WindowEvent::RedrawRequested => {
				ze_log::trace!("RedrawRequested");

				if let Some(renderer) = &mut self.renderer {
					let Some(scene) = self.scenes.get_mut(&self.active_scene) else {
						return;
					};

					scene.with_system_mut::<RenderSystem, _>(|render_system, scene| {
						render_system.render(scene, renderer, &self.resources);
					});

					// let Some(render_result) = render_result else {
					// 	ze_log::warn!("No RenderSystem found in scene `{}`",
					// scene.name); 	return;
					// };

					// if let Err(error) = render_result {
					// 	ze_log::error!("Render system error: {error:?}");
					// }
				}
			}
			_ => {}
		}
	}

	fn device_event(
		&mut self,
		_event_loop: &ActiveEventLoop,
		_device_id: winit::event::DeviceId,
		event: winit::event::DeviceEvent,
	) {
		if let winit::event::DeviceEvent::MouseMotion { delta } = event {
			Input::update_globally(|input| {
				input.add_mouse_delta(delta.0 as f32, delta.1 as f32);
			});
		}
	}
}
