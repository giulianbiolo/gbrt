// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the HittableList struct and the Hittable trait

use std::sync::Arc;

use glam::Vec3A;

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::point3::Point3;
use crate::utility;


pub trait Hittable: Sync + Send {
    // The hit function returns true if the ray hits the object.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn is_light(&self) -> bool;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3A) -> f32 { 0.0 }
    fn random(&self, _o: &Point3) -> Vec3A { Vec3A::X }
}

pub type HittableList = Vec<Arc<dyn Hittable + Sync + Send>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.iter()
        .filter_map(|object| object.hit(ray, t_min, t_max))
        .filter(|hit| hit.t > t_min && hit.t < t_max)
        .min_by(|hit1, hit2| { hit1.t.partial_cmp(&hit2.t).unwrap() })
    }
    fn is_light(&self) -> bool { false }
    fn pdf_value(&self, origin: &Point3, v: &Vec3A) -> f32 {
        let weight: f32 = 1.0 / self.len() as f32;
        self.iter().map(|object| weight * object.pdf_value(origin, v)).sum()
    }
    fn random(&self, o: &Point3) -> Vec3A { self[utility::random_usize_range(0, self.len())].random(o) }
}

impl std::fmt::Debug for dyn Hittable + Sync + Send {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn Hittable + Sync + Send>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::sphere::Sphere;
    use crate::color::Color;
    use crate::point3::Point3;

    #[test]
    fn test_hit() {
        let mut world: HittableList = HittableList::new();
        world.push(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))), 0)));
        world.push(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))), 0)));
        let r: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, -1.0));
        assert!(world.hit(&r, 0.0, 100.0).is_some());
    }
}
