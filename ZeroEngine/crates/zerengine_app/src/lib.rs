use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	keyboard::PhysicalKey,
	window::{Window, WindowId},
};
use zerengine_core::*;

impl ApplicationHandler for App {
	#[profile_function]
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		zerengine_log::debug!("App resumed");
		let attrs = Window::default_attributes()
			.with_title("ZeroEngine")
			.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

		self.window = Some(event_loop.create_window(attrs).unwrap());
	}

	#[profile_function]
	fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
		zerengine_log::debug!("App update");

		profile_new_frame!();
		event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

		if Input::key_just_pressed(ZKeyCode::Escape) {
			zerengine_log::info!("Exiting...");
			event_loop.exit(); // TODO: TEMP
		}

		if let Some(window) = &self.window {
			window.request_redraw();
		}
		Input::update_globally(|i| i.late_update());
	}

	// fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvent) {
	// 	match event {
	// 		CustomEvent::Timer => {
	// 			if let Some(window) = &self.window {
	// 				window.request_redraw();
	// 			}
	// 		}
	// 	}
	// }

	#[profile_function]
	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		zerengine_log::debug!("window event");
		match event {
			WindowEvent::CloseRequested => {
				zerengine_log::debug!("Exiting...");
				event_loop.exit();
			}
			WindowEvent::KeyboardInput { event: key_event, .. } => {
				if let PhysicalKey::Code(code) = key_event.physical_key {
					Input::update_globally(|i| {
						i.set_key(ZKeyCode::from(code), key_event.state.is_pressed());
					});
				}
			}
			WindowEvent::CursorMoved { position, .. } => {
				Input::update_globally(|i| {
					i.mouse_pos = (position.x as f32, position.y as f32);
				});
			}
			WindowEvent::MouseInput { state, button, .. } => {
				Input::update_globally(|i| {
					i.set_mouse_button(ZMouseCode::from(button), state.is_pressed());
				});
			}
			WindowEvent::RedrawRequested => {
				zerengine_log::debug!("RedrawRequested");

				// wgpu render here
			}
			_ => {}
		}
	}
}

#[derive(Default)]
pub struct App {
	pub window: Option<Window>,
}
