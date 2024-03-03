use crate::vector::Vector;
use rand::Rng;
use std::ops::Range;

pub trait GammaCorrect {
    fn gamma_correct(self) -> Self;
}

pub trait Clamp {
    fn clamp(self, min: f64, max: f64) -> Self;
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
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
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Self {
        let h = h * 6.0;
        let i = h.floor() as i32;
        let f = h - i as f64;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        match i {
            0 => Self::new(v, t, p),
            1 => Self::new(q, v, p),
            2 => Self::new(p, v, t),
            3 => Self::new(p, q, v),
            4 => Self::new(t, p, v),
            _ => Self::new(v, p, q),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        const INV_PHI: f64 = 1.0 / std::f64::consts::PHI;
        const SATURATION: f64 = 0.75;
        const VALUE: f64 = 0.95;

        // generate hsv
        let h = rng.gen::<f64>();
        let h = h + INV_PHI;
        let h = h % 1.0;

        Self::hsv_to_rgb(h, SATURATION, VALUE)
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
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        ]
    }
}

impl From<[u8; 3]> for Color {
    fn from(slice: [u8; 3]) -> Color {
        Color {
            r: slice[0] as f64 / 255.0,
            g: slice[1] as f64 / 255.0,
            b: slice[2] as f64 / 255.0,
        }
    }
}

impl From<&[u8]> for Color {
    fn from(slice: &[u8]) -> Color {
        debug_assert_eq!(slice.len(), 3, "Slice must be of length 3");
        Color {
            r: slice[0] as f64 / 255.0,
            g: slice[1] as f64 / 255.0,
            b: slice[2] as f64 / 255.0,
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

        Color { r, g, b }
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

impl std::ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl std::ops::Div<Color> for f64 {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_new() {
        let color = Color::new(0.1, 0.2, 0.3);
        assert_eq!(color.r, 0.1);
        assert_eq!(color.g, 0.2);
        assert_eq!(color.b, 0.3);
    }

    #[test_case(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)] // black
    #[test_case(0.0, 0.0, 1.0, 1.0, 1.0, 1.0)] // white
    #[test_case(0.0, 1.0, 1.0, 1.0, 0.0, 0.0)] // red
    #[test_case(0.5, 1.0, 1.0, 0.0, 1.0, 1.0)] // cyan
    #[test_case(0.0, 0.0, 0.75, 0.75, 0.75, 0.75)] // light gray
    fn test_hsv_to_rgb(h: f64, s: f64, v: f64, r: f64, g: f64, b: f64) {
        let color = Color::hsv_to_rgb(h, s, v);
        assert_eq!(color, Color::new(r, g, b));
    }

    #[test]
    fn test_from_color_for_u8_array() {
        let color = Color::new(0.0, 0.5, 1.0);
        let u8_array: [u8; 3] = color.into();
        assert_eq!(u8_array, [0, 127, 255]);
    }

    #[test]
    fn test_from_u8_array_for_color() {
        let u8_array = [0, 51, 255];
        let color: Color = u8_array.into();
        assert_eq!(color, Color::new(0.0, 0.2, 1.0));
    }

    #[test]
    fn test_from_u8_slice_for_color() {
        let slice: &[u8] = &[0, 51, 255];
        let color: Color = slice.into();
        assert_eq!(color, Color::new(0.0, 0.2, 1.0));
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_from_invalid_slice_for_color() {
        let slice: &[u8] = &[0, 51, 255, 1];
        let _: Color = slice.into();
    }

    #[test]
    fn test_from_vector_for_color() {
        let vector = Vector::new(0.0, 0.5, 1.0);
        let color: Color = vector.into();
        assert_eq!(color, Color::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn test_from_vec_of_colors_for_color() {
        let colors = vec![Color::new(0.0, 0.5, 1.0), Color::new(0.5, 1.0, 1.0)];
        let color: Color = colors.into();
        assert_eq!(color, Color::new(0.25, 0.75, 1.0));
    }

    #[test]
    fn test_gamma_correct() {
        let color = Color::new(0.0, 0.25, 1.0);
        let gamma_corrected = color.gamma_correct();
        assert_eq!(gamma_corrected, Color::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn test_clamp() {
        let color = Color::new(0.0, 0.25, 1.25);
        let clamped = color.clamp(0.5, 1.0);
        assert_eq!(clamped, Color::new(0.5, 0.5, 1.0));
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(0.0, 0.25, 1.0);
        let color2 = Color::new(0.5, 0.75, 1.0);
        let added = color1 + color2;
        assert_eq!(added, Color::new(0.5, 1.0, 2.0));
    }

    #[test]
    fn test_mul() {
        let color1 = Color::new(0.0, 0.25, 1.0);
        let color2 = Color::new(0.5, 0.75, 1.0);
        let multiplied = color1 * color2;
        assert_eq!(multiplied, Color::new(0.0, 0.1875, 1.0));
    }

    #[test]
    fn test_mul_f64_for_color() {
        let color = Color::new(0.0, 0.25, 1.0);
        let multiplied = 2.0 * color;
        assert_eq!(multiplied, Color::new(0.0, 0.5, 2.0));
    }

    #[test]
    fn test_mul_color_for_f64() {
        let color = Color::new(0.0, 0.25, 1.0);
        let multiplied = color * 2.0;
        assert_eq!(multiplied, Color::new(0.0, 0.5, 2.0));
    }

    #[test]
    fn test_div_f64_for_color() {
        let color = Color::new(0.0, 0.25, 1.0);
        let divided = color / 2.0;
        assert_eq!(divided, Color::new(0.0, 0.125, 0.5));
    }

    #[test]
    fn test_div_color_for_f64() {
        let color = Color::new(0.1, 0.25, 1.0);
        let divided = 2.0 / color;
        assert_eq!(divided, Color::new(20.0, 8.0, 2.0));
    }
}
