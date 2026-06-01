mod backend;
pub mod components;
pub mod editor_camera_system;
pub mod render_system;

use std::sync::Arc;

pub use components::*;
pub use editor_camera_system::*;
pub use render_system::*;
use wgpu::util::DeviceExt;
use ze_assets::ResourceManager;
use ze_core::{Mat4, Vec3};

use crate::backend::{
	mesh::{Mesh, Vertex},
	pipeline::{self, Pipeline},
	texture::{SpriteMaterial, SpriteMaterialUniform, TextureCache},
	ubo::{Ubo, UboGroup},
};

pub struct Renderer {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	pipeline: Pipeline,
	debug_pipeline: Pipeline,
	quad_mesh: Mesh,
	texture_cache: TextureCache,
	materials: Vec<SpriteMaterial>,
	material_bind_group_layout: wgpu::BindGroupLayout,
	ubo_bind_group_layout: wgpu::BindGroupLayout,
	ubo: Option<UboGroup>,
	ubo_object_count: usize,
	projection_ubo: Option<Ubo>,
	depth_texture: wgpu::Texture,
	depth_view: wgpu::TextureView,
}

impl Renderer {
	pub async fn new(window: Arc<winit::window::Window>) -> ze_core::Result<Self> {
		let instance = wgpu::Instance::default();

		let surface = instance.create_surface(window.clone())?;

		let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
			power_preference: wgpu::PowerPreference::HighPerformance,
			compatible_surface: Some(&surface),
			force_fallback_adapter: false,
		};

		let adapter = instance.request_adapter(&adapter_descriptor).await?;

		let info = adapter.get_info();

		// ze_log::info!("Renderer: {} ({:?})", info.name, info.backend);

		println!("Renderer:");
		println!("-> Vendor: {}", info.driver);
		println!("-> Name: {}", info.name);
		println!("-> Backend: {}", info.backend);
		println!("-> Driver: {} {}", info.driver, info.driver_info);

		let device_descriptor = wgpu::DeviceDescriptor {
			required_features: wgpu::Features::POLYGON_MODE_LINE,
			required_limits: wgpu::Limits::default(),
			label: Some("Device"),
			memory_hints: wgpu::MemoryHints::Performance,
			trace: wgpu::Trace::Off,
			experimental_features: wgpu::ExperimentalFeatures::disabled(),
		};

		let (device, queue) = adapter.request_device(&device_descriptor).await?;

		let size = window.inner_size();

		let surface_capabilities = surface.get_capabilities(&adapter);
		let surface_format = surface_capabilities
			.formats
			.iter()
			.copied()
			.find(wgpu::TextureFormat::is_srgb)
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

		let (depth_texture, depth_view) = Self::create_depth_texture(&device, config.width, config.height);

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
			let resources = ResourceManager::new("assets");
			let pipeline_builder = pipeline::Builder::new(&device)
				.with_shader_source(resources.engine_string("shaders/engine/sprite.wgsl")?)
				.with_pixel_format(surface_format)
				.with_vertex_buffer_layout(Vertex::get_layout())
				.with_depth_write_enabled(false)
				.with_depth_compare(wgpu::CompareFunction::Always)
				.with_bind_group_layout(&material_bind_group_layout)
				.with_bind_group_layout(&ubo_bind_group_layout)
				.with_bind_group_layout(&ubo_bind_group_layout);

