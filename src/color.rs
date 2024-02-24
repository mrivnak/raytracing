use crate::vector::Vector;

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

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
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

        // Clamp
        r = r.clamp(0.0, 1.0);
        g = g.clamp(0.0, 1.0);
        b = b.clamp(0.0, 1.0);

        Color { r, g, b }
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
