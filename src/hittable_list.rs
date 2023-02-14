// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the HittableList struct and the Hittable trait

use crate::hit_record::HitRecord;
use crate::ray::Ray;


pub trait Hittable: Sync + Send {
    // The hit function returns true if the ray hits the object.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub type HittableList = Vec<Box<dyn Hittable + Sync + Send>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
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
        world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))), 0)));
        world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))), 0)));
        let r: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, -1.0));
        let mut rec: HitRecord = HitRecord::empty();
        assert!(world.hit(&r, 0.0, 100.0, &mut rec));
    }
}
