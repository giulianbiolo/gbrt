// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Camera struct
use crate::vec3::{Point3, Vec3};
use crate::utility::degrees_to_radians;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
        let theta: f32 = degrees_to_radians(vfov);
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vec3 = (*lookfrom - *lookat).unit_vector();
        let u: Vec3 = vup.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u);

        let origin: Point3 = *lookfrom;
        let horizontal: Vec3 = viewport_width * u;
        let vertical: Vec3 = viewport_height * v;
        let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - w;

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
        let camera: Camera = Camera::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, -1.0), &Vec3::new(0.0, 1.0, 0.0), 90.0, 1.0);
        let ray: Ray = camera.get_ray(0.0, 0.0);
        assert_eq!(camera.origin, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.origin(), Point3::new(0.0, 0.0, 0.0));
        Ok(())
    }
}
