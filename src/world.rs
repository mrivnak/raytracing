use crate::color::Color;
use crate::material::{Lambertian, Material, Metal};
use crate::object::{Collection, Object, Sphere};
use crate::vector::Point;

pub fn create_world() -> Object {
    // TODO: Specify in a config file or something
    // Scene 1
    // Object::Collection(Collection {
    //     objects: vec![
    //         Object::Sphere(Sphere {
    //             center: Point::new(0.0, 0.0, -1.0),
    //             radius: 0.5,
    //             material: Material::Lambertian(Lambertian {
    //                 albedo: Color::new(0.1, 0.2, 0.5),
    //             })
    //         }),
    //         Object::Sphere(Sphere {
    //             center: Point::new(0.0, -100.5, -1.0),
    //             radius: 100.0,
    //             material: Material::Lambertian(Lambertian {
    //                 albedo: Color::new(0.1, 0.2, 0.5),
    //             })
    //         }),
    //     ],
    // })

    // Scene 2
    let material_ground = Material::Lambertian(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Material::Lambertian(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Material::Metal(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    });
    let material_right = Material::Metal(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
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
