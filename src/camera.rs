// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Camera struct
use crate::vec3::{Point3, Vec3};
use crate::utility;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(utility::VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, utility::VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, utility::FOCAL_LENGTH);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_camera() -> Result<(), std::fmt::Error> {
        let camera: Camera = Camera::new();
        let ray: Ray = camera.get_ray(0.0, 0.0);
        assert_eq!(camera.origin, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.origin(), Point3::new(0.0, 0.0, 0.0));
        Ok(())
    }
}
