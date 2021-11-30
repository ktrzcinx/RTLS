/*
 author: Karol Trzciński <k.trzcinski95@gmail.com>
*/

use std::cmp::{min, max};
use log::{info, trace};

use crate::device;
use crate::measure;
use crate::utils::{Trace, Timestamp, DevId};

pub struct Zone {
	pub id: u32,
	measures: Vec<measure::List>,
	devices: Vec<device::Data>,
}

#[derive(PartialEq, Debug)]
pub enum ExitCode {
	Ok,
	UnknownDevice,
	AlreadyExist,
}

impl Zone {
	pub fn new(id: u32) -> Zone {
		let zone = Zone {
			id: id,
			measures:   Vec::new(),
			devices:    Vec::new(),
		};
		zone
	}

	pub fn add_device(&mut self, id: DevId, pos: [i32; 3]) -> ExitCode {
		let count = self.devices.iter().filter(|x| x.id() == id).count();
		assert_eq!(count, 0);
		let dev = device::Data::new_with_pos(id, pos);
		self.devices.push(dev);
		ExitCode::Ok
	}

	fn calc_dev_position(&self, dev: &device::Data, timestamp: Timestamp) -> Trace
	{
		let measures: Vec<&measure::List> = self.measures.iter()
			.filter(|&x| (x.id(0) == dev.id() || x.id(1) == dev.id()))
			.collect();
		let connected_devices_id: Vec<u32> = measures.iter()
			.map(|x| if x.id(0) == dev.id() { x.id(1) } else { x.id(0) } )
			.collect();
		let devices: Vec<&device::Data> = self.devices.iter()
			.filter(|&x| connected_devices_id.iter().any(|&v| v == x.id()))
			.collect();
		let pos = dev.calc_position(&measures, &devices, timestamp);
		pos
	}

	fn update_dev_position(&mut self, id: DevId, timestamp: Timestamp, allow_dev_creation : bool) -> ExitCode
	{
		let dev_index = match self.devices.iter().position(|x| x.id() == id) {
				Some(idx) => idx,
				None => {
					if allow_dev_creation {
						let ret = self.add_device(id, [0, 0, 0]);
						if ret != ExitCode::Ok {
							return ret;
						}
						self.devices.len() - 1
					} else {
						return ExitCode::UnknownDevice;
					}
				}
			};
		let pos = self.calc_dev_position(&self.devices[dev_index], timestamp);
		self.devices[dev_index].save_position(pos);
		ExitCode::Ok
	}

	pub fn add_measure(&mut self, id1: DevId, id2: DevId,
			   distance: f32, timestamp: Timestamp,
			   allow_dev_creation : bool) -> ExitCode {
		let id = [min(id1, id2), max(id1, id2)];
		let meas = measure::Distance::new([id[0], id[1]], timestamp, distance);
		let ml = self.measures.iter_mut().find(|x| x.id(0) == id[0] && x.id(1) == id[1]);
		match ml {
			Some(l) => {
				l.update(meas);
				trace!("Update measure {}-{} {}", id[0], id[1], distance);
				for &i in id.iter() {
					let ret = self.update_dev_position(i, timestamp, allow_dev_creation);
					if ret != ExitCode::Ok {
						return ret;
					}
				}
			},
			None => {
				info!("New connection {}-{} {}!", id[0], id[1], distance);
				let new_ml = measure::List::new(meas);
				self.measures.push(new_ml);
			},
		}
		ExitCode::Ok
	}

	pub fn get_device_ptr(&self, id: DevId) -> *const device::Data {
		let dev = self.devices.iter().find(|x| x.id() == id);
		dev.unwrap()
	}

	pub fn get_dev_position(&self, id: DevId, timestamp: Timestamp) -> Option<device::Description>
	{
		let dev = match self.devices.iter().find(|d| d.id() == id) {
			Some(d)	=> d,
			None	=> return None,
		};
		Some(device::Description::new(dev, timestamp))
	}

	pub fn get_all_devices_position(&mut self, timestamp: Timestamp) -> Vec<device::Description> {
		let mut pos: Vec<device::Description> = Vec::new();
		pos.reserve(self.devices.len());
		for dev in self.devices.iter() {
			pos.push(device::Description::new(dev, timestamp));
		}
		pos
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_all_devices_position() {
		let mut zone = Zone::new(1);
		zone.add_device(1, [1, 2, 3]);
		zone.add_device(2, [-1, -2, -3]);
		let v: Vec<device::Description> = zone.get_all_devices_position(0);
		info!("pos {:?}", pos);
		assert_eq!(v.len(), 2);
		assert_eq!(v[0].id(), 1);
		assert_eq!(v[1].id(), 2);
		assert!(v[0].pos.coords[0] < v[0].pos.coords[1]);
		assert!(v[0].pos.coords[1] < v[0].pos.coords[2]);
		assert!(v[1].pos.coords[0] > v[1].pos.coords[1]);
		assert!(v[1].pos.coords[1] > v[1].pos.coords[2]);
	}
}