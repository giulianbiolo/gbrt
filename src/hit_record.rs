// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the HitRecord struct

use glam::Vec3A;

use crate::ray::Ray;
use crate::material::{Material, Lambertian};
use crate::color::Color;
use crate::point3::Point3;


#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3A,
    pub mat_ptr: Box<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            mat_ptr: Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false
        }
    }
    pub fn new(p: Point3, normal: Vec3A, mat_ptr: Box<dyn Material>, t: f32, u: f32, v: f32, front_face: bool) -> HitRecord { HitRecord { p, normal, mat_ptr, t, u, v, front_face } }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3A) {
        // This function is used to determine whether the ray is inside or outside the object.
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
    pub fn reset(&mut self) {
        self.p = Point3::new(0.0, 0.0, 0.0);
        self.normal = Vec3A::new(0.0, 0.0, 0.0);
        self.mat_ptr = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        self.t = 0.0;
        self.front_face = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utility::EPSILON;

    #[test]
    fn test_hitrecord() -> Result<(), std::fmt::Error> {
        let mut hit_record: HitRecord = HitRecord::empty();
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, 1.0));
        let outward_normal: Vec3A = Vec3A::new(0.0, 0.0, 1.0);
        hit_record.set_face_normal(&ray, &outward_normal);
        assert!((hit_record.normal - Vec3A::new(0.0, 0.0, -1.0)).length() <= EPSILON);
        assert_eq!(hit_record.front_face, false);
        Ok(())
    }
}
