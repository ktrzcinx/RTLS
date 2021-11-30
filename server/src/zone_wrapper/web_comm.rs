// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//

use log::{info};
use super::messages::*;

pub fn parse(zone : &mut engine::zone::Zone, cmd : &MessageFormat,
	sender : &SharedSender) -> Option<MessageTarget> {
	match cmd {
		MessageFormat::Text(txt) =>
			info!("zone {} received web txt cmd {}", zone.id, txt),
		MessageFormat::Bin(bin) =>
			info!("zone {} received web bin cmd {:?}", zone.id, bin),
	}
	Some(MessageTarget::Direct(MessageFormat::Text("Yes, sir!".to_string()), sender.clone()))
}
