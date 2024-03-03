use crate::data::Size;
use crate::vector::Point;
use crate::world::Scene;
#[cfg(feature = "gui")]
use log::info;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gui")]
use std::error::Error;
#[cfg(feature = "gui")]
use std::io::{ErrorKind, Read, Write};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenderSettings {
    pub size: Size<u32>,
    pub samples: u32,
    pub max_depth: u32,
    pub camera_position: Point,
    pub focus_point: Point,
    pub field_of_view: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub scene: Scene,
}

pub struct CameraSettings {
    pub camera_position: Point,
    pub focus_point: Point,
    pub field_of_view: f32,
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
            focus_point: Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            field_of_view: 90.0,
            defocus_angle: 0.0,
            focus_distance: 10.0,
            scene: Scene::OneSphere,
        }
    }
}

#[cfg(feature = "gui")]
pub fn save_settings(settings: &RenderSettings) -> Result<(), std::io::Error> {
    let path = get_settings_path();
    let parent_dir = path.parent().unwrap();
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }
    let mut file = std::fs::File::create(path)?;
    let toml = toml::to_string(settings).unwrap();
    file.write_all(toml.as_bytes())
}

#[cfg(feature = "gui")]
pub fn load_settings() -> Result<RenderSettings, Box<dyn Error>> {
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

#[cfg(feature = "gui")]
fn get_settings_path() -> std::path::PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("raytracer");
    path.push("settings.toml");
    path
}
