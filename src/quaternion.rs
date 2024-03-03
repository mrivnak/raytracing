use crate::vector::{Point, Vector};

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_axis_angle(axis: Vector, angle: f64) -> Self {
        let half_theta = angle / 2.0;
        let sin_half_theta = half_theta.sin();
        let cos_half_theta = half_theta.cos();
        Self {
            x: axis.x * sin_half_theta,
            y: axis.y * sin_half_theta,
            z: axis.z * sin_half_theta,
            w: cos_half_theta,
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn rotate_point(self, point: Point) -> Point {
        let prime = self.inverse() * Quaternion::new(point.x, point.y, point.z, 0.0) * self;
        Point::new(prime.x, prime.y, prime.z)
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;
    use super::*;

    #[test]
    fn test_quaternion_rotate_point() {
        let quat = Quaternion::from_axis_angle(Vector::new(0.0, 1.0, 0.0), 90.0_f64.to_radians());
        let point = Point::new(1.0, 0.0, 0.0);
        let rotated = quat.rotate_point(point);
        let expected = Point::new(0.0, 0.0, 1.0);

        assert!(rotated.x.sub(expected.x).abs() < 1e-8, "x: {} != {}", rotated.x, expected.x);
        assert!(rotated.y.sub(expected.y).abs() < 1e-8, "y: {} != {}", rotated.y, expected.y);
        assert!(rotated.z.sub(expected.z).abs() < 1e-8, "z: {} != {}", rotated.z, expected.z);

        let quat = Quaternion::from_axis_angle(Vector::new(0.0, 1.0, 0.0), 180.0_f64.to_radians());
        let point = Point::new(1.0, 0.0, 0.0);
        let rotated = quat.rotate_point(point);
        let expected = Point::new(-1.0, 0.0, 0.0);

        assert!(rotated.x.sub(expected.x).abs() < 1e-8, "x: {} != {}", rotated.x, expected.x);
        assert!(rotated.y.sub(expected.y).abs() < 1e-8, "y: {} != {}", rotated.y, expected.y);
        assert!(rotated.z.sub(expected.z).abs() < 1e-8, "z: {} != {}", rotated.z, expected.z);
    }
}