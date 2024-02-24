use crate::object::{Collection, Object, Sphere};
use crate::vector::Point;

pub fn create_world() -> Object {
    // TODO: Specify in a config file or something
    Object::Collection(Collection {
        objects: vec![
            Object::Sphere(Sphere {
                center: Point::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Object::Sphere(Sphere {
                center: Point::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    })
}
