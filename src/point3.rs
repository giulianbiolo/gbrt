use crate::vec3;

impl vec3::Point3 {
    pub fn distance_squared(&self, other: &vec3::Point3) -> f32 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        let dz = self.z() - other.z();
        dx * dx + dy * dy + dz * dz
    }
    pub fn distance(&self, other: &vec3::Point3) -> f32 { self.distance_squared(other).sqrt() }
    pub fn to_vec3(&self) -> vec3::Vec3 { vec3::Vec3::new(self.x(), self.y(), self.z()) }
    pub fn to_point3(&self) -> vec3::Point3 { vec3::Point3::new(self.x(), self.y(), self.z()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance() -> Result<(), std::fmt::Error> {
        let p1 = vec3::Point3::new(1.0, 2.0, 3.0);
        let p2 = vec3::Point3::new(4.0, 5.0, 6.0);
        // Use a delta to assert equality for f32 errors
        assert!((p1.distance_squared(&p2) - 27.0).abs() < 0.0000001);
        assert!((p1.distance(&p2) - 5.196152).abs() < 0.000001);
        Ok(())
    }
}
