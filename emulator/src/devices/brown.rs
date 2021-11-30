use super::Device;
use ndarray::*;
use ndarray_linalg::Norm;
use rand::{rngs::StdRng, RngCore, SeedableRng};

static mut RNG: Option<StdRng> = None;

unsafe fn rng() -> &'static mut StdRng {
    if RNG.is_none() {
        RNG = Some(SeedableRng::seed_from_u64(123456));
    }

    RNG.as_mut().unwrap()
}

pub struct Brown {
    /// device identification number
    pub did: u32,
    /// device position: {x, y, z}
    pub position: Array1<f32>,
    /// position variability
    pub radius: f32,
}

impl Device for Brown {
    fn init(&mut self) {}
    fn did(&self) -> u32 {
        self.did
    }
    fn position(&self) -> Array1<f32> {
        self.position.to_owned()
    }

    fn update(&mut self, _dt: f32) -> Array1<f32> {
        let mut rand_pos = array![0.0, 0.0, 0.0];
        unsafe {
            let gen = rng();
            rand_pos[0] = gen.next_u32() as f32;
            rand_pos[1] = gen.next_u32() as f32;
            rand_pos[2] = gen.next_u32() as f32;
        }
        let movement = self.radius / rand_pos.norm().max(0.001) * rand_pos;
        let result = self.position.to_owned() + movement;
        self.position.assign(&result);
        result
    }
}
