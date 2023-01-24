// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the HittableList struct
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList { HittableList { objects: Vec::new() } }
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, object: Box<dyn Hittable>) { self.objects.push(object); }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::empty();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                // Here we update the closest_so_far value.
                // This triggered the borrow checker error without implementing the Copy trait to HitRecord.
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::vec3::Point3;
    use crate::sphere::Sphere;
    use crate::hittable::Hittable;

    #[test]
    fn test_hit() {
        let mut world: HittableList = HittableList::new();
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        let r: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, -1.0));
        let mut rec: HitRecord = HitRecord::empty();
        assert!(world.hit(&r, 0.0, 100.0, &mut rec));
    }
}
