#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}


impl Vec2 {
    pub fn default() -> Vec2 { Vec2 { x: 0.0, y: 0.0 }}
    pub fn from_number(num: f32) -> Vec2 { Vec2 { x: num, y: num }}
    pub fn add(&self, v: Vec2) -> Vec2 { Vec2 { x: self.x + v.x, y: self.y + v.y }}
    pub fn add_num(&self, num: f32) -> Vec2 { Vec2 { x: self.x + num, y: self.y + num }}
    pub fn sub(&self, v: Vec2) -> Vec2 { Vec2 { x: self.x + v.x, y: self.y + v.y }}
    pub fn mul(&self, v: Vec2) -> Vec2 { Vec2 { x: self.x * v.x, y: self.y * v.y }}
    pub fn mul_num(&self, num: f32) -> Vec2 { Vec2 { x: self.x * num, y: self.y * num }}
    pub fn div(&self, v: Vec2) -> Vec2 { Vec2 { x: self.x / v.x, y: self.y / v.y }}
    pub fn dot(&self, v: Vec2) -> f32 { self.x*v.x + self.y*v.y }
    pub fn norm(&self) -> Vec2 { self.mul_num(1.0 / self.lenght()) }
    pub fn lenght(&self) -> f32 { f32::sqrt(self.x*self.x + self.y*self.y) }
    pub fn print(&self) { print!("({}, {})", self.x, self.y) }
    pub fn equal(&self, v: Vec2) -> bool { self.x == v.x && self.y == v.y }
}