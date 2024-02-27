use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point, Vector};
use enum_dispatch::enum_dispatch;
use std::ops::Range;

pub enum Facing {
    Inward,
    // equivalent to true in the book
    Outward,
}

pub struct Collision<'a> {
    pub point: Point,
    pub normal: Point,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub facing: Facing,
    pub material: &'a Material,
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
    Quad,
    Collection,
}

#[enum_dispatch(Object)]
pub trait Hit {
    fn hit(&self, ray: &Ray, t: Range<f64>) -> Option<Collision>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    fn uv(&self, point: &Point) -> (f64, f64) {
        let p = (*point - self.center) / self.radius;
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::PI / 2.0) / std::f64::consts::PI;
        (u, v)
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t: Range<f64>) -> Option<Collision> {
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
        let (u, v) = self.uv(&point);

        Some(Collision {
            point,
            normal,
            t,
            u,
            v,
            facing,
            material: &self.material,
        })
    }
}

pub struct Quad {
    q: Point,
    u: Vector,
    v: Vector,
    normal: Vector,
    d: f64,
    w: Vector,
    material: Material,
}

impl Quad {
    pub fn new(q: Point, u: Vector, v: Vector, material: Material) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.length_squared();
        Quad {
            q,
            u,
            v,
            normal,
            d,
            w,
            material,
        }
    }

    fn uv(alpha: f64, beta: f64) -> Option<(f64, f64)> {
        if alpha < 0.0 || 1.0 < alpha || beta < 0.0 || 1.0 < beta {
            None
        } else {
            Some((alpha, beta))
        }
    }
}

impl Hit for Quad {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<Collision> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / denominator;
        if !ray_t.contains(&t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_vector));

        let Some((u, v)) = Quad::uv(alpha, beta) else {
            return None;
        };

        let (normal, facing) = set_facing(ray, self.normal);

        Some(Collision {
            point: intersection,
            normal,
            t,
            u,
            v,
            facing,
            material: &self.material,
        })
    }
}

pub struct Collection {
    pub objects: Vec<Object>,
}

impl Hit for Collection {
    fn hit(&self, ray: &Ray, t: Range<f64>) -> Option<Collision> {
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
