use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere { Sphere { center, radius } }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return false; } // No real roots, so no intersection.
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root; // The ray hits the sphere at this value for 't'.
        rec.p = ray.at(rec.t); // The ray hits the sphere at this point 'p'.
        // The outward normal is the vector from the center of the sphere to the point of intersection.
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        // This function is used to determine whether the ray is inside or outside the object.
        rec.set_face_normal(ray, &outward_normal);
        true
    }
}
