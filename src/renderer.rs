use crate::color::Color;
use crate::data::Size;
use crate::object::{Hit, Object, Sphere};
use crate::ray::Ray;
use crate::vector::{Point, Vector};
use crate::world::create_world;
use crate::RenderSettings;
use eframe::egui;
use image::{ImageOutputFormat, RgbImage};
use log::info;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use single_value_channel::Updater;
use std::io::Cursor;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};

pub fn render(
    settings: RenderSettings,
    sender: Updater<f32>,
    context: &mut egui::Context,
) -> Vec<u8> {
    let mut image = Arc::new(Mutex::new(RgbImage::new(
        settings.size.width,
        settings.size.height,
    )));
    const Y_UP: Vector = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let viewport_size = Size {
        height: 2.0,
        width: 2.0 * settings.size.width as f64 / settings.size.height as f64,
    };

    let viewport_u = Vector::new(viewport_size.width, 0.0, 0.0);
    let viewport_v = Vector::new(0.0, -viewport_size.height, 0.0);

    let pixel_delta_u = viewport_u / settings.size.width as f64;
    let pixel_delta_v = viewport_v / settings.size.height as f64;

    let viewport_origin = settings.camera_position
        - Vector::new(0.0, 0.0, settings.focal_length as f64)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let origin_pixel = viewport_origin + (pixel_delta_u + pixel_delta_v) / 2.0;

    let world = create_world();

    let mut completed_pixels = AtomicU32::new(0);
    (0..settings.size.width).into_par_iter().for_each(|x| {
        for y in 0..settings.size.height {
            let pixel_center =
                origin_pixel + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);

            let samples = (0..settings.samples)
                .into_iter()
                .map(|_| {
                    let ray = get_ray(
                        pixel_center,
                        settings.camera_position,
                        pixel_delta_u,
                        pixel_delta_v,
                    );
                    ray_color(&ray, &world, settings.max_depth)
                })
                .collect::<Vec<_>>();
            let color: Color = samples.into();

            image
                .lock()
                .unwrap()
                .put_pixel(x, y, image::Rgb(color.into()));
            let pixels = completed_pixels.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let _ = sender
                .update((pixels + 1) as f32 / (settings.size.width * settings.size.height) as f32);
            context.request_repaint();
        }
    });

    let mut buffer = Cursor::new(vec![]);
    let result = image
        .lock()
        .unwrap()
        .write_to(&mut buffer, ImageOutputFormat::Png);

    match result {
        Ok(_) => buffer.into_inner(),
        Err(e) => {
            info!("Error writing image: {}", e);
            vec![]
        }
    }
}

fn get_ray(pixel_center: Point, camera_position: Point, pixel_du: Vector, pixel_dv: Vector) -> Ray {
    let pixel_sample = pixel_center + pixel_sample_square(pixel_du, pixel_dv);
    let ray_direction = pixel_sample - camera_position;
    Ray::new(camera_position, ray_direction)
}

fn pixel_sample_square(du: Vector, dv: Vector) -> Vector {
    let px = -0.5 + rand::random::<f64>();
    let py = -0.5 + rand::random::<f64>();
    return px * du + py * dv;
}

fn ray_color(ray: &Ray, obj: &Object, depth: u32) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    if let Some(rec) = obj.hit(ray, 0.001..f64::INFINITY) {
        let direction = Vector::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.point, direction), obj, depth - 1);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
