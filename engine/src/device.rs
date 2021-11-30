use serde_derive::{Serialize, Deserialize};
use std::collections::VecDeque;

use crate::measure;
use crate::utils::{DevId, Timestamp, Coords, Trace, Scent};
use ndarray::*;
use ndarray_linalg::Norm;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

const POSITION_TRACE_DEPTH : usize = 3;

#[derive(Serialize, Deserialize)]
pub struct Description {
	pub pos: Trace,
	pub id: DevId,
	pub timestamp: Timestamp, // last activity timestamp
}

pub struct Data {
	scent: Scent,
	id: DevId,
	timestamp: Timestamp, // last activity timestamp
}

impl Description {
	pub fn new(dev: &Data, timestamp: Timestamp) -> Description {
		Description {
			id: dev.id,
			pos: dev.estimate_position(timestamp),
			timestamp: timestamp,
		}
	}

	pub fn id(&self) -> DevId { self.id }
}

impl Data {
	pub fn new(id: DevId) -> Data {
		Data::new_with_pos(id, [0, 0, 0])
	}

	pub fn id(&self) -> DevId { self.id }

	pub fn new_with_pos(id: DevId, pos: [i32; 3]) -> Data {
		let pos = Trace {
			timestamp: 0,
			coords: Coords([pos[0] as f32, pos[1] as f32, pos[2] as f32]),
			};
		let mut dev = Data {
			id: id,
			timestamp: 0,
			scent: Scent::with_capacity(POSITION_TRACE_DEPTH),
		};
		dev.scent.add(pos);
		dev
	}

	pub fn calc_position(&self, _measures: &Vec<&measure::List>,
		_devices: &Vec<&Data>, timestamp: u32) -> Trace
	{
		// new position calculation or extrapolation when calculation is fresher than measurements
		// fake calculation or estimation when calculation is fresher than measurements
		//todo: Add position calculation
		let mut new_pos = self.scent.get(0).unwrap().clone();
		new_pos.coords[0] += 0.9;
		new_pos.coords[1] += 0.55;
		if new_pos.coords[0] > 700.0 {
			new_pos.coords[0] = 0.0;
		}
		if new_pos.coords[1] > 700.0 {
			new_pos.coords[1] = 0.0;
		}
		new_pos.timestamp = timestamp;
		new_pos
	}

	pub fn estimate_position(&self, timestamp: Timestamp) -> Trace
	{
		let mut pos = *self.scent.get(0).unwrap();
		pos.timestamp = timestamp;
		pos
	}

	pub fn save_position(&mut self, pos: Trace)
	{
		self.scent.add(pos);
	}
}
