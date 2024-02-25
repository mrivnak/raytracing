use std::ops::Range;
use rand::Rng;
use crate::vector::Vector;

trait GammaCorrect {
    fn gamma_correct(self) -> Self;
}

trait Clamp {
    fn clamp(self, min: f64, max: f64) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }

    pub fn random_with_range(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(range.clone()),
            g: rng.gen_range(range.clone()),
            b: rng.gen_range(range),
        }
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> [u8; 3] {
        [
            (color.r * 254.999) as u8,
            (color.g * 254.999) as u8,
            (color.b * 254.999) as u8,
        ]
    }
}

impl From<Vec<Color>> for Color {
    fn from(colors: Vec<Color>) -> Color {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        let samples = colors.len() as f64;

        // Sample average
        for color in colors {
            r += color.r;
            g += color.g;
            b += color.b;
        }
        r /= samples;
        g /= samples;
        b /= samples;

        Color { r, g, b }.gamma_correct().clamp(0.0, 1.0)
    }
}

impl GammaCorrect for Color {
    fn gamma_correct(self) -> Self {
        Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl Clamp for Color {
    fn clamp(self, min: f64, max: f64) -> Self {
        Self {
            r: self.r.clamp(min, max),
            g: self.g.clamp(min, max),
            b: self.b.clamp(min, max),
        }
    }
}

impl From<Vector> for Color {
    fn from(vector: Vector) -> Color {
        Color {
            r: vector.x,
            g: vector.y,
            b: vector.z,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}
