use crate::color::Color;
use crate::object::{Collision, Facing};
use crate::ray::Ray;
use crate::texture::{ColorAt, Texture};
use crate::vector::Vector;
use enum_dispatch::enum_dispatch;

pub struct Deflection {
    pub attenuation: Color,
    pub ray: Ray,
}

#[enum_dispatch]
#[derive(Clone)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    Simple,
    Light,
}

#[enum_dispatch(Material)]
pub trait Deflect {
    fn deflect(&self, ray: &Ray, hit: &Collision) -> Option<Deflection>;
}

#[enum_dispatch(Material)]
pub trait Emit {
    fn emit(&self, _u: f64, _v: f64, _point: &Vector) -> Color {
        Color::BLACK
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Deflect for Lambertian {
    fn deflect(&self, _ray: &Ray, hit: &Collision) -> Option<Deflection> {
        let mut scatter_direction = hit.normal + Vector::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        Some(Deflection {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}

impl Emit for Lambertian {}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Deflect for Metal {
    fn deflect(&self, ray: &Ray, hit: &Collision) -> Option<Deflection> {
        let reflected = ray.direction.normalize().reflect(&hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vector::random_unit_vector(),
        };
        Some(Deflection {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}

impl Emit for Metal {}

#[derive(Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
    // TODO: add fuzz
}

impl Deflect for Dielectric {
    fn deflect(&self, ray: &Ray, hit: &Collision) -> Option<Deflection> {
        let attenuation = Color::WHITE;
        let refraction_ratio = match hit.facing {
            Facing::Inward => 1.0 / self.refraction_index,
            Facing::Outward => self.refraction_index,
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let deflected =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand::random() {
                unit_direction.reflect(&hit.normal)
            } else {
                unit_direction.refract(&hit.normal, refraction_ratio)
            };

        let scattered = Ray {
            origin: hit.point,
            direction: deflected,
        };

        Some(Deflection {
            attenuation,
            ray: scattered,
        })
    }
}

impl Emit for Dielectric {}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[derive(Clone)]
pub struct Simple {
    pub texture: Texture,
}

impl Deflect for Simple {
    fn deflect(&self, _ray: &Ray, hit: &Collision) -> Option<Deflection> {
        let mut scatter_direction = hit.normal + Vector::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        Some(Deflection {
            attenuation: self.texture.color_at(hit.u, hit.v, &hit.point),
            ray: scattered,
        })
    }
}

impl Emit for Simple {}

#[derive(Clone)]
pub struct Light {
    pub color: Color,
}

impl Deflect for Light {
    fn deflect(&self, _ray: &Ray, _hit: &Collision) -> Option<Deflection> {
        None
    }
}

impl Emit for Light {
    fn emit(&self, _u: f64, _v: f64, _point: &Vector) -> Color {
        self.color
    }
}
