use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub scale: f32,
    pub aspect: f32,
}

impl Screen {
    pub fn default() -> Screen { Screen { width: 1920, height: 1080, scale: 1.0, aspect: 16.0 / 9.0 }}
    pub fn scale(&self) -> Screen {
        Screen {
            width: (self.width as f32 / self.scale) as usize,
            height: (self.height as f32 / self.scale) as usize,
            scale: 1.0,
            aspect: self.get_aspect(),
        }
    }
    pub fn get_aspect(&self) -> f32 { self.height as f32 / self.width as f32 }
    pub fn get_size(&self) -> Vec2 { Vec2 {x: self.width as f32, y: self.height as f32 }}
}