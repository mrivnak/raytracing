use std::error::Error;
use std::path::PathBuf;
use enum_dispatch::enum_dispatch;
use crate::color::Color;
use crate::perlin::Perlin;
use crate::vector::Point;

#[enum_dispatch]
#[derive(Clone)]
pub enum Texture {
    Solid,
    Checker,
    Image,
    Noise
}

#[enum_dispatch(Texture)]
pub trait ColorAt {
    fn color_at(&self, u: f64, v: f64, point: &Point) -> Color;
}

#[derive(Clone)]
pub struct Solid {
    pub color: Color,
}

impl ColorAt for Solid {
    fn color_at(&self, _u: f64, _v: f64, _point: &Point) -> Color {
        self.color
    }
}

#[derive(Clone)]
pub struct Checker {
    even: Color,
    odd: Color,
    inverse_scale: f64,
}

impl Checker {
    pub fn new(even: Color, odd: Color, scale: f64) -> Checker {
        Checker {
            even,
            odd,
            inverse_scale: 1.0 / scale,
        }
    }
}

impl ColorAt for Checker {
    fn color_at(&self, _u: f64, _v: f64, point: &Point) -> Color {
        let x = (self.inverse_scale * point.x).floor() as i32;
        let y = (self.inverse_scale * point.y).floor() as i32;
        let z = (self.inverse_scale * point.z).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;
        if is_even {
            self.even
        } else {
            self.odd
        }
    }
}

#[derive(Clone)]
pub struct Image {
    pub data: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
struct TextureError;

impl Error for TextureError {
    fn description(&self) -> &str {
        "Invalid texture"
    }
}

impl std::fmt::Display for TextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid texture")
    }
}

impl Image {
    pub fn load(path: PathBuf) -> Result<Image, Box<dyn Error>> {
        let img = image::open(path.clone())?;
        let data = img.to_rgb8().into_raw().chunks_exact(3).map(Color::from).collect();
        let width = img.width();
        let height = img.height();
        if width == 0 || height == 0 {
            return Err(Box::new(TextureError));
        }
        Ok(Image { data, width, height })
    }
}

impl Default for Image {
    fn default() -> Image {
        const GRID_SIZE: u32 = 10;

        let mut data = Vec::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let color = if (i + j) % 2 == 0 { Color::BLACK } else { Color::MAGENTA };
                data.push(color);
            }
        }

        Image {
            data,
            width: GRID_SIZE,
            height: GRID_SIZE,
        }
    }
}

impl ColorAt for Image {
    fn color_at(&self, u: f64, v: f64, _point: &Point) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let x = (u * self.width as f64) as u32;
        let y = (v * self.height as f64) as u32;

        let index = (y * self.width + x) as usize;
        self.data[index]
    }
}

#[derive(Clone)]
pub struct Noise {
    perlin: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Noise {
        Noise {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl ColorAt for Noise {
    fn color_at(&self, _u: f64, _v: f64, point: &Point) -> Color {
        let s = *point * self.scale;
        Color::new(1.0, 1.0, 1.0) * self.perlin.turbulence(&s, None)
    }
}
