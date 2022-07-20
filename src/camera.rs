use crate::{vec2::Vec2, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub ray_direction: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn create(position: Vec3) -> Camera {
        Camera { 
            fov: 1.0,
            position: position, 
            ray_direction: Vec3::default(),
        }
    }
    
    pub fn cast_ray (&mut self, uv: Vec2, mouse_xy: Vec2) {
        let mut ray_direction = Vec3 { x: self.fov, y: uv.x, z: uv.y }.norm();
        ray_direction = Vec3 {
            x: ray_direction.x * f32::cos(mouse_xy.y) - ray_direction.z * f32::sin(mouse_xy.y),
            z: ray_direction.x * f32::sin(mouse_xy.y) + ray_direction.z * f32::cos(mouse_xy.y),
            y: ray_direction.y, 
        };
        ray_direction = Vec3 {
            x: ray_direction.x * f32::cos(mouse_xy.x) - ray_direction.y * f32::sin(mouse_xy.x),
            y: ray_direction.x * f32::sin(mouse_xy.x) + ray_direction.y * f32::cos(mouse_xy.x),
            z: ray_direction.z, 
        };
        self.ray_direction = ray_direction;
    }

    pub fn movement(&mut self, mut camera_move: Vec3, mouse_xy: Vec2) {
        camera_move = Vec3 {
            x: camera_move.x * f32::cos(mouse_xy.x) - camera_move.y * f32::sin(mouse_xy.x),
            y: camera_move.x * f32::sin(mouse_xy.x) + camera_move.y * f32::cos(mouse_xy.x),
            z: camera_move.z,
        };

        self.position = self.position.add(camera_move);
    }
}