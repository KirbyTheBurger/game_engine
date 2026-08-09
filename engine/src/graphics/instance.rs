use std::collections::HashMap;

use glam::{Vec2, Vec3};
use wgpu::BindGroup;

use crate::{Shared, graphics::texture::{self, Texture}};

pub static INSTANCES: Shared<Vec<Instance>> = Shared::new();
pub static TEXTURE_REG: Shared<TextureReg> = Shared::new();

#[derive(Clone)]
pub struct Instance {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec2,
    pub texture_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model = glam::Mat4::from_scale_rotation_translation(
            self.scale.extend(1.0),
            glam::Quat::from_rotation_z(self.rotation),
            self.position,
        );

        InstanceRaw { model: model.to_cols_array_2d() }
    }
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct TextureReg(pub HashMap<i32, (Texture, BindGroup)>);

impl TextureReg {
    pub fn load(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        id: i32,
        bytes: &[u8]
    ) -> anyhow::Result<()> {
        let texture = texture::Texture::from_bytes(
            device,
            queue,
            bytes,
            &format!("Texture (id: {id})"),
        )?;
        
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler)
                },
            ],
            label: Some(&format!("Bind group (id: {id})")),
        });

        self.0.insert(id, (texture, bind_group));

        Ok(())
    }
}
