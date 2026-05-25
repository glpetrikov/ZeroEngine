use crate::backend::bind_group;

use std::num::NonZeroU64;

pub struct UBO {
    pub buffer: wgpu::Buffer,
    pub bind_groups: Vec<wgpu::BindGroup>,
    allignment: u64,
}

impl UBO {
    pub fn new(device: &wgpu::Device, object_count: usize, layout: &wgpu::BindGroupLayout) -> Self {
        fn align_to(value: u64, alignment: u64) -> u64 {
            ((value + alignment - 1) / alignment) * alignment
        }
        let stride = align_to(
            std::mem::size_of::<glam::Mat4>() as u64,
            device.limits().min_uniform_buffer_offset_alignment as u64,
        );

        let buffer_descriptor = wgpu::BufferDescriptor {
            label: Some("UBO"),
            size: object_count as u64 * stride,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };

        let buffer = device.create_buffer(&buffer_descriptor);

        let mut bind_groups: Vec<wgpu::BindGroup> = Vec::new();
        for i in 0..object_count {
            let mut builder = bind_group::Builder::new(device);
            builder.set_layout(layout);
            builder.add_buffer(
                &buffer,
                i as u64 * stride,
                NonZeroU64::new(std::mem::size_of::<glam::Mat4>() as u64).unwrap(),
            );
            bind_groups.push(builder.build("Matrix"));
        }

        Self {
            buffer,
            bind_groups,
            allignment: stride,
        }
    }

    pub fn upload(&mut self, i: u64, matrix: glam::Mat4, queue: &wgpu::Queue) {
        let offset = i * self.allignment;
        let data = bytemuck::bytes_of(&matrix);
        queue.write_buffer(&self.buffer, offset, data);
    }
}