#![feature(more_float_constants)]

mod color;
mod data;
mod material;
mod object;
mod perlin;
mod ray;
mod renderer;
mod settings;
mod texture;
mod vector;
mod world;
mod quaternion;

use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;
use std::error::Error;

#[cfg(not(feature = "gui"))]
use clap::error::ErrorKind;
#[cfg(not(feature = "gui"))]
use clap::{CommandFactory, Parser};
#[cfg(not(feature = "gui"))]
use regex::Regex;

#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use log::{info, warn};
#[cfg(feature = "gui")]
use single_value_channel::{Receiver, Updater};
#[cfg(feature = "gui")]
use std::thread::JoinHandle;
#[cfg(feature = "gui")]
use std::time::Duration;
#[cfg(feature = "gui")]
use uuid::Uuid;

use crate::renderer::render;
use crate::settings::RenderSettings;
use crate::world::{get_scene_camera, Scene};

#[cfg(feature = "gui")]
use crate::settings::{load_settings, save_settings};

#[cfg(not(feature = "gui"))]
use crate::vector::Point;

/// Software raytracer
#[cfg(not(feature = "gui"))]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Scene to render
    #[arg(short, long, default_value = "cornell-box-empty")]
    scene: Scene,

    /// Camera position
    #[arg(short, long)]
    camera_position: Option<String>,

    /// Focus point
    #[arg(short, long)]
    focus_point: Option<String>,

    /// Render height
    #[arg(short, long)]
    height: Option<u32>,

    /// Render width
    #[arg(short, long)]
    width: Option<u32>,

    /// Samples per pixel
    #[arg(short, long)]
    samples: Option<u32>,

    /// Output file
    #[arg(short, long, default_value = "render.png")]
    output: String,

    /// Print settings
    /// Print the settings and exit
    #[arg(short, long)]
    print_settings: bool,
}

#[cfg(feature = "gui")]
#[cfg(not(tarpaulin_include))]
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

#[cfg(not(feature = "gui"))]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut settings = RenderSettings {
        scene: args.scene,
        ..Default::default()
    };

    let scene_camera = get_scene_camera(&settings.scene);
    settings.camera_position = scene_camera.camera_position;
    settings.focus_point = scene_camera.focus_point;
    settings.field_of_view = scene_camera.field_of_view;

    let point_re = Regex::new(r"\(?(?:\d+(?:\.\d+)?,\s?){2}(?:\d+(?:\.\d+)?)\)?")?;
    if let Some(camera_position) = args.camera_position.as_deref() {
        if point_re.is_match(camera_position) {
            let mut parts = camera_position.split(',');
            let x = parts.next().unwrap().parse::<f64>()?;
            let y = parts.next().unwrap().parse::<f64>()?;
            let z = parts.next().unwrap().parse::<f64>()?;
            settings.camera_position = Point::new(x, y, z);
        } else {
            let mut cmd = Args::command();
            cmd.error(ErrorKind::ValueValidation, "Invalid camera position")
                .exit()
        }
    }

    if let Some(focus_point) = args.camera_position.as_deref() {
        if point_re.is_match(focus_point) {
            let mut parts = focus_point.split(',');
            let x = parts.next().unwrap().parse::<f64>()?;
            let y = parts.next().unwrap().parse::<f64>()?;
            let z = parts.next().unwrap().parse::<f64>()?;
            settings.focus_point = Point::new(x, y, z);
        } else {
            let mut cmd = Args::command();
            cmd.error(ErrorKind::ValueValidation, "Invalid focus point")
                .exit()
        }
    }

    if let Some(height) = args.height {
        settings.size.height = height;
    }

    if let Some(width) = args.width {
        settings.size.width = width;
    }

    if let Some(samples) = args.samples {
        settings.samples = samples;
    }

    if args.print_settings {
        println!("{:#?}", settings);
        return Ok(());
    }

    let start = std::time::Instant::now();
    let image = render(settings);
    let duration = start.elapsed();
    std::fs::write(args.output, &image)?;
    println!("Render time: {}", duration.human(Truncate::Millis));
    Ok(())
}

#[cfg(feature = "gui")]
#[cfg(not(tarpaulin_include))]
struct RaytracerApp {
    image: Vec<u8>,
    image_id: Uuid,
    render_settings: RenderSettings,
    render_handle: Option<JoinHandle<(Vec<u8>, Duration)>>,
    duration: Option<Duration>,
    progress_updater: Updater<f32>,
    progress: Receiver<f32>,
}

#[cfg(feature = "gui")]
#[cfg(not(tarpaulin_include))]
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

#[cfg(feature = "gui")]
#[cfg(not(tarpaulin_include))]
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

#[cfg(feature = "gui")]
#[cfg(not(tarpaulin_include))]
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
                            // TODO: there's probably a way to use a macro for this
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::CornellBoxEmpty,
                                Scene::CornellBoxEmpty.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::CornellBoxTwoBoxes,
                                Scene::CornellBoxTwoBoxes.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::OneSphere,
                                Scene::OneSphere.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::MetalSpheres,
                                Scene::MetalSpheres.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::GlassSpheres,
                                Scene::GlassSpheres.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::ThreeSpheres,
                                Scene::ThreeSpheres.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::HollowGlassSphere,
                                Scene::HollowGlassSphere.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::RedAndBlue,
                                Scene::RedAndBlue.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::ManySpheres,
                                Scene::ManySpheres.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::Earth,
                                Scene::Earth.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::TwoPerlinSpheres,
                                Scene::TwoPerlinSpheres.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::Quads,
                                Scene::Quads.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.render_settings.scene,
                                Scene::SimpleLight,
                                Scene::SimpleLight.to_string(),
                            );
                        });
                    if ui.button("Reset camera").clicked() {
                        let cam_settings = get_scene_camera(&self.render_settings.scene);
                        self.render_settings.camera_position = cam_settings.camera_position;
                        self.render_settings.focus_point = cam_settings.focus_point;
                        self.render_settings.field_of_view = cam_settings.field_of_view;
                    }
                    ui.end_row();

                    ui.label("Width");
                    ui.add(egui::DragValue::new(&mut self.render_settings.size.width).speed(1.0));
                    ui.end_row();

                    ui.label("Height");
                    ui.add(egui::DragValue::new(&mut self.render_settings.size.height).speed(1.0));
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

                    ui.label("Focus Point");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(
                            egui::DragValue::new(&mut self.render_settings.focus_point.x)
                                .speed(0.1),
                        );
                        ui.label("Y:");
                        ui.add(
                            egui::DragValue::new(&mut self.render_settings.focus_point.y)
                                .speed(0.1),
                        );
                        ui.label("Z:");
                        ui.add(
                            egui::DragValue::new(&mut self.render_settings.focus_point.z)
                                .speed(0.1),
                        );
                    });
                    ui.end_row();

                    ui.label("Field of View");
                    ui.add(
                        egui::DragValue::new(&mut self.render_settings.field_of_view).speed(0.1),
                    );
                    ui.end_row();

                    ui.label("Defocus Angle");
                    ui.add(
                        egui::DragValue::new(&mut self.render_settings.defocus_angle).speed(0.1),
                    );
                    ui.end_row();

                    ui.label("Focus Distance");
                    ui.add(
                        egui::DragValue::new(&mut self.render_settings.focus_distance).speed(0.1),
                    );
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
