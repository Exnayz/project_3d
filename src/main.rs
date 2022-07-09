use minifb::{Window, WindowOptions, Key};

fn main() {
    let screen_aspect = 1.25; // this is disabling the scale for small screens (Windows setting)
    
    let width = 1920;
    let height = 1080;
    let width = (width as f32 / screen_aspect) as usize;
    let height = (height as f32 / screen_aspect) as usize;

    let aspect = height as f32 / width as f32;
    
    let mut buffer = vec![0u32; width * height];
    let mut window = Window::new(
        "Test",
        width,
        height,
        WindowOptions { 
            borderless: true,
            ..WindowOptions::default()
        }
    ).unwrap();
    window.set_position(-10, 0);

    let mut x: f32;
    let mut y: f32;
    let r = 0.5;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for w in 0..width {
            for h in 0..height {
                x = w as f32 / width as f32 * 2.0 - 1.0;
                y = h as f32 / height as f32 * 2.0 - 1.0;
                y = y * aspect;
                if x*x + y*y <= r*r {
                    buffer[h*width + w] = 0x_ff_ff_ff; 
                }
            }
        }
        
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}