// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Vec3 struct, a backbone of various other structs throughout the project
use std::ops;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }
    pub fn new_init() -> Vec3 { Vec3 { x: 0.0, y: 0.0, z: 0.0 } }
    pub fn new_from(v: &Vec3) -> Vec3 { Vec3 { x: v.x, y: v.y, z: v.z } }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }
    pub fn length(&self) -> f32 { self.length_squared().sqrt() }
    pub fn dot(&self, other: &Vec3) -> f32 { self.x * other.x + self.y * other.y + self.z * other.z }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn unit_vector(&self) -> Vec3 {
        let k: f32 = 1.0 / self.length();
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }
}
impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f32) -> Vec3 {
        Vec3::new(self.x + other, self.y + other, self.z + other)
    }
}
impl ops::Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self + other.x, self + other.y, self + other.z)
    }
}
impl ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}
impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f32) -> Vec3 {
        Vec3::new(self.x - other, self.y - other, self.z - other)
    }
}
impl ops::Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self - other.x, self - other.y, self - other.z)
    }
}
impl ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

// I want it to be treatable like an array, so I can say v[1] and get the y value.
impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
// Also want to be able to assign to it like an array.
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
// And want to print it to the console.
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}
// Create aliases for Vec3 (Point3 and Color)
pub type Color = Vec3;
pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() -> Result<(), std::fmt::Error> {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        Ok(())
    }
    #[test]
    fn test_length() -> Result<(), std::fmt::Error> {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length_squared(), 14.0);
        assert_eq!(v.length(), 3.7416573867739413);
        Ok(())
    }
    #[test]
    fn test_dot() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
        Ok(())
    }
    #[test]
    fn test_cross() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), 6.0);
        assert_eq!(v3.z(), -3.0);
        Ok(())
    }
    #[test]
    fn test_unit_vector() -> Result<(), std::fmt::Error> {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v.unit_vector();
        //Assert equality with a small delta because of floating point error.
        assert!(v2.length() - 1.0 < 0.0000001);
        Ok(())
    }
    #[test]
    fn test_add() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);
        Ok(())
    }
    #[test]
    fn test_sub() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), -3.0);
        assert_eq!(v3.z(), -3.0);
        Ok(())
    }
    #[test]
    fn test_mul() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 * v2;
        assert_eq!(v3.x(), 4.0);
        assert_eq!(v3.y(), 10.0);
        assert_eq!(v3.z(), 18.0);
        Ok(())
    }
    #[test]
    fn test_mul_scalar() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 4.0);
        assert_eq!(v2.z(), 6.0);
        Ok(())
    }
    #[test]
    fn test_div()  -> Result<(), std::fmt::Error>{
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2.x(), 0.5);
        assert_eq!(v2.y(), 1.0);
        assert_eq!(v2.z(), 1.5);
        Ok(())
    }
    #[test]
    fn test_neg() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2.x(), -1.0);
        assert_eq!(v2.y(), -2.0);
        assert_eq!(v2.z(), -3.0);
        Ok(())
    }
    #[test]
    fn test_add_assign() -> Result<(), std::fmt::Error> {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x(), 5.0);
        assert_eq!(v1.y(), 7.0);
        assert_eq!(v1.z(), 9.0);
        Ok(())
    }
    #[test]
    fn test_sub_assign() -> Result<(), std::fmt::Error> {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 -= v2;
        assert_eq!(v1.x(), -3.0);
        assert_eq!(v1.y(), -3.0);
        assert_eq!(v1.z(), -3.0);
        Ok(())
    }
    #[test]
    fn test_mul_assign() -> Result<(), std::fmt::Error> {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 *= v2;
        assert_eq!(v1.x(), 4.0);
        assert_eq!(v1.y(), 10.0);
        assert_eq!(v1.z(), 18.0);
        Ok(())
    }
    #[test]
    fn test_mul_assign_scalar() -> Result<(), std::fmt::Error> {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1.x(), 2.0);
        assert_eq!(v1.y(), 4.0);
        assert_eq!(v1.z(), 6.0);
        Ok(())
    }
    #[test]
    fn test_div_assign() -> Result<(), std::fmt::Error> {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1.x(), 0.5);
        assert_eq!(v1.y(), 1.0);
        assert_eq!(v1.z(), 1.5);
        Ok(())
    }
    #[test]
    fn test_index() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1[0], 1.0);
        assert_eq!(v1[1], 2.0);
        assert_eq!(v1[2], 3.0);
        Ok(())
    }
    #[test]
    fn test_index_mut() -> Result<(), std::fmt::Error> {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1[0] = 4.0;
        v1[1] = 5.0;
        v1[2] = 6.0;
        assert_eq!(v1[0], 4.0);
        assert_eq!(v1[1], 5.0);
        assert_eq!(v1[2], 6.0);
        Ok(())
    }
    #[test]
    fn test_display() -> Result<(), std::fmt::Error> {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", v1), "Vec3(1, 2, 3)");
        Ok(())
    }
}
