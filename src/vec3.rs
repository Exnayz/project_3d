#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32, 
    pub z: f32,
}

impl Vec3 {
    pub fn default() -> Vec3 { Vec3 { x: 0.0, y: 0.0, z: 0.0 }}
    pub fn from_number(num: f32) -> Vec3 { Vec3 { x: num, y: num, z: num }}
    pub fn add(&self, v: Vec3) -> Vec3 { Vec3 { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }}
    pub fn sub(&self, v: Vec3) -> Vec3 { Vec3 { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }}
    pub fn mul(&self, v: Vec3) -> Vec3 { Vec3 { x: self.x * v.x, y: self.y * v.y, z: self.z * v.z }}
    pub fn mul_num(&self, num: f32) -> Vec3 { Vec3 { x: self.x * num, y: self.y * num, z: self.z * num }}
    pub fn div(&self, v: Vec3) -> Vec3 { Vec3 { x: self.x / v.x, y: self.y / v.y, z: self.z / v.z }}
    pub fn dot(&self, v: Vec3) -> f32 { self.x*v.x + self.y*v.y + self.z*v.z }
    pub fn norm(&self) -> Vec3 { self.mul_num(1.0 / self.lenght()) }
    pub fn lenght(&self) -> f32 { f32::sqrt(self.x*self.x + self.y*self.y + self.z*self.z) }
    pub fn print(&self) { print!("({}, {}, {})", self.x, self.y, self.z) }
    pub fn reverse(&self) -> Vec3 { Vec3 { x: -self.x, y: -self.y, z: -self.z }}
}

