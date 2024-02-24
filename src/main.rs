mod color;
mod data;
mod material;
mod object;
mod ray;
mod renderer;
mod vector;
mod world;

use crate::data::Size;
use eframe::egui;
use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;
use image::{ImageOutputFormat, RgbImage};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use single_value_channel::{Receiver, Updater};
use std::error::Error;
use std::io::{Cursor, ErrorKind, Read, Write};
use std::thread::JoinHandle;
use std::time::Duration;
use uuid::Uuid;

use crate::renderer::render;
use crate::vector::Point;
use crate::world::Scene;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Raytracing",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(RaytracerApp::with_settings(
                load_settings().unwrap_or_default(),
            ))
        }),
    )?;
    Ok(())
}

struct RaytracerApp {
    image: Vec<u8>,
    image_id: Uuid,
    render_settings: RenderSettings,
    render_handle: Option<JoinHandle<(Vec<u8>, Duration)>>,
    duration: Option<Duration>,
    progress_updater: Updater<f32>,
    progress: Receiver<f32>,
}

impl RaytracerApp {
    fn with_settings(settings: RenderSettings) -> Self {
        let (receiver, updater) = single_value_channel::channel_starting_with(0.0);
        Self {
            image: vec![],
            image_id: Uuid::new_v4(),
            render_settings: settings,
            render_handle: None,
            duration: None,
            progress_updater: updater,
            progress: receiver,
        }
    }
}

impl Default for RaytracerApp {
    fn default() -> Self {
        let image = vec![];
        let (receiver, updater) = single_value_channel::channel_starting_with(0.0);
        Self {
            image,
            image_id: Uuid::new_v4(),
            render_settings: RenderSettings::default(),
            render_handle: None,
            duration: None,
            progress_updater: updater,
            progress: receiver,
        }
    }
}

impl eframe::App for RaytracerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.render_handle.is_some() && self.render_handle.as_ref().unwrap().is_finished() {
            let render_result = self.render_handle.take().unwrap().join().unwrap();
            self.image = render_result.0;
            self.duration = Some(render_result.1);
            self.image_id = Uuid::new_v4();
            self.render_handle = None;
            info!("Render complete");
            ctx.request_repaint();
        }

        let image_source = egui::ImageSource::Bytes {
            uri: format!("bytes://render-{}.png", self.image_id).into(),
            bytes: self.image.clone().into(),
        };

        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::same(0.),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if !self.image.is_empty() {
                    ui.add(egui::Image::new(image_source));
                }
            });

        egui::Window::new("Render settings").show(ctx, |ui| {
            if ui.button("Reset").clicked() {
                let scene = self.render_settings.scene.clone();
                self.render_settings = RenderSettings {
                    scene,
                    ..Default::default()
                }
            }

            egui::Grid::new("render_settings")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Scene");
                    egui::ComboBox::from_label("")
                        .selected_text(self.render_settings.scene.to_string())
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            ui.selectable_value(&mut self.render_settings.scene, Scene::Scene1, Scene::Scene1.to_string());
                            ui.selectable_value(&mut self.render_settings.scene, Scene::Scene2, Scene::Scene2.to_string());
                        });
                    ui.end_row();

                    ui.label("Height");
                    ui.add(egui::DragValue::new(&mut self.render_settings.size.height).speed(1.0));
                    ui.end_row();

                    ui.label("Width");
                    ui.add(egui::DragValue::new(&mut self.render_settings.size.width).speed(1.0));
                    ui.end_row();

                    ui.label("Samples");
                    ui.add(egui::DragValue::new(&mut self.render_settings.samples).speed(1.0));
                    ui.end_row();

                    ui.label("Max Depth");
                    ui.add(
                        egui::DragValue::new(&mut self.render_settings.max_depth)
                            .clamp_range(0..=2048)
                            .speed(1.0),
                    );
                    ui.end_row();

                    ui.label("Camera Position");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(
                            egui::DragValue::new(&mut self.render_settings.camera_position.x)
                                .speed(0.1),
                        );
                        ui.label("Y:");
                        ui.add(
                            egui::DragValue::new(&mut self.render_settings.camera_position.y)
                                .speed(0.1),
                        );
                        ui.label("Z:");
                        ui.add(
                            egui::DragValue::new(&mut self.render_settings.camera_position.z)
                                .speed(0.1),
                        );
                    });
                    ui.end_row();

                    ui.label("Focal Length");
                    ui.add(egui::DragValue::new(&mut self.render_settings.focal_length).speed(0.1));
                    ui.end_row();
                });

            if self.render_handle.is_none() {
                if ui.button("Render").clicked() {
                    self.image = vec![];
                    let render_settings = self.render_settings.clone();
                    let sender = self.progress_updater.clone();
                    let mut context = ctx.clone();
                    self.render_handle = Some(std::thread::spawn(move || {
                        let start = std::time::Instant::now();
                        let ret = render(render_settings, sender, &mut context);
                        let duration = start.elapsed();
                        context.request_repaint();
                        (ret, duration)
                    }));
                }
                if let Some(duration) = self.duration {
                    ui.label(format!("Render time: {}", duration.human(Truncate::Millis)));
                }
            } else {
                ui.add_enabled(false, egui::Button::new("Render"));
                ui.end_row();
                ui.add(egui::ProgressBar::new(*self.progress.latest()).show_percentage());
                ui.end_row();
            }
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            let _ = save_settings(&self.render_settings)
                .inspect_err(|e| warn!("Error saving settings: {}", e));
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct RenderSettings {
    size: Size<u32>,
    samples: u32,
    max_depth: u32,
    camera_position: Point,
    focal_length: f32,
    scene: Scene,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            size: Size {
                width: 1920,
                height: 1080,
            },
            samples: 100,
            max_depth: 50,
            camera_position: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            focal_length: 1.0,
            scene: Scene::Scene1,
        }
    }
}

fn save_settings(settings: &RenderSettings) -> Result<(), std::io::Error> {
    let path = get_settings_path();
    let parent_dir = path.parent().unwrap();
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }
    let mut file = std::fs::File::create(path)?;
    let toml = toml::to_string(settings).unwrap();
    file.write_all(toml.as_bytes())
}

fn load_settings() -> Result<RenderSettings, Box<dyn Error>> {
    let path = get_settings_path();
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            info!("No settings file found, using defaults");
            return Ok(RenderSettings::default());
        }
        Err(err) => return Err(Box::new(err)),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let settings: RenderSettings = toml::from_str(contents.as_str())?;
    Ok(settings)
}

fn get_settings_path() -> std::path::PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("raytracer");
    path.push("settings.toml");
    path
}
