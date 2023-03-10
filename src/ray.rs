// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Ray struct

use glam::Vec3A;

use crate::point3::Point3;


pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3A,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3A) -> Ray { Ray { origin, direction } }
    pub fn empty() -> Ray { Ray { origin: Point3::new(0.0, 0.0, 0.0), direction: Vec3A::new(0.0, 0.0, 0.0) } }
    pub fn origin(&self) -> Point3 { self.origin }
    pub fn direction(&self) -> Vec3A { self.direction }
    pub fn at(&self, t: f32) -> Point3 { self.origin + self.direction * t }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() -> Result<(), std::fmt::Error> {
        let origin: Point3 = Point3::new(2.0, 3.0, 4.0);
        let direction: Vec3A = Vec3A::new(1.0, 0.0, 0.0);
        let ray: Ray = Ray::new(origin, direction);
        assert_eq!(ray.at(0.0), origin);
        assert_eq!(ray.at(1.0), Point3::new(3.0, 3.0, 4.0));
        assert_eq!(ray.at(-1.0), Point3::new(1.0, 3.0, 4.0));
        assert_eq!(ray.at(2.5), Point3::new(4.5, 3.0, 4.0));
        Ok(())
    }
}
