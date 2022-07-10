mod vec2;
mod vec3;
pub mod screen;

use std::str::SplitWhitespace;

pub use minifb::{Window, Key, WindowOptions, Scale, KeyRepeat};
pub use screen::*;
use vec3::*;
use vec2::*;

pub fn run(screen: &Screen, window: &mut Window, buffer: &mut [u32]) {

    let camera_pos = Vec3 { x: -5.0, y: 0.0, z: 0.0 };
    let mut main_light: Vec3;
    let mut ray_direction: Vec3;
    let mut uv: Vec2;

    let speed = 0.1;
    let mut time = 0.0;
    let r = 1.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for w in 0..screen.width {
            for h in 0..screen.height {
                let w_h = Vec2{x: w as f32, y: h as f32};
                
                main_light = Vec3 { x: f32::sin(time * speed), y: f32::cos(time * speed), z: 0.5}.norm();

                uv = w_h.div(screen.get_size()).mul_num(2.0).add_num(-1.0);
                uv.y = uv.y * screen.aspect;

                ray_direction = Vec3 { x: 1.0, y: uv.x, z: uv.y }.norm();
                let intersection = sphere_colosion(camera_pos, ray_direction, r);

                let mut color = 0;
                if intersection.x > 0.0 {
                    let normal = camera_pos.add(ray_direction.mul_n(intersection.x)).norm();
                    let tmp_color = -normal.cos_angle(main_light) * 255.0;
                    color = from_u8_rgb(tmp_color as u8, tmp_color as u8, tmp_color as u8);
                }
                
                buffer[h*screen.width + w] = color; 
            }
        }
        window.update_with_buffer(&buffer, screen.width, screen.height).unwrap();
        time = time + 1.0;
    }
}


fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn clamp(value: f32, minn: f32, maxx: f32) -> f32{
    f32::min(f32::max(value, minn), maxx)
}

fn sphere_colosion(camera_pos: Vec3, camera_direction: Vec3, r: f32) -> Vec2 {
    let b = camera_pos.dot(camera_direction);
    let c = camera_pos.dot(camera_pos) - r*r;
    let h = b*b - c;
    if h < 0.0 {
        return Vec2::from_number(-1.0);
    }
    let h = f32::sqrt(h);
    Vec2 { x: -b - h, y: -b + h }
}