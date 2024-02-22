// Test that we can paint to the screen using glow directly.

use std::error::Error;
use std::io::{Cursor, ErrorKind, Read, Write};
use std::thread::JoinHandle;
use eframe::egui;
use eframe::egui::{Frame, Margin};
use image::{ImageOutputFormat, RgbImage};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use single_value_channel::{Receiver, Updater};
use uuid::Uuid;

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

            Box::new(RaytracerApp::with_settings(load_settings().unwrap_or_default()))
        }),
    )?;
    Ok(())
}

struct RaytracerApp {
    image: Vec<u8>,
    image_id: Uuid,
    render_settings: RenderSettings,
    render_handle: Option<JoinHandle<Vec<u8>>>,
    progress_updater: Updater<f32>,
    progress: Receiver<f32>,
}

impl RaytracerApp {
    fn with_settings(settings: RenderSettings) -> Self {
        let (receiver, updater) = single_value_channel::channel_starting_with(0.0_f32);
        Self {
            image: vec![],
            image_id: Uuid::new_v4(),
            render_settings: settings,
            render_handle: None,
            progress_updater: updater,
            progress: receiver,
        }
    }
}

impl Default for RaytracerApp {
    fn default() -> Self {
        let image = vec![];
        let (receiver, updater) = single_value_channel::channel_starting_with(0.0_f32);
        Self {
            image,
            image_id: Uuid::new_v4(),
            render_settings: RenderSettings::default(),
            render_handle: None,
            progress_updater: updater,
            progress: receiver,
        }
    }
}

impl eframe::App for RaytracerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.render_handle.is_some() && self.render_handle.as_ref().unwrap().is_finished() {
            self.image = self.render_handle.take().unwrap().join().unwrap();
            self.image_id = Uuid::new_v4();
            self.render_handle = None;
            info!("Render complete");
            ctx.request_repaint();
        }

        let image_source = egui::ImageSource::Bytes {
            uri: format!("bytes://render-{}.png", self.image_id).into(),
            bytes: self.image.clone().into(),
        };


        egui::CentralPanel::default().frame(Frame { inner_margin: Margin::same(0.), ..Default::default()}).show(ctx, |ui| {
            if !self.image.is_empty() {
                ui.add(egui::Image::new(image_source));
            }
        });

        egui::Window::new("Render settings").show(ctx, |ui| {
            egui::Grid::new("render_settings").num_columns(2).show(ui, |ui| {
                ui.label("Height");
                ui.add(egui::DragValue::new(&mut self.render_settings.height).speed(1.0));
                ui.end_row();

                ui.label("Width");
                ui.add(egui::DragValue::new(&mut self.render_settings.width).speed(1.0));
                ui.end_row();

                ui.label("Samples");
                ui.add(egui::DragValue::new(&mut self.render_settings.samples).speed(1.0));
                ui.end_row();
            });


            if self.render_handle.is_none() {
                if ui.button("Render").clicked() {
                    let render_settings = self.render_settings.clone();
                    let sender = self.progress_updater.clone();
                    let mut context = ctx.clone();
                    self.render_handle = Some(std::thread::spawn(move || {
                        let ret = render(render_settings, sender, &mut context);
                        context.request_repaint();
                        ret
                    }));
                }
            } else {
                ui.add_enabled(false, egui::Button::new("Render"));
                ui.end_row();
                ui.add(egui::ProgressBar::new(*self.progress.latest()).show_percentage());
                ui.end_row();
            }
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            let _ = save_settings(&self.render_settings).inspect_err(|e| warn!("Error saving settings: {}", e));
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct RenderSettings {
    width: u32,
    height: u32,
    samples: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            samples: 100,
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

fn render(settings: RenderSettings, sender: Updater<f32>, context: &mut egui::Context) -> Vec<u8> {
    let mut image = RgbImage::new(settings.width, settings.height);

    for x in 0..settings.width {
        for y in 0..settings.height {
            let r = (x as f32 / settings.width as f32) * 255.0;
            let g = (y as f32 / settings.height as f32) * 255.0;
            let b = 0.0;
            image.put_pixel(x, y, image::Rgb([r as u8, g as u8, b as u8]));
            let _ = sender.update((x * settings.height + y) as f32 / (settings.width * settings.height) as f32);
            context.request_repaint();
        }
    }

    let mut buffer = Cursor::new(vec![]);
    let result = image.write_to(&mut buffer, ImageOutputFormat::Png);

    match result {
        Ok(_) => buffer.into_inner(),
        Err(e) => {
            info!("Error writing image: {}", e);
            vec![]
        }
    }
}

