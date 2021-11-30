use serde_derive::{Serialize, Deserialize};
use std::cmp::{min, max};
use crate::utils::{Timestamp, DevId};

#[derive(Serialize, Deserialize)]
pub struct Distance {
	pub id: [DevId; 2],
	pub timestamp: Timestamp,
	pub distance: f32,
}

const MEASURE_DEPTH : usize = 5;
pub struct List {
	dev: [DevId; 2],
	measures_ts: [u32; MEASURE_DEPTH],
	measures_val: [f32; MEASURE_DEPTH],
}

fn array_insert_pop<T>(arr: &mut [T; MEASURE_DEPTH], new_val: T) -> &[T; MEASURE_DEPTH] where T: Copy {
	for i in 1..MEASURE_DEPTH {
		arr[i] = arr[i-1];
	}
	arr[0] = new_val;
	arr
}

impl Distance {
	pub fn new(id: [DevId; 2], timestamp: Timestamp, distance: f32) -> Distance {
		Distance {
			id: id,
			timestamp: timestamp,
			distance: distance,
		}
	}
}

impl List {
	pub fn new(meas: Distance) -> List {
		let lo = min(meas.id[0], meas.id[1]);
		let hi = max(meas.id[0], meas.id[1]);
		List {
		dev: [lo, hi],
		measures_ts: [meas.timestamp; MEASURE_DEPTH],
		measures_val: [meas.distance; MEASURE_DEPTH],
		}
	}

	pub fn id(&self, i: usize) -> u32 { if i >= 2 { panic!(); } self.dev[i] }

	pub fn update(&mut self, meas: Distance) {
		array_insert_pop(&mut self.measures_val, meas.distance);
		array_insert_pop(&mut self.measures_ts, meas.timestamp);
	}

	pub fn estimate(&self, _timestamp: u32) -> f32 {
		*self.measures_val.last().unwrap()
	}
}
