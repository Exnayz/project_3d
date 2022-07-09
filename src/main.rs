use minifb::{Window, WindowOptions, Key, Scale};

fn main() {
    let screen_scale = 1.25 * 4.0; // this is disabling the scale for small screens (Windows setting)
    
    let width = 1920;
    let height = 1080;
    let width = (width as f32 / screen_scale) as usize;
    let height = (height as f32 / screen_scale) as usize;

    let aspect = height as f32 / width as f32;
    
    let mut buffer = vec![0u32; width * height];
    let mut window = Window::new(
        "Test",
        width,
        height,
        WindowOptions {
            scale: Scale::X4, 
            borderless: true,
            ..WindowOptions::default()
        }
    ).unwrap();
    window.set_position(-10, 0);

    let mut x: f32;
    let mut y: f32;
    let r = 0.5;

    let mut time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for w in 0..width {
            for h in 0..height {
                x = w as f32 / width as f32 * 2.0 - 1.0 ;
                y = h as f32 / height as f32 * 2.0 - 1.0;
                x = x + f32::sin(time * 0.000025) * 0.5;
                y = y * aspect;
                if x*x + y*y <= r*r {
                    let tmp = ((1.0 - ((x*x + y*y) / (r*r))) * 255.0) as u8;
                    let color = from_u8_rgb(tmp, tmp, tmp);
                    buffer[h*width + w] = color; 
                } else {
                    buffer[h*width + w] = 0; 
                }
            }
            time = time + 1.0;
        }
        
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}