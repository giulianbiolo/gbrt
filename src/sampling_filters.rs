// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 22/02/2023
// Description: This file implements various Filters used to generate rays to sample each pixel of the image

use crate::utility::NEAR_ZERO;


pub trait Filter : std::fmt::Display { fn sample(&self, x: f32) -> f32; }

/*********************** Uniform Filter ***********************/
pub struct UniformFilter { }
impl UniformFilter { pub fn new() -> Self { UniformFilter { } } }
impl Filter for UniformFilter { fn sample(&self, x: f32) -> f32 { x } }
impl std::fmt::Display for UniformFilter { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "UniformFilter {{ }}") } }

/*********************** Tent Filter ***********************/
pub struct TentFilter { }
impl TentFilter { pub fn new() -> Self { TentFilter { } } }
impl Filter for TentFilter { fn sample(&self, x: f32) -> f32 { if x < 0.5 { 2.0 * x } else { 2.0 * (1.0 - x) } } }
impl std::fmt::Display for TentFilter { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "TentFilter {{ }}") } }

/*********************** Lanczos Filter ***********************/
pub struct LanczosFilter { }
impl LanczosFilter { pub fn new() -> Self { LanczosFilter { } } }
impl Filter for LanczosFilter {
    fn sample(&self, x: f32) -> f32 {
        let x: f32 = 2.0 * x - 1.0;
        if x < NEAR_ZERO { 1.0 }
        else if x < 2.0 {
            let pi_x: f32 = std::f32::consts::PI * x;
            let pi_x_2: f32 = std::f32::consts::PI * x / 2.0;
            pi_x.sin() * pi_x_2.sin() / (pi_x * pi_x / 2.0)
        } else { 0.0 }
    }
}
impl std::fmt::Display for LanczosFilter { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "LanczosFilter {{ }}") } }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utility::random_f32;

    #[test]
    fn test_uniform_filter() {
        let filter: UniformFilter = UniformFilter::new();
        assert_eq!(filter.sample(0.0), 0.0);
        assert_eq!(filter.sample(0.5), 0.5);
        assert_eq!(filter.sample(1.0), 1.0);
    }
    #[test]
    fn test_tent_filter() {
        let filter: TentFilter = TentFilter::new();
        assert_eq!(filter.sample(0.0), 0.0);
        assert_eq!(filter.sample(0.5), 1.0);
        assert_eq!(filter.sample(1.0), 0.0);
    }
    #[test]
    fn test_lanczos_filter() {
        let filter: LanczosFilter = LanczosFilter::new();
        for _ in 0..100 {
            let x: f32 = random_f32();
            let y: f32 = filter.sample(x);
            assert!(y >= 0.0 && y <= 1.0);
        }
    }
}