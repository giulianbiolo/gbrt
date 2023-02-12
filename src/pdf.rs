use crate::utility;
use crate::onb::ONB;
use crate::hittable_list::Hittable;

use glam;
use glam::Vec3A;


pub trait PDF {
    fn value(&self, direction: Vec3A, lights: &Box<dyn Hittable + Send + Sync>) -> f32;
    fn generate(&self, lights: &Box<dyn Hittable + Send + Sync>) -> Vec3A;
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
    fn value(&self, direction: Vec3A, _: &Box<dyn Hittable + Send + Sync>) -> f32 {
        let cosine: f32 = direction.normalize().dot(self.uvw.w());
        if cosine < 0.0 { 0.0 } else { cosine / std::f32::consts::PI }
    }
    fn generate(&self, _: &Box<dyn Hittable + Send + Sync>) -> Vec3A { self.uvw.local_vec(utility::random_cosine_direction()) }
}

/*********************** Hittable PDF ***********************/
pub struct HittablePDF {
    origin: Vec3A,
}

impl HittablePDF {
    pub fn new(origin: Vec3A) -> HittablePDF { HittablePDF { origin } }
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vec3A, lights: &Box<dyn Hittable + Send + Sync>) -> f32 { lights.pdf_value(&self.origin, &direction) }
    fn generate(&self, lights: &Box<dyn Hittable + Send + Sync>) -> Vec3A { lights.random(&self.origin) }
}

/*********************** Mixture PDF ***********************/
pub struct MixturePDF {
    p0: Box<dyn PDF + Send + Sync>,
    p1: Box<dyn PDF + Send + Sync>,
}

impl MixturePDF {
    pub fn new(p0: Box<dyn PDF + Send + Sync>, p1: Box<dyn PDF + Send + Sync>) -> MixturePDF { MixturePDF { p0, p1 } }
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vec3A, lights: &Box<dyn Hittable + Send + Sync>) -> f32 { 0.5 * self.p0.value(direction, lights) + 0.5 * self.p1.value(direction, lights) }
    fn generate(&self, lights: &Box<dyn Hittable + Send + Sync>) -> Vec3A {
        if utility::random_f32() < 0.5 { self.p0.generate(lights) } else { self.p1.generate(lights) }
    }
}
