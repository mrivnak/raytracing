use serde::{Deserialize, Serialize};
use crate::color::Color;
use crate::material::{Lambertian, Material, Metal};
use crate::object::{Collection, Object, Sphere};
use crate::vector::Point;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, strum_macros::Display)]
pub enum Scene {
    #[strum(to_string = "Two Spheres")]
    Scene1,
    #[strum(to_string = "Metal Spheres")]
    Scene2,
}

pub fn create_world(scene: &Scene) -> Object {
    match scene {
        Scene::Scene1 => create_scene1(),
        Scene::Scene2 => create_scene2(),
    }
}

fn create_scene1() -> Object {
    Object::Collection(Collection {
        objects: vec![
            Object::Sphere(Sphere {
                center: Point::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Lambertian(Lambertian {
                    albedo: Color::new(0.1, 0.2, 0.5),
                }),
            }),
            Object::Sphere(Sphere {
                center: Point::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian(Lambertian {
                    albedo: Color::new(0.1, 0.2, 0.5),
                }),
            }),
        ],
    })
}

fn create_scene2() -> Object {
    let material_ground = Material::Lambertian(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Material::Lambertian(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Material::Metal(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Material::Metal(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    Object::Collection(Collection {
        objects: vec![
            Object::Sphere(Sphere {
                center: Point::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: material_ground,
            }),
            Object::Sphere(Sphere {
                center: Point::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: material_center,
            }),
            Object::Sphere(Sphere {
                center: Point::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: material_left,
            }),
            Object::Sphere(Sphere {
                center: Point::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: material_right,
            }),
        ],
    })
}
