use crate::utility;
use crate::onb::ONB;
use crate::hittable_list::Hittable;

use glam;
use glam::Vec3A;


pub trait PDF {
    fn value(&self, direction: Vec3A) -> f32;
    fn generate(&self) -> Vec3A;
}

/*********************** Cosine PDF ***********************/
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3A) -> CosinePDF {
        let mut uvw: ONB = ONB::new();
        uvw.build_from_w(w);
        CosinePDF { uvw }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3A) -> f32 {
        let cosine: f32 = direction.normalize().dot(self.uvw.w());
        if cosine < 0.0 { 0.0 } else { cosine / std::f32::consts::PI }
    }
    fn generate(&self) -> Vec3A { self.uvw.local_vec(utility::random_cosine_direction()) }
}

/*********************** Hittable PDF ***********************/
pub struct HittablePDF<'a> {
    o: Vec3A,
    ptr: &'a Box<dyn Hittable + Send + Sync>,
}

impl <'a> HittablePDF<'a> {
    pub fn new(o: Vec3A, ptr: &'a Box<dyn Hittable + Send + Sync>) -> HittablePDF<'a> {
        HittablePDF { o, ptr }
    }
}

impl <'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: Vec3A) -> f32 { self.ptr.pdf_value(&self.o, &direction) }
    fn generate(&self) -> Vec3A { self.ptr.random(&self.o) }
}

pub fn hittable_pdf_generate(p: Vec3A, ptr: &Box<dyn Hittable + Send + Sync>) -> Vec3A { ptr.random(&p) }
pub fn hittable_pdf_value(p: Vec3A, direction: Vec3A, ptr: &Box<dyn Hittable + Send + Sync>) -> f32 { ptr.pdf_value(&p, &direction) }
