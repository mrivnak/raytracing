use crate::ray::Ray;
use crate::vector::{Point, Vector};
use enum_dispatch::enum_dispatch;
use std::ops::Range;

pub enum Facing {
    Inward,
    // equivalent to true in the book
    Outward,
}

pub struct HitRecord {
    pub point: Point,
    pub normal: Point,
    pub t: f32,
    pub facing: Facing,
}

pub fn set_facing(ray: &Ray, normal: Vector) -> (Vector, Facing) {
    match ray.direction.dot(&normal) < 0.0 {
        true => (normal, Facing::Inward),
        false => (-normal, Facing::Outward),
    }
}

#[enum_dispatch]
pub enum Object {
    Sphere,
    Collection,
}

#[enum_dispatch(Object)]
pub trait Hit {
    fn hit(&self, ray: &Ray, t: Range<f32>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if !t.contains(&root) {
            root = (-half_b + sqrt_d) / a;
            if !t.contains(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let (normal, facing) = set_facing(ray, normal);

        Some(HitRecord {
            point,
            normal,
            t,
            facing,
        })
    }
}

pub struct Collection {
    pub objects: Vec<Object>,
}

impl Hit for Collection {
    fn hit(&self, ray: &Ray, t: Range<f32>) -> Option<HitRecord> {
        let mut closest = t.end;
        let mut record = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t.clone()) {
                if hit.t < closest {
                    closest = hit.t;
                    record = Some(hit);
                }
            }
        }

        record
    }
}
