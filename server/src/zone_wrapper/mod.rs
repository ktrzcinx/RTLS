// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//
// Library wrapper, responsible for conversion between messages handles by
// clients and engine library interface
//

use engine;

pub mod messages;
pub use messages::*;

mod dev_data_msg;

mod dev_comm;
mod dev_data;
mod web_comm;
mod web_data;

pub fn parse(zone : &mut engine::zone::Zone, msg : MessageContext) -> Option<MessageTarget> {
	let response : Option<MessageTarget>;
	match &msg.data {
		MessageSource::DevCommand(cmd) =>
			response = dev_comm::parse(zone, cmd, &msg.sender),
		MessageSource::DevData(cmd) =>
			response = dev_data::parse(zone, cmd, &msg.sender),
		MessageSource::WebCommand(cmd) =>
			response = web_comm::parse(zone, cmd, &msg.sender),
		MessageSource::WebData(cmd) =>
			response = web_data::parse(zone, cmd, &msg.sender),
	}
	response
}