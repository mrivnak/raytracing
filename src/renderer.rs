use crate::color::{Clamp, Color, GammaCorrect};
use crate::data::Size;
use crate::material::{Deflect, Emit};
use crate::object::{Hit, Object};
use crate::ray::Ray;
use crate::vector::{Point, Vector};
use crate::world::create_world;
use crate::RenderSettings;
#[cfg(feature = "gui")]
use eframe::egui;
use image::{ImageOutputFormat, RgbImage};
use log::info;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
#[cfg(feature = "gui")]
use single_value_channel::Updater;
use std::io::Cursor;
#[cfg(feature = "gui")]
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};

const V_UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub fn render(
    settings: RenderSettings,
    #[cfg(feature = "gui")] sender: Updater<f32>,
    #[cfg(feature = "gui")] context: &mut egui::Context,
) -> Vec<u8> {
    let image = Arc::new(Mutex::new(RgbImage::new(
        settings.size.width,
        settings.size.height,
    )));

    let focal_length = (settings.camera_position - settings.focus_point).length();
    let theta = settings.field_of_view.to_radians();
    let h = (theta / 2.0).tan();
    let viewport_size = {
        let height = 2.0 * h as f64 * focal_length;
        Size {
            height,
            width: height * settings.size.width as f64 / settings.size.height as f64,
        }
    };

    let w = (settings.camera_position - settings.focus_point).normalize();
    let u = V_UP.cross(&w).normalize();
    let v = w.cross(&u);

    let viewport_u = u * viewport_size.width;
    let viewport_v = -v * viewport_size.height;

    let pixel_delta_u = viewport_u / settings.size.width as f64;
    let pixel_delta_v = viewport_v / settings.size.height as f64;

    let viewport_origin =
        settings.camera_position - focal_length * w - viewport_u / 2.0 - viewport_v / 2.0;
    let origin_pixel = viewport_origin + (pixel_delta_u + pixel_delta_v) / 2.0;

    let defocus_radius =
        settings.focus_distance as f64 * (settings.defocus_angle as f64 / 2.0).to_radians().tan();
    let defocus_u = u * defocus_radius;
    let defocus_v = v * defocus_radius;

    let world = create_world(&settings.scene);

    #[cfg(feature = "gui")]
    let completed_pixels = AtomicU32::new(0);

    (0..settings.size.width).into_par_iter().for_each(|x| {
        for y in 0..settings.size.height {
            let pixel_center =
                origin_pixel + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);

            let samples = (0..settings.samples)
                .map(|_| {
                    let ray = get_ray(
                        pixel_center,
                        settings.camera_position,
                        pixel_delta_u,
                        pixel_delta_v,
                        settings.defocus_angle,
                        defocus_u,
                        defocus_v,
                    );
                    ray_color(&ray, &world.object, &world.background, settings.max_depth)
                })
                .collect::<Vec<_>>();
            let color: Color = Color::from(samples).gamma_correct().clamp(0.0, 1.0);

            image
                .lock()
                .unwrap()
                .put_pixel(x, y, image::Rgb(color.into()));

            #[cfg(feature = "gui")]
            {
                let pixels = completed_pixels.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let _ = sender.update(
                    (pixels + 1) as f32 / (settings.size.width * settings.size.height) as f32,
                );
                context.request_repaint();
            }
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

fn get_ray(
    pixel_center: Point,
    camera_position: Point,
    pixel_du: Vector,
    pixel_dv: Vector,
    defocus_angle: f32,
    defocus_u: Vector,
    defocus_v: Vector,
) -> Ray {
    let pixel_sample = pixel_center + pixel_sample_square(pixel_du, pixel_dv);

    let ray_origin = if defocus_angle > 0.0 {
        defocus_disk_sample(camera_position, defocus_u, defocus_v)
    } else {
        camera_position
    };
    let ray_direction = pixel_sample - ray_origin;
    Ray::new(ray_origin, ray_direction)
}

fn pixel_sample_square(du: Vector, dv: Vector) -> Vector {
    let px = -0.5 + rand::random::<f64>();
    let py = -0.5 + rand::random::<f64>();
    px * du + py * dv
}

fn defocus_disk_sample(camera_position: Point, defocus_u: Vector, defocus_v: Vector) -> Point {
    let p = Vector::random_in_unit_disk();
    return camera_position + (p.x * defocus_u) + (p.y * defocus_v);
}

fn ray_color(ray: &Ray, obj: &Object, background: &Color, depth: u32) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    let Some(hit) = obj.hit(ray, 0.001..f64::INFINITY) else {
        return *background;
    };

    let color_from_emission = hit.material.emit(hit.u, hit.v, &hit.point);
    let Some(deflection) = hit.material.deflect(ray, &hit) else {
        return color_from_emission;
    };

    let color_from_deflection =
        deflection.attenuation * ray_color(&deflection.ray, obj, background, depth - 1);
    color_from_emission + color_from_deflection
}
