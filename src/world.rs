use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::object::{Collection, Object, Sphere};
use crate::vector::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, strum_macros::Display)]
pub enum Scene {
    #[strum(to_string = "One Sphere")]
    Scene1,
    #[strum(to_string = "Metal Spheres")]
    Scene2,
    #[strum(to_string = "Glass Spheres")]
    Scene3,
    #[strum(to_string = "Three Spheres")]
    Scene4,
    #[strum(to_string = "Hollow Glass Sphere")]
    Scene5,
    #[strum(to_string = "Red and Blue")]
    Scene6,
    #[strum(to_string = "Many Spheres")]
    Scene7,
}

pub fn create_world(scene: &Scene) -> Object {
    match scene {
        Scene::Scene1 => create_scene1(),
        Scene::Scene2 => create_scene2(),
        Scene::Scene3 => create_scene3(),
        Scene::Scene4 => create_scene4(),
        Scene::Scene5 => create_scene5(),
        Scene::Scene6 => create_scene6(),
        Scene::Scene7 => create_scene7(),
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

fn create_scene3() -> Object {
    let material_ground = Material::Lambertian(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Material::Dielectric(Dielectric {
        refraction_index: 1.5,
    });
    let material_left = Material::Dielectric(Dielectric {
        refraction_index: 1.5,
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

fn create_scene4() -> Object {
    let material_ground = Material::Lambertian(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Material::Lambertian(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Material::Dielectric(Dielectric {
        refraction_index: 1.5,
    });
    let material_right = Material::Metal(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
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

fn create_scene5() -> Object {
    let material_ground = Material::Lambertian(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Material::Lambertian(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Material::Dielectric(Dielectric {
        refraction_index: 1.5,
    });
    let material_right = Material::Metal(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
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
                material: material_left.clone(),
            }),
            Object::Sphere(Sphere {
                center: Point::new(-1.0, 0.0, -1.0),
                radius: -0.4,
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

fn create_scene6() -> Object {
    let material_left = Material::Lambertian(Lambertian {
        albedo: Color::new(0.0, 0.0, 1.0),
    });
    let material_right = Material::Lambertian(Lambertian {
        albedo: Color::new(1.0, 0.0, 0.0),
    });

    let r = (std::f64::consts::PI / 4.0).cos();

    Object::Collection(Collection {
        objects: vec![
            Object::Sphere(Sphere {
                center: Point::new(-r, 0.0, -1.0),
                radius: r,
                material: material_left,
            }),
            Object::Sphere(Sphere {
                center: Point::new(r, 0.0, -1.0),
                radius: r,
                material: material_right,
            }),
        ],
    })
}

fn create_scene7() -> Object {
    let mut objects = vec![];

    let ground_material = Material::Lambertian(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    objects.push(Object::Sphere(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian(Lambertian { albedo });
                    objects.push(Object::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_range(0.5..1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    let sphere_material = Material::Metal(Metal { albedo, fuzz });
                    objects.push(Object::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else {
                    // glass
                    let sphere_material = Material::Dielectric(Dielectric {
                        refraction_index: 1.5,
                    });
                    objects.push(Object::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                }
            }
        }
    }

    let material_1 = Material::Dielectric(Dielectric {
        refraction_index: 1.5,
    });
    objects.push(Object::Sphere(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material_1,
    }));

    let material_2 = Material::Lambertian(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    objects.push(Object::Sphere(Sphere {
        center: Point::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material_2,
    }));

    let material_3 = Material::Metal(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    objects.push(Object::Sphere(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material_3,
    }));

    Object::Collection(Collection {
        objects
    })
}
