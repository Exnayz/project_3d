use project_3d::*;

fn main() {
    let screen = Screen{
        scale: 1.25 * 4.0,          // resampling
        ..Screen::default()
    };
    let screen = screen.scale();

    let mut buffer = vec![0u32; screen.width * screen.height];
    let mut window = Window::new(
        "Test",
        screen.width,
        screen.height,
        WindowOptions {
            scale: Scale::X4,       // resampling
            borderless: true,
            ..WindowOptions::default()
        }
    ).unwrap();
    window.set_position(-10, 0);
    
    
    run(&screen, &mut window, &mut buffer);
}