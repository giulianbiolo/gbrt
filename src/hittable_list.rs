// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the HittableList struct and the Hittable trait

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use glam;
use glam::Vec3A;


pub trait Hittable: Sync + Send {
    // The hit function returns true if the ray hits the object.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn pdf_value(&self, origin: &Vec3A, v: &Vec3A) -> f32;
    fn random(&self, origin: &Vec3A) -> Vec3A;
    fn is_light(&self) -> bool;
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
    fn is_light(&self) -> bool {
        for object in self {
            if object.is_light() { return true; }
        }
        false
    }
    fn pdf_value(&self, _origin: &Vec3A, _v: &Vec3A) -> f32 { 0.0 }
    fn random(&self, _origin: &Vec3A) -> Vec3A { Vec3A::ZERO }
}
pub fn get_lights(world: &HittableList) -> Vec<&Box<dyn Hittable + Sync + Send>> {
    world.iter().filter(|object| object.is_light()).collect()
}

pub fn get_light(world: &HittableList) -> &Box<dyn Hittable + Sync + Send> { get_lights(world).first().unwrap() }

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
