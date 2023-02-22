// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 22/02/2023
// Description: This file implements various Filters used to generate rays to sample each pixel of the image

use crate::utility::CONSTS;
use crate::utility::NEAR_ZERO;
use crate::utility::random_f32;

pub trait Filter : std::fmt::Display {
    fn sample(&self) -> f32;
}

/*********************** Uniform Filter ***********************/
pub struct UniformFilter { }
impl UniformFilter {
    pub fn new() -> Self {
        UniformFilter { }
    }
}
impl Filter for UniformFilter {
    fn sample(&self) -> f32 { random_f32() }
}
impl std::fmt::Display for UniformFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniformFilter {{ }}")
    }
}

/*********************** Tent Filter ***********************/
pub struct TentFilter {
    radius: f32,
    inv_radius: f32,
    num_samples: usize,
}
impl TentFilter {
    pub fn new() -> Self {
        let radius: f32 = 0.5;
        TentFilter {
            radius,
            inv_radius: 1.0 / radius,
            num_samples: CONSTS.samples_per_pixel as usize,
        }
    }
}
impl Filter for TentFilter {
    fn sample(&self) -> f32 {
        // This function returns a random number in the range [-1, 1] following a tent distribution
        let x: f32 = random_f32();
        if x < 0.5 { 2.0 * x } else { 2.0 * (1.0 - x) }
    }
}
impl std::fmt::Display for TentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TentFilter {{ radius: {}, inv_radius: {}, num_samples: {} }}", self.radius, self.inv_radius, self.num_samples)
    }
}

/*********************** Lanczos Filter ***********************/
pub struct LanczosFilter {
    radius: f32,
    inv_radius: f32,
    num_samples: usize,
}
impl LanczosFilter {
    pub fn new() -> Self {
        let radius: f32 = 2.0;
        LanczosFilter {
            radius,
            inv_radius: 1.0 / radius,
            num_samples: CONSTS.samples_per_pixel as usize,
        }
    }
}
impl Filter for LanczosFilter {
    fn sample(&self) -> f32 {
        // This function returns a random number in the range [-1, 1] following a Lanczos distribution
        let x: f32 = 2.0 * random_f32() - 1.0;
        if x < NEAR_ZERO { 1.0 }
        else if x < 2.0 {
            let pi_x: f32 = std::f32::consts::PI * x;
            let pi_x_2: f32 = std::f32::consts::PI * x / 2.0;
            pi_x.sin() * pi_x_2.sin() / (pi_x * pi_x / 2.0)
        } else { 0.0 }
    }
}
impl std::fmt::Display for LanczosFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LanczosFilter {{ radius: {}, inv_radius: {}, num_samples: {} }}", self.radius, self.inv_radius, self.num_samples)
    }
}
