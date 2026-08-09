use std::collections::HashMap;

use crate::Shared;

pub static CAMERA: Shared<Camera> = Shared::new();
pub static CAM_INSTANCES: Shared<CamInstances> = Shared::new();

#[derive(Clone, Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub aspect: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection(&self) -> glam::Mat4 {
        let half_h = self.zoom;
        let half_w = self.zoom * self.aspect;

        let view = glam::Mat4::from_translation(
            glam::Vec3::new(-self.x, -self.y, 0.0)
        );
        let proj = glam::camera::rh::proj::directx::orthographic(
            -half_w, half_w,
            -half_h, half_h,
            self.znear, self.zfar,
        );

        proj * view
    }
}

#[derive(Clone)]
pub struct CamInstances {
    pub cameras: HashMap<i32, (f32, f32)>,
    next_id: i32,
    pub primary: Option<i32>,
}

impl CamInstances {
    pub fn new() -> Self {
        Self {
            cameras: HashMap::new(),
            next_id: 0,
            primary: None,
        }
    }

    pub fn next_id(&mut self) -> i32 {
        self.next_id += 1;
        self.next_id - 1
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection().to_cols_array_2d();
    }
}
