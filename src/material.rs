use crate::color::Color;
use crate::object::{Collision, Facing};
use crate::ray::Ray;
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
}

#[enum_dispatch(Material)]
pub trait Deflect {
    fn deflect(&self, ray: &Ray, hit: &Collision) -> Option<Deflection>;
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

#[derive(Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
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

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
