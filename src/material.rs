use crate::color::Color;
use crate::object::Collision;
use crate::ray::Ray;
use crate::vector::Vector;
use enum_dispatch::enum_dispatch;

pub struct Reflection {
    pub attenuation: Color,
    pub ray: Ray,
}

#[enum_dispatch]
pub enum Material {
    Lambertian,
    Metal,
}

#[enum_dispatch(Material)]
pub trait Reflect {
    fn reflect(&self, ray: &Ray, hit: &Collision) -> Option<Reflection>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Reflect for Lambertian {
    fn reflect(&self, _ray: &Ray, hit: &Collision) -> Option<Reflection> {
        let mut scatter_direction = hit.normal + Vector::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        Some(Reflection {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Reflect for Metal {
    fn reflect(&self, ray: &Ray, hit: &Collision) -> Option<Reflection> {
        let reflected = ray.direction.normalize().reflect(&hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected,
        };
        Some(Reflection {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}
