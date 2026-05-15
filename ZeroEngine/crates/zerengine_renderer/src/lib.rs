mod backend;

use std::sync::Arc;

use crate::backend::pipeline::*;

pub struct Renderer {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	triangle_pipeline: Pipeline,
}

impl Renderer {
	pub async fn new(window: Arc<winit::window::Window>) -> zerengine_core::Result<Self> {
		let instance = wgpu::Instance::default();

		let surface = instance.create_surface(window.clone()).unwrap();

		let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
			power_preference: wgpu::PowerPreference::HighPerformance,
			compatible_surface: Some(&surface),
			force_fallback_adapter: false,
		};

		let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

		let info = adapter.get_info();

		zerengine_log::info!("Renderer: {} ({:?})", info.name, info.backend);

		let device_descriptor = wgpu::DeviceDescriptor {
			required_features: wgpu::Features::empty(),
			required_limits: wgpu::Limits::default(),
			label: Some("Device"),
			memory_hints: wgpu::MemoryHints::default(),
			trace: wgpu::Trace::Off,
			experimental_features: wgpu::ExperimentalFeatures::disabled(),
		};

		let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

		let size = window.inner_size();

		let surface_capabilities = surface.get_capabilities(&adapter);
		let surface_format = surface_capabilities
			.formats
			.iter()
			.copied()
			.filter(|f| f.is_srgb())
			.next()
			.unwrap_or(surface_capabilities.formats[0]);

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::AutoVsync,
			alpha_mode: surface_capabilities.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		};

		surface.configure(&device, &config);

		let triangle_pipeline = PipelineBuilder::new()
            .with_name("Triangle")
            .with_shader_source(include_str!("../assets/shaders/triangle.wgsl"))
            .with_pixel_format(surface_format)
            .build(&device)?;

		Ok(Self {
			surface,
			device,
			queue,
			config,
			size,
			triangle_pipeline,
		})
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn request_redraw(&mut self) {
		// TODO: add Result return type and handle Surface errors
		match self.render() {
			Ok(_) => {}
			Err(wgpu::SurfaceStatus::Lost) => {
				let size = self.size;
				self.resize(size);
			}
			Err(e) => zerengine_log::error!("Render error: {:?}", e),
		}
	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceStatus> {
		let drawable = self.surface.get_current_texture();
		let drawable = match drawable {
			wgpu::CurrentSurfaceTexture::Timeout => return Ok(()),
			wgpu::CurrentSurfaceTexture::Lost => return Ok(()),
			wgpu::CurrentSurfaceTexture::Success(t) => t,
			_ => todo!(),
		};

		let image_view_descriptor = wgpu::TextureViewDescriptor::default();
		let image_view = drawable.texture.create_view(&image_view_descriptor);

		let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		};
		let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

		let color_attachment = wgpu::RenderPassColorAttachment {
			view: &image_view,
			resolve_target: None,
			depth_slice: None,
			ops: wgpu::Operations {
				load: wgpu::LoadOp::Clear(wgpu::Color {
					r: 0.12,
					g: 0.12,
					b: 0.13,
					a: 1.0,
				}),
				store: wgpu::StoreOp::Store,
			},
		};

		let render_pass_descriptor = wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(color_attachment)],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
			multiview_mask: None,
		};

		let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
		render_pass.set_pipeline(&self.triangle_pipeline.render_pipeline);
		render_pass.draw(0..3, 0..1);
		drop(render_pass);
		self.queue.submit(std::iter::once(command_encoder.finish()));

		drawable.present();

		Ok(())
	}

	#[allow(dead_code)]
	pub fn render_sprite(/* &zerengine_ecs::Sprite, &zerengine_ecs::Transform */) {
		// TODO
		todo!()
	}
	#[allow(dead_code)]
	fn render_mesh(/* &zerengine_ecs::Mesh, &zerengine_ecs::Transform */) {
		// TODO
		todo!()
	}
}
