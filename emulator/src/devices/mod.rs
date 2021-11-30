use ndarray::Array1;

pub mod brown;
pub mod interpolated;
pub mod stationary;

pub use brown::Brown;
pub use interpolated::Interpolated;
pub use stationary::Stationary;

pub trait Device {
    fn init(&mut self);
    fn did(&self) -> u32;
    fn position(&self) -> Array1<f32>;
    fn update(&mut self, dt: f32) -> Array1<f32>;
}
