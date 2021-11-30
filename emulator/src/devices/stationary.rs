use super::Device;
use ndarray::Array1;

pub struct Stationary {
    /// device identification number
    pub did: u32,
    /// device position: {x, y, z}
    pub position: Array1<f32>,
}

impl Device for Stationary {
    fn init(&mut self) {}
    fn did(&self) -> u32 {
        self.did
    }
    fn position(&self) -> Array1<f32> {
        self.position.to_owned()
    }
    fn update(&mut self, _dt: f32) -> Array1<f32> {
        self.position.to_owned()
    }
}
