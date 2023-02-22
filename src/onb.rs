// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the ONB (Orthogonal Normal Basis) struct

use glam::Vec3A;

#[derive(Clone)]
pub struct ONB {
    pub u: Vec3A,
    pub v: Vec3A,
    pub w: Vec3A,
}

impl ONB {
    pub fn new() -> Self { ONB { u: Vec3A::ZERO, v: Vec3A::ZERO, w: Vec3A::ZERO } }
    pub fn local_vec(&self, a: &Vec3A) -> Vec3A { self.u * a.x + self.v * a.y + self.w * a.z }
    pub fn build_from_w(&mut self, n: &Vec3A) {
        self.w = n.normalize();
        let a: Vec3A = if self.w.x.abs() > 0.9 { Vec3A::new(0.0, 1.0, 0.0) } else { Vec3A::new(1.0, 0.0, 0.0) };
        self.v = self.w.cross(a).normalize();
        self.u = self.w.cross(self.v);
    }
}
