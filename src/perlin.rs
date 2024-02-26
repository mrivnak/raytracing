use rand::Rng;
use crate::vector::{Point, Vector};

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vector>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;
    pub fn new() -> Self {
        let ranvec = (0..Self::POINT_COUNT).map(|_| Vector::random_with_range(-1.0..1.0).normalize()).collect();
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: &Point) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[Vector::ZERO; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize] ^
                                                  self.perm_y[((j + dj as i32) & 255) as usize] ^
                                                  self.perm_z[((k + dk as i32) & 255) as usize]) as usize];
                }
            }
        }

        Self::perlin_interpolation(c, u, v, w)
    }

    fn perlin_interpolation(c: [[[Vector; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vector::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                             (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                             (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) *
                             c[i][j][k].dot(&weight_v);
                }
            }
        }

        accum
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(Self::POINT_COUNT);
        for i in 0..Self::POINT_COUNT {
            p.push(i as i32);
        }
        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut Vec<i32>) {
        let mut rng = rand::thread_rng();
        for i in (0..Self::POINT_COUNT).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }
}