use crate::{vec2::Vec2, vec3::Vec3, camera::Camera};

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub color: Vec3,
    pub z: f32,
}

impl Plane {
    pub fn collision(&self, camera: Camera) -> f32 {
        let intersec = -(camera.position.dot(self.normal) - self.z) / camera.ray_direction.dot(self.normal);
        intersec
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub position: Vec3,
    pub color: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn collision(&self, camera: Camera) -> (f32, f32) {
        let camera_pos = camera.position.sub(self.position);
        let b = camera_pos.dot(camera.ray_direction);
        let c = camera_pos.dot(camera_pos) - self.radius * self.radius;
        let h = b*b - c;
        if h < 0.0 { return (-1.0, -1.0); }
        let h = f32::sqrt(h);
        (-b - h, -b + h)
    }
}
