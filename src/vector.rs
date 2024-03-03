use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vector;

impl Vector {
    pub const ZERO: Vector = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    pub fn random_with_range(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        // TODO: Rejection sampling is slow, use a better method
        // should be better to generate a random angle and figure out where it lands on the sphere
        // since this vector is currently always normalized afterward, that would remove the necessity for that step
        loop {
            let p = Self::random_with_range(-1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().normalize()
    }

    pub fn random_in_hemisphere(normal: &Vector) -> Self {
        let in_unit_sphere = Self::random_unit_vector();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        // TODO: same as for random_in_unit_sphere
        let mut rng = rand::thread_rng();
        loop {
            let p = Self {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
                z: 0.0,
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn reflect(self, normal: &Vector) -> Self {
        self - (2.0 * self.dot(normal) * *normal)
    }

    pub fn refract(self, normal: &Vector, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }

    pub fn is_near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a.cross(&b), Vector::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_length() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_length_squared() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a.length_squared(), 14.0);
    }

    #[test]
    fn test_normalize() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let normalized = a.normalize();
        assert_eq!(
            normalized,
            Vector::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
        assert_eq!(normalized.length(), 1.0);
    }

    #[test]
    fn test_random_with_range() {
        for _ in 0..100 {
            let a = Vector::random_with_range(0.0..1.0);
            assert!(a.x >= 0.0 && a.x < 1.0);
            assert!(a.y >= 0.0 && a.y < 1.0);
            assert!(a.z >= 0.0 && a.z < 1.0);
        }
    }

    #[test]
    fn test_random_in_unit_sphere() {
        for _ in 0..100 {
            let a = Vector::random_in_unit_sphere();
            assert!(a.length() < 1.0);
        }
    }

    #[test]
    fn test_random_unit_vector() {
        for _ in 0..100 {
            let a = Vector::random_unit_vector();
            assert!(
                (a.length() - 1.0).abs() < 1e-8,
                "a.length(): {:?}",
                a.length()
            );
        }
    }

    #[test]
    fn test_random_in_hemisphere() {
        for _ in 0..100 {
            let normal = Vector::new(0.0, 0.0, 1.0);
            let a = Vector::random_in_hemisphere(&normal);
            assert!(a.dot(&normal) > 0.0);
        }
    }

    #[test]
    fn test_random_in_unit_disk() {
        for _ in 0..100 {
            let a = Vector::random_in_unit_disk();
            assert!(a.length() < 1.0);
            assert_eq!(a.z, 0.0);
        }
    }

    #[test]
    fn test_reflect() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(&n), Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_is_near_zero() {
        let a = Vector::new(1e-9, 1e-9, 1e-9);
        assert!(a.is_near_zero());
    }

    #[test]
    fn test_add() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vector::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a - b, Vector::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_mul_f64_for_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a * 2.0, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vector_for_f64() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(2.0 * a, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_f64_for_vector() {
        let a = Vector::new(2.0, 4.0, 6.0);
        assert_eq!(a / 2.0, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_mul_assign_f64_for_vector() {
        let mut a = Vector::new(1.0, 2.0, 3.0);
        a *= 2.0;
        assert_eq!(a, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_neg() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(-a, Vector::new(-1.0, -2.0, -3.0));
    }
}