			render_pipeline = pipeline_builder.build()?;
		}

		let debug_pipeline = pipeline::Builder::new(&device)
			.with_name("Physics Debug Draw")
			.with_shader_source(DEBUG_LINE_SHADER)
			.with_pixel_format(surface_format)
			.with_vertex_buffer_layout(Vertex::get_layout())
			.with_bind_group_layout(&ubo_bind_group_layout)
			.with_topology(wgpu::PrimitiveTopology::LineList)
			.with_polygon_mode(wgpu::PolygonMode::Line)
			.with_cull_mode(None)
			.with_depth_write_enabled(false)
			.with_depth_compare(wgpu::CompareFunction::Always)
			.build()?;

		let projection_ubo = Some(Ubo::new(&device, &ubo_bind_group_layout));
		let texture_cache = TextureCache::new(&device, &queue);
		let materials = Vec::new();

		Ok(Self {
			surface,
			device,
			queue,
			config,
			size,
			pipeline: render_pipeline,
			debug_pipeline,
			quad_mesh,
			texture_cache,
			materials,
			material_bind_group_layout,
			ubo_bind_group_layout,
			ubo: None,
			ubo_object_count: 0,
			projection_ubo,
			depth_texture,
			depth_view,
		})
	}

	pub fn build_ubos_for_objects(&mut self, objects_count: usize) {
		let objects_count = objects_count.max(1);
		self.ubo = Some(UboGroup::new(&self.device, objects_count, &self.ubo_bind_group_layout));
		self.ubo_object_count = objects_count;
	}

	fn ensure_ubos_for_objects(&mut self, objects_count: usize) {
		let objects_count = objects_count.max(1);
		if self.ubo.is_none() || self.ubo_object_count != objects_count {
			self.build_ubos_for_objects(objects_count);
		}
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;

			self.surface.configure(&self.device, &self.config);

			let (depth_texture, depth_view) =
				Self::create_depth_texture(&self.device, self.config.width, self.config.height);

			self.depth_texture = depth_texture;
			self.depth_view = depth_view;
		}
	}

	fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView) {
		let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
			label: Some("Depth Texture"),
			size: wgpu::Extent3d {
				width: width.max(1),
				height: height.max(1),
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Depth32Float,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		});

		let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

		(depth_texture, depth_view)
	}

	pub fn aspect_ratio(&self) -> f32 { self.size.width as f32 / self.size.height.max(1) as f32 }

	pub fn request_sprite_redraw(
		&mut self,
		items: &[render_system::SpriteRenderItem],
		debug_lines: &[render_system::DebugLine],
		camera: &render_system::CameraRenderData,
		resources: &ResourceManager,
	) {
		self.render_sprite_items(items, debug_lines, camera, resources);
	}

	fn update_projection(&mut self, camera: &render_system::CameraRenderData) {
		self.projection_ubo
			.as_mut()
			.expect("Cannon get projection_ubo as mut :(")
			.upload(&camera.view_projection, &self.queue);
	}

	fn render_sprite_items(
		&mut self,
		items: &[render_system::SpriteRenderItem],
		debug_lines: &[render_system::DebugLine],
		camera: &render_system::CameraRenderData,
		resources: &ResourceManager,
	) {
		self.ensure_ubos_for_objects(items.len());
		self.update_projection(camera);

		self.materials.clear();
		self.materials
			.reserve(items.len().saturating_sub(self.materials.capacity()));

		for (i, item) in items.iter().enumerate() {
			let texture = self
				.texture_cache
				.get_or_load(&item.texture, resources, &self.device, &self.queue);
			let sprite_size = sprite_size_to_world_scale(&item.size, texture.dimensions());
			let transform = item.transform * Mat4::from_scale(Vec3::new(sprite_size[0], sprite_size[1], 1.0));

			self.ubo
				.as_mut()
				.expect("Cannon get ubo as mut ):")
				.upload(i as u64, &transform, &self.queue);

			let tint = item.color.tint.unwrap_or([1.0, 1.0, 1.0, 1.0]);
			let mode = match item.color.mode {
				components::SpriteColorMode::None => 0.0,
				components::SpriteColorMode::Multiply => 1.0,
				components::SpriteColorMode::GrayscaleTint => 2.0,
			};
			let uniform = SpriteMaterialUniform {
				tint,
				params: [
					mode,
					item.color.strength,
					item.color.saturation_threshold,
					item.texture_rotation_degrees.to_radians(),
				],
			};

			self.materials.push(SpriteMaterial::new(
				"Sprite Material",
				texture,
				uniform,
				&self.device,
				&self.material_bind_group_layout,
			));
		}

		let debug_vertices = debug_lines
			.iter()
			.flat_map(|line| {
				[
					Vertex {
						position: line.start.to_array(),
						color: line.color,
					},
					Vertex {
						position: line.end.to_array(),
						color: line.color,
					},
				]
			})
			.collect::<Vec<_>>();
		let debug_vertex_buffer = (!debug_vertices.is_empty()).then(|| {
			self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Physics Debug Lines"),
				contents: bytemuck::cast_slice(&debug_vertices),
				usage: wgpu::BufferUsages::VERTEX,
			})
		});

		let drawable = self.surface.get_current_texture();
		let wgpu::CurrentSurfaceTexture::Success(drawable) = drawable else {
			return;
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
					r: f64::from(camera.clear_color[0]),
					g: f64::from(camera.clear_color[1]),
					b: f64::from(camera.clear_color[2]),
					a: f64::from(camera.clear_color[3]),
				}),
				store: wgpu::StoreOp::Store,
			},
		};

		let render_pass_descriptor = wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(color_attachment)],
			depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
				view: &self.depth_view,
				depth_ops: Some(wgpu::Operations {
					load: wgpu::LoadOp::Clear(1.0),
					store: wgpu::StoreOp::Store,
				}),
				stencil_ops: None,
			}),
			occlusion_query_set: None,
			timestamp_writes: None,
			multiview_mask: None,
		};

		let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
		render_pass.set_pipeline(&self.pipeline.render_pipeline);
		render_pass.set_bind_group(
			2,
			&self
				.projection_ubo
				.as_ref()
				.expect("Cannon get projection_ubo as ref :(")
				.bind_group,
			&[],
		);

		for (i, material) in self.materials.iter().enumerate() {
			render_pass.set_bind_group(0, &material.bind_group, &[]);
			render_pass.set_bind_group(
				1,
				&self.ubo.as_ref().expect("Cannon get ubo as ref :(").bind_groups[i],
				&[],
			);

			render_pass.set_vertex_buffer(0, self.quad_mesh.buffer.slice(..self.quad_mesh.offset));
			render_pass.set_index_buffer(
				self.quad_mesh.buffer.slice(self.quad_mesh.offset..),
				wgpu::IndexFormat::Uint16,
			);
			render_pass.draw_indexed(0..6, 0, 0..1);
		}

		if let Some(debug_vertex_buffer) = &debug_vertex_buffer {
			render_pass.set_pipeline(&self.debug_pipeline.render_pipeline);
			render_pass.set_bind_group(
				0,
				&self
					.projection_ubo
					.as_ref()
					.expect("Cannon get projection_ubo as ref :(")
					.bind_group,
				&[],
			);
			render_pass.set_vertex_buffer(0, debug_vertex_buffer.slice(..));
			render_pass.draw(0..debug_vertices.len() as u32, 0..1);
		}

		drop(render_pass);
		self.queue.submit(std::iter::once(command_encoder.finish()));

		drawable.present();
	}
}

const DEBUG_LINE_SHADER: &str = r"
@group(0) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = view_projection * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
";

fn sprite_size_to_world_scale(size: &components::SpriteSize, dimensions: (u32, u32)) -> [f32; 2] {
	match size {
		components::SpriteSize::Auto => auto_sprite_size(dimensions.0, dimensions.1),
		components::SpriteSize::Custom { width, height } => [*width, *height],
	}
}

fn auto_sprite_size(width: u32, height: u32) -> [f32; 2] {
	if width == 0 || height == 0 {
		return [1.0, 1.0];
	}

	if width >= height {
		[width as f32 / height as f32, 1.0]
	} else {
		[1.0, height as f32 / width as f32]
	}
}
