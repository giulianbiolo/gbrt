// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Camera struct

use glam::Vec3A;

use crate::utility;
use crate::ray::Ray;
use crate::point3::Point3;
use crate::parser;


#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3A,
    vertical: Vec3A,
    u: Vec3A,
    v: Vec3A,
    //w: Vec3A,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: &Vec3A, lookat: &Vec3A, vup: &Vec3A, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta: f32 = f32::to_radians(vfov);
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vec3A = (*lookfrom - *lookat).normalize();
        let u: Vec3A = vup.cross(w).normalize();
        let v: Vec3A = w.cross(u);

        let origin: Point3 = *lookfrom;
        let horizontal: Vec3A = u * focus_dist * viewport_width;
        let vertical: Vec3A = v * focus_dist * viewport_height;
        let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius: f32 = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            //w,
            lens_radius,
        }
    }
    pub fn new_from_yaml(filename: &str) -> Camera { parser::parse_yaml_camera(filename) }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3A = utility::random_in_unit_disk() * self.lens_radius;
        let offset: Vec3A = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera() -> Result<(), std::fmt::Error> {
        let camera: Camera = Camera::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(0.0, 0.0, -1.0),
            &Vec3A::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
            0.0,
            1.0
        );
        let ray: Ray = camera.get_ray(0.0, 0.0);
        assert_eq!(camera.origin, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.origin(), Point3::new(0.0, 0.0, 0.0));
        Ok(())
    }
}
