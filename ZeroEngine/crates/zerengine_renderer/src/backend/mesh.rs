use bytemuck::{Pod, Zeroable};
use glam::*;
use wgpu::util::DeviceExt;

pub struct Mesh {
	pub buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
	pub position: [f32; 3],
	pub color: [f32; 4],
}

impl Vertex {
	pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
		const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];

		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as u64,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &ATTRIBUTES,
		}
	}

	pub fn make_triangle(device: &wgpu::Device) -> Mesh {
		let vertices: [Vertex; 3] = [
			Vertex {
				position: Vec3::new(-0.75, -0.75, 0.0).to_array(),
				color: Vec4::new(0.6, 0.6, 0.6, 1.0).to_array(),
			},
			Vertex {
				position: Vec3::new(0.75, -0.75, 0.0).to_array(),
				color: Vec4::new(0.0, 1.0, 0.0, 1.0).to_array(),
			},
			Vertex {
				position: Vec3::new(0.0, 0.75, 0.0).to_array(),
				color: Vec4::new(0.0, 0.0, 1.0, 1.0).to_array(),
			},
		];

		let bytes: &[u8] = bytemuck::cast_slice(&vertices);

		let buffer_descriptor = wgpu::util::BufferInitDescriptor {
			label: Some("triangle vertex buffer"),
			contents: bytes,
			usage: wgpu::BufferUsages::VERTEX,
		};

		Mesh {
			buffer: device.create_buffer_init(&buffer_descriptor),
		}
	}
}
