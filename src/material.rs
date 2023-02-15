// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Material trait and its implementations

use likely_stable::unlikely;
use dyn_clone::DynClone;

use glam::Vec3A;

use crate::color::Color;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::utility::{self, NEAR_ZERO};


pub trait Material: DynClone + Send {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
    fn emitted(&self) -> Color { Color::new(0.0, 0.0, 0.0) }
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone, Debug)]
pub struct Lambertian {
    // The Lambertian material is a diffuse material that reflects light equally in all directions.
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian { Lambertian { albedo } }
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        // Scatter direction will be the normal plus a random vector in the unit sphere
        let mut scatter_direction: Vec3A = rec.normal + utility::random_unit_vector();
        // If the scatter direction is too close to zero, we set it to the normal
        if unlikely(scatter_direction.length_squared() < utility::NEAR_ZERO) { scatter_direction = rec.normal; }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo; // The attenuation is the albedo
        true
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    // The Metal material is a shiny material that reflects light in a specular way.
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
        // The scattered ray is the reflected ray plus a random vector in the unit sphere times the fuzz factor
        let reflected: Vec3A = reflect(&ray_in.direction().normalize(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + utility::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo; // The attenuation is the albedo
        // The ray is scattered only if the dot product is positive
        // If the dot product is negative, the ray would be reflected backwards, inside the object
        // If the dot product is zero, the ray would be reflected in the same direction, might result in infinite loops
        scattered.direction().dot(rec.normal) > 0.0
    }
}

#[derive(Clone, Debug)]
pub struct Dielectric {
    // The Dielectric material is a transparent material that refracts light.
    refr_idx: f32,
}
impl Dielectric {
    pub fn new(refr_idx: f32) -> Dielectric { Dielectric { refr_idx } }
    fn reflectance(&self, cos: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0: f32 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0); // The attenuation is white
        let refraction_rate = if rec.front_face { 1.0 / self.refr_idx } else { self.refr_idx };
        let unit_direction: Vec3A = ray_in.direction().normalize();
        
        let cos_theta: f32 = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta: f32 = (1.0 - cos_theta.powi(2)).sqrt();

        let direction: Vec3A;
        if refraction_rate * sin_theta > 1.0 || self.reflectance(cos_theta, refraction_rate) > utility::random_f32() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_rate);
        }
        *scattered = Ray::new(rec.p, direction);
        true
    }
}

fn reflect(vec: &Vec3A, normal: &Vec3A) -> Vec3A { *vec - *normal * vec.dot(*normal) * 2.0 }
fn refract(vec: &Vec3A, normal: &Vec3A, etai_over_etat: f32) -> Vec3A {
    let cos_theta: f32 = (-*vec).dot(*normal).min(1.0);
    let r_out_perp: Vec3A = (*vec + *normal * cos_theta) * etai_over_etat;
    let r_out_parallel: Vec3A = *normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    // The DiffuseLight material is a light source that emits light equally in all directions.
    emit: Color,
}
impl DiffuseLight {
    pub fn new(emit: Color) -> DiffuseLight { DiffuseLight { emit } }
}
impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Color, _: &mut Ray) -> bool { false }
    fn emitted(&self) -> Color { self.emit }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambertian() -> Result<(), std::fmt::Error> {
        let material: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        assert_eq!(material.albedo, Color::new(0.5, 0.5, 0.5));
        Ok(())
    }
    #[test]
    fn test_metal() -> Result<(), std::fmt::Error> {
        let material: Metal = Metal::new(Color::new(0.5, 0.5, 0.5), 0.0);
        assert_eq!(material.albedo, Color::new(0.5, 0.5, 0.5));
        assert_eq!(material.fuzz, 0.0);
        Ok(())
    }
    #[test]
    fn test_dielectric() -> Result<(), std::fmt::Error> {
        let material: Dielectric = Dielectric::new(1.5);
        assert_eq!(material.refr_idx, 1.5);
        Ok(())
    }
    #[test]
    fn test_diffuse_light() -> Result<(), std::fmt::Error> {
        let material: DiffuseLight = DiffuseLight::new(Color::new(0.5, 0.5, 0.5));
        assert_eq!(material.emit, Color::new(0.5, 0.5, 0.5));
        Ok(())
    }
}
