pub use ndarray::*;
use ndarray_linalg::Norm;
use serde::Deserialize;
// use toml::{Value, de::Error};
use ndarray::Array1;
use std::rc::Rc;

use super::Device;

#[derive(Deserialize)]
pub struct ConfigPoint {
    pos: [f32; 3],
    velocity: Option<f32>,
}

#[derive(Deserialize)]
pub struct ConfigPathFile {
    point: Vec<ConfigPoint>,
}

pub struct Interpolated {
    /// device identification number
    pub did: u32,
    /// device position: {x, y, z}
    pub position: Array1<f32>,
    config: Rc<ConfigPathFile>,
    point_idx: usize,
}

impl Device for Interpolated {
    fn init(&mut self) {}
    fn did(&self) -> u32 {
        self.did
    }
    fn position(&self) -> Array1<f32> {
        self.position.to_owned()
    }

    fn update(&mut self, dt: f32) -> Array1<f32> {
        let point_next_idx = (self.point_idx + 1) % self.config.point.len();
        let velocity = self.config.point[point_next_idx].velocity.unwrap_or(3.0);
        let distance = velocity * dt;
        let moved = self.move_to_next_point(distance);
        if moved < distance {
            self.move_to_next_point(distance - moved);
        }
        self.position.to_owned()
    }
}

impl Interpolated {
    pub fn create(id: u32, config: &Rc<ConfigPathFile>) -> Interpolated {
        Interpolated {
            did: id,
            point_idx: 0,
            config: config.clone(),
            position: arr1(&[0.0, 0.0, 0.0]),
        }
    }
    fn move_to_next_point(&mut self, max_distance: f32) -> f32 {
        let point_next_idx = (self.point_idx + 1) % self.config.point.len();
        let dst_point: Array1<f32> = arr1(&self.config.point[point_next_idx].pos);
        let movement = dst_point - self.position.to_owned();
        let distance = Norm::norm(&movement.to_owned());
        let versor = movement.to_owned() / distance;
        if distance > max_distance {
            self.position = self.position.to_owned() + max_distance * versor;
        } else {
            self.point_idx = point_next_idx;
            self.position = self.position.to_owned() + distance * versor;
        }
        return f32::min(Norm::norm(&movement.to_owned()), max_distance);
    }

    pub fn parse_config(toml_conf: &String) -> Result<ConfigPathFile, toml::de::Error> {
        let config_info: ConfigPathFile = match toml::from_str(toml_conf) {
            Ok(conf) => conf,
            Err(e) => return Result::Err(e),
        };
        Result::Ok(config_info)
    }
}
