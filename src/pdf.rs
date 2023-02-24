// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the PDF (Probability Density Function) trait

use std::sync::Arc;

use glam::Vec3A;

use crate::onb::ONB;
use crate::utility;
use crate::hittable_list::Hittable;
use crate::point3::Point3;

pub trait PDF {
    fn value(&self, direction: &Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}

#[derive(Clone)]
pub struct CosinePDF {
    pub uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3A) -> Self {
        let mut uvw = ONB::new();
        uvw.build_from_w(w);
        CosinePDF { uvw }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3A) -> f32 {
        let cosine = direction.normalize().dot(self.uvw.w);
        if cosine > 0.0 { cosine / utility::PI } else { 0.0 }
    }
    fn generate(&self) -> Vec3A { self.uvw.local_vec(&utility::random_cosine_direction()) }
}

#[derive(Clone)]
pub struct HittablePDF {
    pub origin: Point3,
    pub ptr: Arc<dyn Hittable>,
}

/*
impl HittablePDF { pub fn new(origin: Point3, ptr: Arc<dyn Hittable>) -> Self { HittablePDF { origin, ptr } } }

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3A) -> f32 { self.ptr.pdf_value(&self.origin, &direction) }
    fn generate(&self) -> Vec3A { self.ptr.random(&self.origin) }
}

#[derive(Clone)]
pub struct MixturePDF {
    pub p0: Arc<dyn PDF>,
    pub p1: Arc<dyn PDF>,
}

impl MixturePDF { pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self { MixturePDF { p0, p1 } } }

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3A) -> f32 { 0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction) }
    fn generate(&self) -> Vec3A { if utility::random_f32() < 0.5 { self.p0.generate() } else { self.p1.generate() } }
}

// Example of usage in the ray_color function:
// For only HittablePDF:
//let light_pdf: HittablePDF = HittablePDF::new(rec.p, Arc::new(lights.clone()));
//let mut scattered: Ray = Ray::new(rec.p, light_pdf.generate().normalize());
//let pdf: f32 = light_pdf.value(&scattered.direction());

// For MixturePDF:
//let mixture_pdf: MixturePDF = MixturePDF::new(srec.pdf_ptr.unwrap(), Arc::new(light_pdf));
//let mut scattered: Ray = Ray::new(rec.p, mixture_pdf.generate().normalize());
//let pdf: f32 = mixture_pdf.value(&scattered.direction());

*/