use serde_derive::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops;

pub type DevId = u32;

pub type Timestamp = u32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Coords(pub [f32; 3]);

impl ops::Index<usize> for Coords {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for Coords {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Trace {
    pub coords: Coords,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct Scent(VecDeque<Trace>);

impl Scent {
    pub fn new() -> Scent {
        Scent(VecDeque::with_capacity(10))
    }
    pub fn with_capacity(cap: usize) -> Scent {
        Scent(VecDeque::with_capacity(cap))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn add(&mut self, trace: Trace) {
        if self.0.len() > self.0.capacity() {
            self.0.pop_back();
        }
        self.0.push_front(trace);
    }
    pub fn get(&self, how_old: usize) -> Option<&Trace> {
        self.0.get(how_old)
    }
}
