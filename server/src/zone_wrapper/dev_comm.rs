// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//

use log::{info, error};
use super::messages::*;

fn parse_dev_wake_up(zone : &mut engine::zone::Zone, msg : serde_json::Value,
	_sender : &SharedSender) -> Result<Option<MessageTarget>, MessageFormat>
{
	let id = match &msg["id"] {
		serde_json::Value::Number(n) => n.as_u64().unwrap() as u32,
		_ => {
			println!("Message field 'id' parsing failure");
			return Err(MessageFormat::Text("Invalid id".to_string()));
		}
	};
	zone.add_device(id, [0, 0, 0]);
	let msg = MessageFormat::Text("Added new device".to_string());
	return Ok(Some(MessageTarget::WebData(msg)));
}

fn process_json(zone : &mut engine::zone::Zone, mut msg : serde_json::Value,
	sender : &SharedSender) -> Result<Option<MessageTarget>, MessageFormat>
{
	let msg_type = match &msg["type"] {
		serde_json::Value::Number(n) => n.as_u64().unwrap_or(std::u32::MAX as u64),
		_ => {
			return  Err(MessageFormat::Text("Message field 'type' parsing failure".to_string()));
		}
	};
	println!("msg type {}", msg_type);
	Ok(None)
}

pub fn parse(zone : &mut engine::zone::Zone, cmd : &MessageFormat,
	sender : &SharedSender) -> Option<MessageTarget> {
	match cmd {
		MessageFormat::Text(txt) => {
			info!("zone {} received dev txt cmd {}", zone.id, txt);
			let json: serde_json::Value = match serde_json::from_str(txt) {
				Ok(j) => j,
				Err(e) => {
					error!("Received invalid JSON, {}", e);
					let msg = MessageFormat::Text(format!("Invalid JSON `{}`!", txt));
					return Some(MessageTarget::Direct(msg, sender.clone()));
				},
			};
			match process_json(zone, json, sender)
			{
				Ok(_) => (),
				Err(e) => println!("json parse failed"),
			}
		},
		MessageFormat::Bin(bin) =>
			info!("zone {} received dev bin cmd {:?}", zone.id, bin),
	}
	Some(MessageTarget::Direct(MessageFormat::Text("Yes, sir!".to_string()), sender.clone()))
}
