// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//
// Internal messages definition used to communicate between client and engine
// thread.
//

use rayon::prelude::*;
use std::sync::{Arc, Mutex, Weak};
use websocket::OwnedMessage;

pub enum MessageFormat {
    Text(String),
    Bin(Vec<u8>),
}

pub enum MessageSender {
    WS(websocket::sender::Writer<std::net::TcpStream>),
}
pub type SharedSender = Arc<Mutex<MessageSender>>;
pub type SharedSenderPtr = Weak<Mutex<MessageSender>>;

impl MessageSender {
    fn respond(&mut self, msg: MessageFormat) {
        match self {
            MessageSender::WS(ref mut ws) => {
                let om = match msg {
                    MessageFormat::Text(t) => OwnedMessage::Text(t),
                    MessageFormat::Bin(b) => OwnedMessage::Binary(b),
                };
                ws.send_message(&om).unwrap();
            }
        }
    }

    fn respond_ws(&mut self, msg: OwnedMessage) {
        match self {
            MessageSender::WS(ref mut ws) => ws.send_message(&msg).unwrap(),
        }
    }
}

pub trait Sender {
    fn respond(&self, msg: String);
    fn respond_ws(&self, msg: OwnedMessage);
}

impl Sender for SharedSender {
    fn respond(&self, msg: String) {
        let snd: &mut MessageSender = &mut *self.lock().unwrap();
        snd.respond(MessageFormat::Text(msg));
    }

    fn respond_ws(&self, msg: OwnedMessage) {
        let snd: &mut MessageSender = &mut *self.lock().unwrap();
        snd.respond_ws(msg);
    }
}

pub enum MessageSource {
    DevCommand(MessageFormat),
    DevData(MessageFormat),
    WebCommand(MessageFormat),
    WebData(MessageFormat),
}

pub struct MessageContext {
    pub sender: SharedSender,
    pub data: MessageSource,
}

pub enum MessageTarget {
    DevCommand(MessageFormat),
    DevData(MessageFormat),
    WebCommand(MessageFormat),
    WebData(MessageFormat),
    Direct(MessageFormat, SharedSender),
}

pub struct ClientDispatcher {
    pub dev_command: Vec<SharedSenderPtr>,
    pub dev_data: Vec<SharedSenderPtr>,
    pub web_command: Vec<SharedSenderPtr>,
    pub web_data: Vec<SharedSenderPtr>,
}

impl ClientDispatcher {
    pub fn new() -> ClientDispatcher {
        ClientDispatcher {
            dev_command: Vec::new(),
            dev_data: Vec::new(),
            web_command: Vec::new(),
            web_data: Vec::new(),
        }
    }

    pub fn send_to_everyone(clients: &Vec<SharedSenderPtr>, msg: String) {
        clients.par_iter().for_each(|cli| match cli.upgrade() {
            Some(c) => c.respond(msg.clone()),
            None => (),
        });
    }

    pub fn dispatch(&self, msg: MessageTarget) {
        match msg {
            MessageTarget::DevCommand(cmd) => match cmd {
                MessageFormat::Text(t) => ClientDispatcher::send_to_everyone(&self.dev_command, t),
                _ => panic!(),
            },
            MessageTarget::DevData(cmd) => match cmd {
                MessageFormat::Text(t) => ClientDispatcher::send_to_everyone(&self.dev_data, t),
                _ => panic!(),
            },
            MessageTarget::WebCommand(cmd) => match cmd {
                MessageFormat::Text(t) => ClientDispatcher::send_to_everyone(&self.web_command, t),
                _ => panic!(),
            },
            MessageTarget::WebData(cmd) => match cmd {
                MessageFormat::Text(t) => ClientDispatcher::send_to_everyone(&self.web_data, t),
                _ => panic!(),
            },
            MessageTarget::Direct(cmd, sender) => match cmd {
                MessageFormat::Text(t) => sender.respond(t),
                _ => panic!(),
            },
        }
    }
}
