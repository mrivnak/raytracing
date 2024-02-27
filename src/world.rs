use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Light, Material, Metal, Simple};
use crate::object::{Collection, Object, Quad, Sphere};
use crate::settings::CameraSettings;
use crate::texture::{Image, Noise, Texture};
use crate::vector::{Point, Vector};
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
    #[strum(to_string = "Earth")]
    Scene8,
    #[strum(to_string = "Two Perlin Spheres")]
    Scene9,
    #[strum(to_string = "Quads")]
    Scene10,
    #[strum(to_string = "Simple Light")]
    Scene11,
    #[strum(to_string = "Cornell Box")]
    Scene12,
}

pub struct World {
    pub object: Object,
    pub background: Color,
}

pub fn create_world(scene: &Scene) -> World {
    match scene {
        Scene::Scene1 => create_scene1(),
        Scene::Scene2 => create_scene2(),
        Scene::Scene3 => create_scene3(),
        Scene::Scene4 => create_scene4(),
        Scene::Scene5 => create_scene5(),
        Scene::Scene6 => create_scene6(),
        Scene::Scene7 => create_scene7(),
        Scene::Scene8 => create_scene8(),
        Scene::Scene9 => create_scene9(),
        Scene::Scene10 => create_scene10(),
        Scene::Scene11 => create_scene11(),
        Scene::Scene12 => create_scene12(),
    }
}

pub fn get_scene_camera(scene: &Scene) -> CameraSettings {
    match scene {
        Scene::Scene1 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 0.0),
            focus_point: Point::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
        },
        Scene::Scene2 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 0.0),
            focus_point: Point::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
        },
        Scene::Scene3 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 0.0),
            focus_point: Point::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
        },
        Scene::Scene4 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 0.0),
            focus_point: Point::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
        },
        Scene::Scene5 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 0.0),
            focus_point: Point::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
        },
        Scene::Scene6 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 0.0),
            focus_point: Point::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
        },
        Scene::Scene7 => CameraSettings {
            camera_position: Point::new(13.0, 2.0, 3.0),
            focus_point: Point::new(0.0, 0.0, 0.0),
            field_of_view: 20.0,
        },
        Scene::Scene8 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 12.0),
            focus_point: Point::new(0.0, 0.0, 0.0),
            field_of_view: 20.0,
        },
        Scene::Scene9 => CameraSettings {
            camera_position: Point::new(13.0, 2.0, 3.0),
            focus_point: Point::new(0.0, 0.0, 0.0),
            field_of_view: 20.0,
        },
        Scene::Scene10 => CameraSettings {
            camera_position: Point::new(0.0, 0.0, 9.0),
            focus_point: Point::new(0.0, 0.0, 0.0),
            field_of_view: 80.0,
        },
        Scene::Scene11 => CameraSettings {
            camera_position: Point::new(26.0, 3.0, 6.0),
            focus_point: Point::new(0.0, 2.0, 0.0),
            field_of_view: 20.0,
        },
        Scene::Scene12 => CameraSettings {
            camera_position: Point::new(278.0, 278.0, -800.0),
            focus_point: Point::new(278.0, 278.0, 0.0),
            field_of_view: 40.0,
        },
    }
}

