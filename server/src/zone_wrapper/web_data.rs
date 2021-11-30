// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//

use log::{info};
use super::messages::*;

pub fn parse(zone : &mut engine::zone::Zone, cmd : &MessageFormat,
	sender : &SharedSender) -> Option<MessageTarget> {
	match cmd {
		MessageFormat::Text(txt) =>
			info!("zone {} received dev txt data {}", zone.id, txt),
		MessageFormat::Bin(bin) =>
			info!("zone {} received dev bin data {:?}", zone.id, bin),
	}
	Some(MessageTarget::Direct(MessageFormat::Text("Thanks".to_string()), sender.clone()))
}
