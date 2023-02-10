// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Point3 alias

use glam::Vec3A;
pub type Point3 = Vec3A;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<(), std::fmt::Error> {
        let p: Point3 = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p, Vec3A::new(1.0, 2.0, 3.0));
        Ok(())
    }
}
