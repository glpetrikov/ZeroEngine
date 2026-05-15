use std::{fs, path::PathBuf};

pub struct Pipeline {
	pub render_pipeline: wgpu::RenderPipeline,
	#[allow(dead_code)] // TODO: remove
	pub name: String,
}

pub struct PipelineBuilder {
	// TODO: add from string option of shader source code
	shader_filename: PathBuf,
	vertex_entry: String,
	fragment_entry: String,
	// TODO: add Mesh Shaders
	pixel_format: wgpu::TextureFormat,
	name: String,
}

#[allow(dead_code)]
impl PipelineBuilder {
	pub fn new() -> Self {
		Self {
			shader_filename: PathBuf::new(),
			vertex_entry: "vs_main".to_string(),
			fragment_entry: "fs_main".to_string(),
			pixel_format: wgpu::TextureFormat::Bgra8UnormSrgb,
			name: "Unnamed Pipeline".to_string(),
		}
	}

	pub fn with_name(mut self, name: impl Into<String>) -> Self {
		self.name = name.into();
		self
	}

	pub fn with_shader(
		mut self,
		shader_filename: impl Into<PathBuf>,
		vertex_entry: impl Into<String>,
		fragment_entry: impl Into<String>,
	) -> Self {
		self.shader_filename = shader_filename.into();
		self.vertex_entry = vertex_entry.into();
		self.fragment_entry = fragment_entry.into();
		self
	}

	pub fn with_shader_path(mut self, filename: impl Into<PathBuf>) -> Self {
		self.shader_filename = filename.into();
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

	pub fn build(self, device: &wgpu::Device) -> zerengine_core::Result<Pipeline> {
		let filepath = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&self.shader_filename);

		let source_code = fs::read_to_string(&filepath)
			.map_err(|e| zerengine_core::anyhow!("Failed to read shader {:?}: {}", filepath, e))?;

		let shader_module_name = format!("{} Shader", self.name);

		let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
			label: Some(shader_module_name.as_str()),
			source: wgpu::ShaderSource::Wgsl(source_code.into()),
		};
		let shader_module = device.create_shader_module(shader_module_descriptor);

		let pipeline_layout_name = format!("{} Pipeline Layout", self.name);

		let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
			label: Some(pipeline_layout_name.as_str()),
			bind_group_layouts: &[],
			immediate_size: 0,
		};
		let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

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
				buffers: &[],
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

		Ok(Pipeline {
			render_pipeline: device.create_render_pipeline(&render_pipeline_descriptor),
			name: self.name,
		})
	}
}
