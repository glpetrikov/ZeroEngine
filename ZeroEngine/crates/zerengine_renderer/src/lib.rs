mod backend;
pub mod model;

use std::sync::Arc;

use glam::{Mat4, Quat};

use crate::{
	backend::{
		mesh::*,
		pipeline::{self, *},
		texture::Texture,
		ubo::*,
	},
	model::game_object::{self, Camera, Object},
};

// TODO: Temp!
#[derive(Default)]
pub struct World {
	pub quads: Vec<game_object::Object>,
	pub triangles: Vec<game_object::Object>,
	pub camera: Camera,
}

impl World {
	pub fn new() -> Self {
		Self {
			quads: Vec::new(),
			triangles: Vec::new(),
			camera: Camera::new(),
		}
	}

	pub fn update(&mut self, dt: f32) {
		for triangle in &mut self.triangles {
			triangle.angle += 0.5 * dt;

			if triangle.angle > std::f32::consts::TAU {
				triangle.angle -= std::f32::consts::TAU;
			}
		}

		let mut d_right = 0.0;
		let mut d_forwards = 0.0;

		if zerengine_input::Input::is_key_pressed(zerengine_input::ZKeyCode::W) {
			d_forwards += 1.0;
		}
		if zerengine_input::Input::is_key_pressed(zerengine_input::ZKeyCode::S) {
			d_forwards -= 1.0;
		}
		if zerengine_input::Input::is_key_pressed(zerengine_input::ZKeyCode::D) {
			d_right += 1.0;
		}
		if zerengine_input::Input::is_key_pressed(zerengine_input::ZKeyCode::A) {
			d_right -= 1.0;
		}

		let speed = if zerengine_input::Input::is_key_pressed(zerengine_input::ZKeyCode::LShift) {
			10.0
		} else {
			5.0
		};

		self.camera.movec(d_right * speed * dt, d_forwards * speed * dt);

		let mouse_delta = zerengine_input::Input::get_mouse_delta();
		let sensitivity = 0.1;

		self.camera
			.spin(-mouse_delta.x * sensitivity, -mouse_delta.y * sensitivity);
	}
}

pub struct Renderer {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	pipeline: Pipeline,
	triangle_mesh: Mesh,
	quad_mesh: Mesh,
	triangle_material: Texture,
	quad_material: Texture,
	ubo: Option<UboGroup>,
	projection_ubo: Option<Ubo>,
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
			required_features: wgpu::Features::POLYGON_MODE_LINE,
			required_limits: wgpu::Limits::default(),
			label: Some("Device"),
			memory_hints: wgpu::MemoryHints::Performance,
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
			.find(|f| f.is_srgb())
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

		let triangle_mesh = Vertex::make_triangle(&device);
		let quad_mesh = Vertex::make_quad(&device);

		let material_bind_group_layout: wgpu::BindGroupLayout;
		{
			let mut builder = backend::bind_group_layout::Builder::new(&device);
			builder.add_material();
			material_bind_group_layout = builder.build("Material Bind Group Layout");
		}

		let ubo_bind_group_layout: wgpu::BindGroupLayout;
		{
			let mut builder = backend::bind_group_layout::Builder::new(&device);
			builder.add_ubo();
			ubo_bind_group_layout = builder.build("UBO Bind Group Layout");
		}

		let render_pipeline: Pipeline;
		{
			let pipeline_builder = pipeline::Builder::new(&device)
				.with_shader_source(wesl::include_wesl!("shader"))
				.with_pixel_format(surface_format)
				.with_vertex_buffer_layout(Vertex::get_layout())
				.with_bind_group_layout(&material_bind_group_layout)
				.with_bind_group_layout(&ubo_bind_group_layout)
				.with_bind_group_layout(&ubo_bind_group_layout);

			render_pipeline = pipeline_builder.build()?;
		}

		let triangle_material = Texture::new("CheckerBoard.png", &device, &queue, &material_bind_group_layout);
		let quad_material = Texture::new("CheckerBoard.png", &device, &queue, &material_bind_group_layout);

		let projection_ubo = Some(Ubo::new(&device, &ubo_bind_group_layout));