fn create_scene1() -> World {
    let object = Object::Collection(Collection {
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
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene2() -> World {
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

    let object = Object::Collection(Collection {
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
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene3() -> World {
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

    let object = Object::Collection(Collection {
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
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene4() -> World {
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

    let object = Object::Collection(Collection {
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
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene5() -> World {
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

    let object = Object::Collection(Collection {
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
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene6() -> World {
    let material_left = Material::Lambertian(Lambertian {
        albedo: Color::new(0.0, 0.0, 1.0),
    });
    let material_right = Material::Lambertian(Lambertian {
        albedo: Color::new(1.0, 0.0, 0.0),
    });

    let r = (std::f64::consts::PI / 4.0).cos();

    let object = Object::Collection(Collection {
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
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene7() -> World {
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
                if choose_mat < 0.65 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian(Lambertian { albedo });
                    objects.push(Object::Sphere(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else if choose_mat < 0.80 {
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

    let object = Object::Collection(Collection { objects });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene8() -> World {
    let earth_texture = Texture::Image(Image::load("res/earth.jpg".into()).unwrap_or_default());
    let earth_material = Material::Simple(Simple {
        texture: earth_texture,
    });

    let object = Object::Sphere(Sphere {
        center: Point::new(0.0, 0.0, -12.0),
        radius: 2.0,
        material: earth_material,
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene9() -> World {
    let perlin_texture = Texture::Noise(Noise::new(4.0));
    let perlin_material = Material::Simple(Simple {
        texture: perlin_texture,
    });

    let object = Object::Collection(Collection {
        objects: vec![
            Object::Sphere(Sphere {
                center: Point::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: perlin_material.clone(),
            }),
            Object::Sphere(Sphere {
                center: Point::new(0.0, 2.0, 0.0),
                radius: 2.0,
                material: perlin_material,
            }),
        ],
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene10() -> World {
    let left_red = Material::Lambertian(Lambertian {
        albedo: Color::new(1.0, 0.2, 0.2),
    });
    let back_green = Material::Lambertian(Lambertian {
        albedo: Color::new(0.2, 1.0, 0.2),
    });
    let right_blue = Material::Lambertian(Lambertian {
        albedo: Color::new(0.2, 0.2, 1.0),
    });
    let upper_orange = Material::Lambertian(Lambertian {
        albedo: Color::new(1.0, 0.5, 0.0),
    });
    let lower_teal = Material::Lambertian(Lambertian {
        albedo: Color::new(0.2, 0.8, 0.8),
    });

    let object = Object::Collection(Collection {
        objects: vec![
            Object::Quad(Quad::new(
                Point {
                    x: -3.0,
                    y: -2.0,
                    z: 5.0,
                },
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                Vector {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                left_red,
            )),
            Object::Quad(Quad::new(
                Point {
                    x: -2.0,
                    y: -2.0,
                    z: 0.0,
                },
                Vector {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                back_green,
            )),
            Object::Quad(Quad::new(
                Point {
                    x: 3.0,
                    y: -2.0,
                    z: 1.0,
                },
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                Vector {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                right_blue,
            )),
            Object::Quad(Quad::new(
                Point {
                    x: -2.0,
                    y: 3.0,
                    z: 1.0,
                },
                Vector {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                upper_orange,
            )),
            Object::Quad(Quad::new(
                Point {
                    x: -2.0,
                    y: -3.0,
                    z: 5.0,
                },
                Vector {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                lower_teal,
            )),
        ],
    });
    let background = Color::new(0.7, 0.8, 1.0);
    World { object, background }
}

fn create_scene11() -> World {
    let mut objects = Vec::new();
    let perlin_texture = Texture::Noise(Noise::new(4.0));
    objects.push(Object::Sphere(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Simple(Simple {
            texture: perlin_texture.clone(),
        }),
    }));
    objects.push(Object::Sphere(Sphere {
        center: Point::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Material::Simple(Simple {
            texture: perlin_texture,
        }),
    }));

    let light = Material::Light(Light {
        color: Color::new(4.0, 4.0, 4.0),
    });
    objects.push(Object::Quad(Quad::new(
        Point {
            x: 3.0,
            y: 1.0,
            z: -2.0,
        },
        Vector {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        light,
    )));

    let object = Object::Collection(Collection { objects });
    let background = Color::new(0.0, 0.0, 0.0);

    World { object, background }
}

fn create_scene12() -> World {
    let mut objects = Vec::new();

    let red = Material::Lambertian(Lambertian {
        albedo: Color::new(0.65, 0.05, 0.05),
    });
    let white = Material::Lambertian(Lambertian {
        albedo: Color::new(0.73, 0.73, 0.73),
    });
    let green = Material::Lambertian(Lambertian {
        albedo: Color::new(0.12, 0.45, 0.15),
    });
    let light = Material::Light(Light {
        color: Color::new(15.0, 15.0, 15.0),
    });

    objects.push(Object::Quad(Quad::new(
        Point {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    )));
    objects.push(Object::Quad(Quad::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    )));
    objects.push(Object::Quad(Quad::new(
        Point {
            x: 343.0,
            y: 554.0,
            z: 332.0,
        },
        Vector {
            x: -130.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: -105.0,
        },
        light,
    )));
    objects.push(Object::Quad(Quad::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        white.clone(),
    )));
    objects.push(Object::Quad(Quad::new(
        Point {
            x: 555.0,
            y: 555.0,
            z: 555.0,
        },
        Vector {
            x: -555.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: -555.0,
        },
        white.clone(),
    )));
    objects.push(Object::Quad(Quad::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        Vector {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        white.clone(),
    )));

    let object = Object::Collection(Collection { objects });
    let background = Color::new(0.0, 0.0, 0.0);

    World { object, background }
}
