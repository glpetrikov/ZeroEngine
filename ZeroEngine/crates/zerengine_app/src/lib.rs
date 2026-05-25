use std::sync::Arc;

use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	keyboard::{KeyCode, PhysicalKey},
	window::{Window, WindowId},
};
use zerengine_core::Vec3;
use zerengine_input::*;
use zerengine_renderer::{World, model::game_object};

#[derive(Debug)]
pub enum CustomEvents {
	Shutdown,
}

pub struct App {
	runtime: tokio::runtime::Runtime,
	pub window: Option<Arc<Window>>,
	renderer: Option<zerengine_renderer::Renderer>,
	focused: bool,
	occluded: bool,
	minimized: bool,
	world: Option<World>,
}
impl Default for App {
	fn default() -> Self {
		Self {
			runtime: tokio::runtime::Runtime::new().unwrap(),
			window: None,
			renderer: None,
			focused: true,
			occluded: false,
			minimized: false,
			world: Some(World::new()),
		}
	}
}

impl ApplicationHandler<CustomEvents> for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		zerengine_log::trace!("App resumed");

		let attrs = Window::default_attributes()
			.with_title("ZeroEngine")
			.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

		// TODO: add asset manager and icon

		let window = match event_loop.create_window(attrs) {
			Ok(window) => Arc::new(window),
			Err(error) => {
				zerengine_log::error!("Failed to create window: {error}");
				event_loop.exit();
				return;
			}
		};

		let _ = window
			.set_cursor_grab(winit::window::CursorGrabMode::Locked)
			.or_else(|_| window.set_cursor_grab(winit::window::CursorGrabMode::Confined));

		window.set_cursor_visible(false);

		let renderer = self.runtime.block_on(zerengine_renderer::Renderer::new(window.clone()));

		match renderer {
			Ok(renderer) => {
				self.renderer = Some(renderer);
			}
			Err(error) => {
				zerengine_log::error!("Failed to create renderer: {error:?}");
				event_loop.exit();
				return;
			}
		}

		self.window = Some(window);

		self.world.as_mut().unwrap().quads.push(game_object::Object {
			position: Vec3::new(0.5, 0.0, -1.5),
			angle: 0.0,
			scale: Vec3::new(1.0, 1.0, 1.0),
		});

		self.world.as_mut().unwrap().triangles.push(game_object::Object {
			position: Vec3::new(0.0, 0.0, -1.0),
			angle: 0.0,
			scale: Vec3::new(1.0, 1.0, 1.0),
		});

		self.renderer.as_mut().unwrap().build_ubos_for_objects(2);
	}
	fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
		zerengine_log::trace!("App update");

		event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

		if Input::is_key_just_pressed(ZKeyCode::Escape) {
			zerengine_log::info!("Exiting...");
			event_loop.exit(); // TODO: TEMP
		}

		if let Some(window) = &self.window
			&& !self.occluded
			&& !self.minimized
		{
			self.world.as_mut().unwrap().update(0.017);
			window.request_redraw();
		}
		Input::update_globally(|i| i.late_update());
	}

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvents) {
		match event {
			CustomEvents::Shutdown => {
				event_loop.exit();
			}
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		zerengine_log::trace!("window event");
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
					Input::update_globally(|i| i.reset());
				}
			}
			WindowEvent::Occluded(occluded) => {
				self.occluded = occluded;
			}
			WindowEvent::CloseRequested => {
				zerengine_log::debug!("Exiting...");
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
			// === RENDER ==============
			WindowEvent::RedrawRequested => {
				zerengine_log::trace!("RedrawRequested");

				if let Some(renderer) = &mut self.renderer {
					renderer.request_redraw(self.world.as_ref().unwrap(), &self.world.as_ref().unwrap().camera);
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
