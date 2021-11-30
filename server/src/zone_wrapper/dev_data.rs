// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//

use log::{error, info, trace};
use super::messages::*;
use engine::zone::ExitCode;
use super::dev_data_msg::{DevDataMsgType, DevDataDistMeasure};
use num_traits::FromPrimitive;

fn process_dist_measure(zone : &mut engine::zone::Zone, msg : serde_json::Value,
	_sender : &SharedSender) -> Result<Option<MessageTarget>, MessageFormat>
{
	let m : DevDataDistMeasure = match serde_json::from_value(msg) {
		Ok(v)	=> v,
		Err(_)	=> {
			let msg = format!("Invalid distance message format!");
			return Err(MessageFormat::Text(msg));
		}
	};
	let ret = zone.add_measure(m.id[0], m.id[1], m.distance, m.timestamp, true);
	match ret {
		ExitCode::Ok => {
			let mut desc_list : Vec<engine::device::Description> = Vec::new();
			for dev_id in [m.id[0], m.id[1]].iter() {
				match zone.get_dev_position(*dev_id, m.timestamp) {
					Some(dec_desc) => desc_list.push(dec_desc),
					None	=> (),
				};
			}
			let msg = MessageFormat::Text(serde_json::to_string(&desc_list).unwrap());
			return Ok(Some(MessageTarget::WebData(msg)));
		},
		_ => return Err(MessageFormat::Text(format!("Measure processing failed, {:?}", ret))),
	}
}

fn process_json(zone : &mut engine::zone::Zone, mut msg : serde_json::Value,
	sender : &SharedSender) -> Result<Option<MessageTarget>, MessageFormat>
{
	let msg_type = match &msg["cmd"] {
		serde_json::Value::Number(n) => n.as_i64().unwrap_or(-1),
		_ => {
			return Err(MessageFormat::Text("Lack of 'cmd' field".to_string()));
		}
	};
	match FromPrimitive::from_i64(msg_type) {
		Some(DevDataMsgType::DistMeasure)	=> return process_dist_measure(zone, msg["data"].take(), sender),
		_	=> return Err(MessageFormat::Text("Unknown message type".to_string())),
	}
}

pub fn parse(zone : &mut engine::zone::Zone, cmd : &MessageFormat,
	sender : &SharedSender) -> Option<MessageTarget>
{
	match cmd {
		MessageFormat::Text(txt) => {
			trace!("zone {} received web txt data {}", zone.id, txt);
			let json: serde_json::Value = match serde_json::from_str(txt) {
				Ok(j)	=> j,
				Err(e)	=> {
					error!("Received invalid JSON, {}", e);
					let msg = MessageFormat::Text(format!("Invalid JSON `{}`!", txt));
					return Some(MessageTarget::Direct(msg, sender.clone()));
				},
			};
			match process_json(zone, json, sender) {
				Ok(msg)		=> return msg,
				Err(msg)	=> return Some(MessageTarget::Direct(msg, sender.clone())),
			}
		}
		MessageFormat::Bin(bin) =>
			info!("zone {} received web bin data {:?}", zone.id, bin),
	}
	None
}
