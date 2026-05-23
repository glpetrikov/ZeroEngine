use std::{fs, path::PathBuf};

pub enum ShaderSource {
	Path(PathBuf),
	Source(String),
}

#[allow(dead_code)]
pub struct Pipeline {
	pub render_pipeline: wgpu::RenderPipeline,
	pub name: String,
}

pub struct Builder<'a> {
	shader_source: Option<ShaderSource>,
	vertex_entry: String,
	fragment_entry: String,
	pixel_format: wgpu::TextureFormat,
	vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
	name: String,
	bind_group_layouts: Vec<Option<&'a wgpu::BindGroupLayout>>,
	device: &'a wgpu::Device,
}

#[allow(dead_code)]
impl<'a> Builder<'a> {
	// TODO: add shader_from_filepath(path, fs_main, vs_main) and
	// shader_from_source(source, fs_main, vs_main)
	pub fn new(device: &'a wgpu::Device) -> Self {
		Self {
			shader_source: None,
			vertex_entry: "vs_main".to_string(),
			fragment_entry: "fs_main".to_string(),
			pixel_format: wgpu::TextureFormat::Bgra8UnormSrgb,
			vertex_buffer_layouts: vec![],
			name: "Unnamed Pipeline".to_string(),
			bind_group_layouts: vec![],
			device,
		}
	}

	pub fn with_name(mut self, name: impl Into<String>) -> Self {
		self.name = name.into();
		self
	}

	pub fn with_shader_path(mut self, path: impl Into<PathBuf>) -> Self {
		self.shader_source = Some(ShaderSource::Path(path.into()));
		self
	}

	pub fn with_shader_source(mut self, source: impl Into<String>) -> Self {
		self.shader_source = Some(ShaderSource::Source(source.into()));
		self
	}

	pub fn with_shader(
		mut self,
		path: impl Into<PathBuf>,
		vertex_entry: impl Into<String>,
		fragment_entry: impl Into<String>,
	) -> Self {
		self.shader_source = Some(ShaderSource::Path(path.into()));
		self.vertex_entry = vertex_entry.into();
		self.fragment_entry = fragment_entry.into();
		self
	}

	pub fn with_vertex_entry(mut self, entry: impl Into<String>) -> Self {
		self.vertex_entry = entry.into();
		self
	}

	pub fn with_fragment_entry(mut self, entry: impl Into<String>) -> Self {
		self.fragment_entry = entry.into();
		self
	}

	pub fn with_pixel_format(mut self, format: wgpu::TextureFormat) -> Self {
		self.pixel_format = format;
		self
	}

	pub fn with_vertex_buffer_layout(mut self, layout: wgpu::VertexBufferLayout<'static>) -> Self {
		self.vertex_buffer_layouts.push(layout);
		self
	}

	pub fn with_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
		self.bind_group_layouts.push(Some(layout));
		self
	}

	pub fn build(self) -> zerengine_core::Result<Pipeline> {
		let source_code = match self.shader_source {
			Some(ShaderSource::Path(path)) => {
				let filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);

				fs::read_to_string(&filepath)
					.unwrap_or_else(|e| panic!("Failed to read shader `{}`: {e}", filepath.display()))
			}
			Some(ShaderSource::Source(source)) => source,
			None => {
				zerengine_core::bail!("Pipeline `{}` has no shader source", self.name.clone());
			}
		};

		let shader_module_name = format!("{} Shader", self.name);

		let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
			label: Some(&shader_module_name),
			source: wgpu::ShaderSource::Wgsl(source_code.into()),
		};

		let shader_module = self.device.create_shader_module(shader_module_descriptor);

		let pipeline_layout_name = format!("{} Pipeline Layout", self.name);

		let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
			label: Some(pipeline_layout_name.as_str()),
			bind_group_layouts: &self.bind_group_layouts,
			immediate_size: 0,
		};
		let pipeline_layout = self.device.create_pipeline_layout(&pipeline_layout_descriptor);

		let render_targets = [Some(wgpu::ColorTargetState {
			format: self.pixel_format,
			blend: Some(wgpu::BlendState::REPLACE), // TODO: add enum BlendMode
			write_mask: wgpu::ColorWrites::ALL,
		})];

		let render_pipeline_name = format!("{} Render Pipeline", self.name);

		let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
			label: Some(render_pipeline_name.as_str()),
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader_module,
				entry_point: Some(&self.vertex_entry),
				buffers: &self.vertex_buffer_layouts,
				compilation_options: wgpu::PipelineCompilationOptions::default(),
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader_module,
				entry_point: Some(&self.fragment_entry),
				targets: &render_targets,
				compilation_options: wgpu::PipelineCompilationOptions::default(),
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				polygon_mode: wgpu::PolygonMode::Fill,
				unclipped_depth: false,
				conservative: false,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview_mask: None,
			cache: None,
		};

		let render_pipeline = self.device.create_render_pipeline(&render_pipeline_descriptor);

		let pipeline = Pipeline {
			render_pipeline,
			name: self.name.clone(),
		};

		Ok(pipeline)
	}
}
