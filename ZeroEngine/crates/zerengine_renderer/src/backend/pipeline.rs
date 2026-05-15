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

pub struct PipelineBuilder {
    shader_source: Option<ShaderSource>,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    name: String,
}

#[allow(dead_code)]
impl PipelineBuilder {
	pub fn new() -> Self {
        Self {
            shader_source: None,
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

	pub fn build(self, device: &wgpu::Device) -> zerengine_core::Result<Pipeline> {
        let source_code = match self.shader_source {
            Some(ShaderSource::Path(path)) => {
                let filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
                fs::read_to_string(&filepath)?
            }
            Some(ShaderSource::Source(source)) => source,
            None => {
                zerengine_core::bail!("Pipeline `{}` has no shader source", self.name);
            }
        };

        let shader_module_name = format!("{} Shader", self.name);

        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some(&shader_module_name),
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

impl Default for PipelineBuilder {
    fn default() -> Self { Self::new() }
}
