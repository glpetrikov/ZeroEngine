use timetrace::*;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Debug)]
pub enum CustomEvents {
	Shutdown,
}

impl ApplicationHandler<CustomEvents> for App {
	#[profile_function]
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let attrs = Window::default_attributes()
			.with_title("ZeroEngine")
			.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

		self.window = Some(event_loop.create_window(attrs).unwrap());
	}

	#[profile_function]
	fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
		profile_new_frame!();
		event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

		if let Some(window) = &self.window {
			window.request_redraw();
		}
	}

	#[profile_function]
	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvents) {
		match event {
			CustomEvents::Shutdown => {
				event_loop.exit();
			}
		}
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
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			WindowEvent::KeyboardInput { event, .. } if event.physical_key == PhysicalKey::Code(KeyCode::Escape) => {
				event_loop.exit();
			}
			WindowEvent::KeyboardInput { .. } => {}
			WindowEvent::RedrawRequested => {
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
