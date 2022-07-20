mod vec2;
mod vec3;
mod screen;
mod shapes;
mod camera;

pub use minifb::{Window, Key, WindowOptions, Scale, KeyRepeat, MouseMode};

pub use screen::*;
use vec2::*;
use vec3::*;
use shapes::*;
use camera::*;

pub fn run(screen: &Screen, window: &mut Window, buffer: &mut [u32]) {

    const SENSITIVITY: f32 = 0.025;
    const SPEED: f32 = 0.05;

    let mut camera = Camera::create( Vec3 { x: 0.0, y: 0.0, z: 0.0 } );
    let sphere = Sphere {
        position: Vec3 { x: 5.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        color: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
    };
    let plane = Plane {
        normal: Vec3 { x: 0.0, y: 0.0, z: 1.0 }.norm(),
        z: -1.0,
        color: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
    };

    let mut camera_move = Vec3::default();
    let mut light: Vec3;
    let mut uv: Vec2;
    let mut mouse_xy = Vec2::default();
    let mut normal = Vec3::default();

    let mut cur_speed = SPEED;
    let mut time = 0.0;


    //TODO: write more functions from this while
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for w in 0..screen.width {
            for h in 0..screen.height {

                uv = get_uv(w as f32, h as f32, &screen);
                camera.cast_ray(uv, mouse_xy);

                let intersections = get_all_intersections(camera, sphere, plane);
                let (nearest_point, index_of_shape) = get_nearest_point(&intersections);

                let shape_color;
                match index_of_shape {
                    0 => {
                        normal = camera.position.sub(sphere.position).add(camera.ray_direction.mul_num(nearest_point)).norm();
                        shape_color = sphere.color;
                    },
                    1 => {
                        normal = camera.position.sub(sphere.position).add(camera.ray_direction.mul_num(nearest_point)).norm();
                        shape_color = sphere.color;
                    },
                    2 => {
                        normal = plane.normal;
                        shape_color = plane.color;
                    },
                    _ => shape_color = Vec3::default(),
                }

                // Setting lights and colors
                light = Vec3 { x: f32::cos(time), y: f32::sin(time), z: f32::cos(time)-1.0}.norm();

                let mut color = 0x9999FF;
                if camera.ray_direction.dot(light) < -0.99 {
                    color = 0xFFFFAA;
                }
                if nearest_point > 0.0 {
                    let pixel_alpha = clamp(-normal.dot(light), 0.2, 1.0) * 255.0;
                    color = new_color_a(shape_color, pixel_alpha);
                } 

                buffer[h*screen.width + w] = color; 
            }
        }

        window.update_with_buffer(&buffer, screen.width, screen.height).unwrap();
        time = time + 0.01;

        if window.is_key_down(Key::LeftShift) {
            cur_speed = SPEED * 2.0;
        }

        for key in window.get_keys() {
            match key {
                Key::W => camera_move = Vec3 { x: cur_speed, ..camera_move},
                Key::S => camera_move = Vec3 { x: -cur_speed, ..camera_move},
                Key::D => camera_move = Vec3 { y: cur_speed, ..camera_move},
                Key::A => camera_move = Vec3 { y: -cur_speed, ..camera_move},
                Key::Space => camera.position.z = camera.position.z + cur_speed,
                Key::LeftCtrl => camera.position.z = camera.position.z - cur_speed,
                Key::Right => mouse_xy.x = mouse_xy.x + SENSITIVITY,
                Key::Left => mouse_xy.x = mouse_xy.x - SENSITIVITY,
                Key::Up => mouse_xy.y = mouse_xy.y + SENSITIVITY,
                Key::Down => mouse_xy.y = mouse_xy.y - SENSITIVITY,
                _ => (),
            }
        }

        camera.movement(camera_move, mouse_xy);
        
        camera_move = Vec3::default();
        cur_speed = SPEED;
    }
}

fn get_uv(cur_width: f32, cur_heught: f32, screen: &Screen) -> Vec2 {
        let w_h = Vec2{x: cur_width as f32, y: cur_heught as f32};
        let mut uv = w_h.div(screen.get_size()).mul_num(2.0).add_num(-1.0);
        uv.y = uv.y * -screen.aspect;
        uv
    }

fn new_color_a (color: Vec3, alpha: f32) -> u32 {
    let (r, g, b) = ((color.x * alpha) as u32, (color.y * alpha) as u32, (color.z * alpha) as u32);
    (r << 16) | (g << 8) | b
}

fn clamp(value: f32, minn: f32, maxx: f32) -> f32{
    f32::min(f32::max(value, minn), maxx)
}

fn get_all_intersections(camera: Camera, sphere: Sphere, plane: Plane) -> [f32; 3] {
    let mut intersections = [0_f32; 3];
    (intersections[0], intersections[1]) = sphere.collision(camera);
    intersections[2] = plane.collision(camera);

    intersections
} 

fn get_nearest_point (points: &[f32; 3]) -> (f32, usize) {
    let mut index = 0;
    let mut point = -1.0;
    for (i, &p) in points.iter().enumerate() {
        if p > 0.0 && (p < point || point == -1.0) {
            point = p;
            index = i;
        }
    }
    (point, index)
}