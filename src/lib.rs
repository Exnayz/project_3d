mod vec2;
mod vec3;
pub mod screen;

pub use minifb::{Window, Key, WindowOptions, Scale, KeyRepeat, MouseMode};
pub use screen::*;
use vec2::*;
use vec3::*;

pub fn run(screen: &Screen, window: &mut Window, buffer: &mut [u32]) {

    let mut camera_pos = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let sphere_pos = Vec3 { x: 5.0, y: 0.0, z: 0.0 };
    let plane_normal = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    let mut camera_move = Vec3::default();
    let mut ray_direction: Vec3;
    let mut light: Vec3;
    let mut uv: Vec2;
    let mut mouse_xy = Vec2::default();

    let sensitivity = 0.025;
    let speed = 0.05;
    let mut time = 0.0;
    let sphere_r = 1.0;
    let mut color;
    let mut intersection; 
    let mut normal = Vec3::default();
    let mut nearest_point;


    //TODO: write more functions from this while
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for w in 0..screen.width {
            for h in 0..screen.height {
                
                // Creating uv
                let w_h = Vec2{x: w as f32, y: h as f32};
                uv = w_h.div(screen.get_size()).mul_num(2.0).add_num(-1.0);
                uv.y = uv.y * -screen.aspect;

                ray_direction = Vec3 { x: 1.0, y: uv.x, z: uv.y }.norm();
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
                


                // All intersections
                nearest_point = -1.0; // WTF
                intersection = sphere_collision(camera_pos.sub(sphere_pos), ray_direction, sphere_r);
                if intersection.x >= 0.0 {
                    normal = camera_pos.sub(sphere_pos).add(ray_direction.mul_num(intersection.x)).norm();
                    nearest_point = intersection.x;
                } 

                intersection = plane_collision(camera_pos, ray_direction, plane_normal, -1.0);
                if intersection.x >= 0.0 && (nearest_point > intersection.x || nearest_point == -1.0) {
                    normal = plane_normal.norm();
                    nearest_point = intersection.x;
                }

                // Setting lights and colors
                light = Vec3 { x: f32::cos(time), y: f32::sin(time), z: f32::cos(time)}.norm();

                color = 0;
                if nearest_point > 0.0 {
                    let mut clr = -normal.dot(light);
                    clr = clr * 255.0;
                    clr = clamp(clr, 10.0, 255.0);
                    color = color_from_u8(clr as u8);
                }

                buffer[h*screen.width + w] = color; 
            }
        }

        window.update_with_buffer(&buffer, screen.width, screen.height).unwrap();
        time = time + 0.01;

        for key in window.get_keys() {
            match key {
                Key::W => camera_move = Vec3 { x: speed, ..camera_move},
                Key::S => camera_move = Vec3 { x: -speed, ..camera_move},
                Key::D => camera_move = Vec3 { y: speed, ..camera_move},
                Key::A => camera_move = Vec3 { y: -speed, ..camera_move},
                Key::Space => camera_pos.z = camera_pos.z + speed,
                Key::LeftCtrl => camera_pos.z = camera_pos.z - speed,
                Key::Right => mouse_xy.x = mouse_xy.x + sensitivity,
                Key::Left => mouse_xy.x = mouse_xy.x - sensitivity,
                Key::Up => mouse_xy.y = mouse_xy.y + sensitivity,
                Key::Down => mouse_xy.y = mouse_xy.y - sensitivity,
                _ => (),
            }
        }

        camera_move = Vec3 {
            x: camera_move.x * f32::cos(mouse_xy.x) - camera_move.y * f32::sin(mouse_xy.x),
            y: camera_move.x * f32::sin(mouse_xy.x) + camera_move.y * f32::cos(mouse_xy.x),
            z: camera_move.z,
        };
        camera_pos = camera_pos.add(camera_move);
        camera_move = Vec3::default();
    }
}



fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn color_from_u8(num: u8) -> u32 {
    let num = num as u32;
    (num << 16) | (num << 8) | num
}

fn clamp(value: f32, minn: f32, maxx: f32) -> f32{
    f32::min(f32::max(value, minn), maxx)
}

fn sphere_collision(camera_pos: Vec3, ray_direction: Vec3, r: f32) -> Vec2 {
    let b = camera_pos.dot(ray_direction);
    let c = camera_pos.dot(camera_pos) - r*r;
    let h = b*b - c;
    if h < 0.0 { return Vec2::from_number(-1.0); }
    let h = f32::sqrt(h);
    Vec2 { x: -b - h, y: -b + h }
}

fn plane_collision(camera_pos: Vec3, ray_direction: Vec3, plane_normal: Vec3, plane_z: f32) -> Vec2{
    let intersec = -(camera_pos.dot(plane_normal) - plane_z) / ray_direction.dot(plane_normal);
    Vec2::from_number(intersec)
}