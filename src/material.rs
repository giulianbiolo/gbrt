use dyn_clone::DynClone;

use crate::vec3::{Color, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;


pub trait Material: DynClone + Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
dyn_clone::clone_trait_object!(Material);

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian { Lambertian { albedo } }
}
impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() { let scatter_direction: Vec3 = rec.normal; }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = ray_in.direction().unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}