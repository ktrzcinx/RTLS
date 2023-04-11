// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//

use super::messages::*;
use log::info;

pub fn parse(
    zone: &mut engine::zone::Zone,
    cmd: &MessageFormat,
    _: &SharedSender,
) -> Option<MessageTarget> {
    match cmd {
        MessageFormat::Text(txt) => info!("zone {} received dev txt data {}", zone.id, txt),
        MessageFormat::Bin(bin) => info!("zone {} received dev bin data {:?}", zone.id, bin),
    }
    None
}
