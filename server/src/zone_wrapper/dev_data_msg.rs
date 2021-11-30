use serde_derive::{Serialize, Deserialize};
use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum DevDataMsgType {
	DistMeasure = 1,
}

#[derive(Serialize, Deserialize)]
pub struct DevDataDistMeasure {
	pub id: [u32; 2],
	pub timestamp: u32,
	pub distance: f32,
}