		Ok(Self {
			surface,
			device,
			queue,
			config,
			size,
			pipeline: render_pipeline,
			triangle_mesh,
			triangle_material,
			quad_mesh,
			quad_material,
			ubo: None,
			projection_ubo,
		})
	}

	pub fn build_ubos_for_objects(&mut self, objects_count: usize) {
		let ubo_bind_group_layout: wgpu::BindGroupLayout;
		{
			let mut builder = backend::bind_group_layout::Builder::new(&self.device);
			builder.add_ubo();
			ubo_bind_group_layout = builder.build("UBO Bind Group Layout");
		}

		self.ubo = Some(UboGroup::new(&self.device, objects_count, &ubo_bind_group_layout))
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn request_redraw(&mut self, world: &World, camera: &Camera) {
		// TODO: add Result return type and handle Surface errors
		match self.render(&world.quads, &world.triangles, camera) {
			Ok(_) => {}
			Err(wgpu::SurfaceStatus::Lost) => {
				let size = self.size;
				self.resize(size);
			}
			Err(e) => zerengine_log::error!("Render error: {:?}", e),
		}
	}

	fn update_projection(&mut self, camera: &Camera) {
		let view = Mat4::look_to_rh(camera.position, camera.forwards, camera.up);

		let fov_y = 65.0_f32.to_radians();
		let aspect = self.size.width as f32 / self.size.height as f32;
		let z_near = 0.05;
		let z_far = 1000.0;
		let projection = Mat4::perspective_rh(fov_y, aspect, z_near, z_far);
		let view_proj = projection * view;
		self.projection_ubo.as_mut().unwrap().upload(&view_proj, &self.queue);
	}

	fn update_transforms(&mut self, quads: &[Object], triangles: &[Object]) {
		let mut offset: u64 = 0;
		for (i, quad) in quads.iter().enumerate() {
			let matrix =
				Mat4::from_scale_rotation_translation(quad.scale, Quat::from_rotation_z(quad.angle), quad.position);

			self.ubo
				.as_mut()
				.unwrap()
				.upload(offset + i as u64, &matrix, &self.queue);
		}
		offset += quads.len() as u64;
		for (i, triangle) in triangles.iter().enumerate() {
			let matrix = Mat4::from_scale_rotation_translation(
				triangle.scale,
				Quat::from_rotation_z(triangle.angle),
				triangle.position,
			);

			self.ubo
				.as_mut()
				.unwrap()
				.upload(offset + i as u64, &matrix, &self.queue);
		}
	}

	fn render(&mut self, quads: &[Object], triangles: &[Object], camera: &Camera) -> Result<(), wgpu::SurfaceStatus> {
		self.update_projection(camera);
		self.update_transforms(quads, triangles);

		// let event = self.queue.submit([]);
		// let maintain = wgpu::PollType::Wait {
		//     submission_index: Some(event),
		//     timeout: None,
		// };

		// self.device.poll(maintain).unwrap();

		let mut offset: u64 = 0;
		for (i, quad) in quads.iter().enumerate() {
			let matrix =
				Mat4::from_scale_rotation_translation(quad.scale, Quat::from_rotation_z(quad.angle), quad.position);

			self.ubo
				.as_mut()
				.unwrap()
				.upload(offset + i as u64, &matrix, &self.queue);
		}
		offset += quads.len() as u64;
		for (i, triangle) in triangles.iter().enumerate() {
			let matrix = Mat4::from_scale_rotation_translation(
				triangle.scale,
				Quat::from_rotation_z(triangle.angle),
				triangle.position,
			);

			self.ubo
				.as_mut()
				.unwrap()
				.upload(offset + i as u64, &matrix, &self.queue);
		}

		// self.device.poll(maintain).unwrap();
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
		render_pass.set_pipeline(&self.pipeline.render_pipeline);

		render_pass.set_bind_group(0, &self.quad_material.bind_group, &[]);
		render_pass.set_bind_group(2, &self.projection_ubo.as_ref().unwrap().bind_group, &[]);
		render_pass.set_vertex_buffer(0, self.quad_mesh.buffer.slice(..self.quad_mesh.offset));
		render_pass.set_index_buffer(
			self.quad_mesh.buffer.slice(self.quad_mesh.offset..),
			wgpu::IndexFormat::Uint16,
		);

		let mut offset: usize = 0;
		for i in 0..quads.len() {
			render_pass.set_bind_group(1, &self.ubo.as_ref().unwrap().bind_groups[offset + i], &[]);
			render_pass.draw_indexed(0..6, 0, 0..1);
		}

		render_pass.set_bind_group(0, &self.triangle_material.bind_group, &[]);
		render_pass.set_vertex_buffer(0, self.triangle_mesh.buffer.slice(..self.triangle_mesh.offset));
		render_pass.set_index_buffer(
			self.triangle_mesh.buffer.slice(self.triangle_mesh.offset..),
			wgpu::IndexFormat::Uint16,
		);
		offset += quads.len();
		for i in 0..triangles.len() {
			render_pass.set_bind_group(1, &self.ubo.as_ref().unwrap().bind_groups[offset + i], &[]);
			render_pass.draw_indexed(0..3, 0, 0..1);
		}

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
