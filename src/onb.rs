

use glam;
use glam::Vec3A;

pub struct ONB {
    axis: [Vec3A; 3],
}

impl ONB {
    pub fn new() -> ONB { ONB { axis: [Vec3A::ZERO; 3] } }
    pub fn u(&self) -> Vec3A { self.axis[0] }
    pub fn v(&self) -> Vec3A { self.axis[1] }
    pub fn w(&self) -> Vec3A { self.axis[2] }
    pub fn local(&self, a: f32, b: f32, c: f32) -> Vec3A { a * self.u() + b * self.v() + c * self.w() }
    pub fn local_vec(&self, a: Vec3A) -> Vec3A { a.x * self.u() + a.y * self.v() + a.z * self.w() }
    pub fn build_from_w(&mut self, n: Vec3A) {
        self.axis[2] = n.normalize();
        let a: Vec3A = if self.w().x.abs() > 0.9 { Vec3A::Y } else { Vec3A::X };
        self.axis[1] = self.w().cross(a).normalize();
        self.axis[0] = self.w().cross(self.v());
    }
}
