// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//
// RTLS websocket client handler
//

use log::{info, error};
use std::sync::{Arc, mpsc, Mutex};
use websocket::sync::Client;
use websocket::OwnedMessage;

pub use crate::zone_wrapper::messages::*;

fn handle_client_message(
	rcv_message : std::result::Result<websocket::OwnedMessage, websocket::WebSocketError>,
	ip : std::net::SocketAddr,
	shared_sender : &SharedSender,
	msg_channel_tx  : &mpsc::Sender<MessageContext>,
	packer : impl Fn(MessageFormat) -> MessageSource)
	-> Result<(), bool>
{
	let message = match rcv_message {
		Ok(msg)	=> msg,
		Err(e)	=> {
			match e {
				websocket::WebSocketError::IoError(o) => {
					error!("IO error, client {}: {}", ip, o);
				}
				_	=> error!("Error, client {}: {:?}", ip, e),
			}
			return Err(false);
		}
	};

	match message {
		OwnedMessage::Close(_) => {
			shared_sender.respond_ws(OwnedMessage::Close(None));
			info!("Client {} disconnected", ip);
			return Err(false);
		}
		OwnedMessage::Ping(ping) => {
			shared_sender.respond_ws(OwnedMessage::Pong(ping));
		}
		OwnedMessage::Text(msg) => {
			let msg = MessageContext {
				sender: shared_sender.clone(),
				data: packer(MessageFormat::Text(msg)),
			};
			// send message to processing thread
			msg_channel_tx.send(msg).unwrap();
		}
		_ => {
			info!("Received message: {:?}", message);
		}
	}
	return Err(true);
}

fn pack_dev_comm(content : MessageFormat) -> MessageSource
{
	MessageSource::DevCommand(content)
}

pub fn dev_comm_client(mut client : Client<std::net::TcpStream>,
		listeners : Arc<Mutex<ClientDispatcher>>,
		msg_channel_tx : mpsc::Sender<MessageContext>)
{
	let ip = client.peer_addr().unwrap();

	// send initial message
	let message = OwnedMessage::Text("Hello, I'm RTLS server".to_string());
	client.send_message(&message).unwrap();

	// Create sender for this client
	let (mut receiver, unlocked_sender) = client.split().unwrap();
	let message_sender = MessageSender::WS(unlocked_sender);
	let shared_sender = Arc::new(Mutex::new(message_sender));

	// Add to proper event listeners lists
	{
		let list = &mut *listeners.lock().unwrap();
		list.dev_command.push(Arc::downgrade(&shared_sender));
	}

	// handle received messages
	for rcv_message in receiver.incoming_messages() {
		match handle_client_message(rcv_message, ip, &shared_sender, &msg_channel_tx, pack_dev_comm) {
			Ok(()) => (),
			Err(keep_connection) => if !keep_connection {
				return;
			}
		}
	}
}

fn pack_dev_data(content : MessageFormat) -> MessageSource
{
	MessageSource::DevData(content)
}

pub fn dev_data_client(mut client : Client<std::net::TcpStream>,
		listeners : Arc<Mutex<ClientDispatcher>>,
		msg_channel_tx : mpsc::Sender<MessageContext>)
{
	let ip = client.peer_addr().unwrap();

	// send initial message
	let message = OwnedMessage::Text("Hello, I'm RTLS server".to_string());
	client.send_message(&message).unwrap();

	// Create sender for this client
	let (mut receiver, unlocked_sender) = client.split().unwrap();
	let message_sender = MessageSender::WS(unlocked_sender);
	let shared_sender = Arc::new(Mutex::new(message_sender));

	// Add to proper event listeners lists
	{
		let list = &mut *listeners.lock().unwrap();
		list.dev_data.push(Arc::downgrade(&shared_sender));
	}

	// handle received messages
	for rcv_message in receiver.incoming_messages() {
		match handle_client_message(rcv_message, ip, &shared_sender, &msg_channel_tx, pack_dev_data) {
			Ok(()) => (),
			Err(keep_connection) => if !keep_connection {
				return;
			}
		}
	}
}

fn pack_web_comm(content : MessageFormat) -> MessageSource
{
	MessageSource::WebCommand(content)
}

pub fn web_comm_client(mut client : Client<std::net::TcpStream>,
		listeners : Arc<Mutex<ClientDispatcher>>,
		msg_channel_tx : mpsc::Sender<MessageContext>)
{
	let ip = client.peer_addr().unwrap();

	// send initial message
	let message = OwnedMessage::Text("Hello, I'm RTLS server".to_string());
	client.send_message(&message).unwrap();

	// Create sender for this client
	let (mut receiver, unlocked_sender) = client.split().unwrap();
	let message_sender = MessageSender::WS(unlocked_sender);
	let shared_sender = Arc::new(Mutex::new(message_sender));

	// Add to proper event listeners lists
	{
		let list = &mut *listeners.lock().unwrap();
		list.web_command.push(Arc::downgrade(&shared_sender));
	}

	// handle received messages
	for rcv_message in receiver.incoming_messages() {
		match handle_client_message(rcv_message, ip, &shared_sender, &msg_channel_tx, pack_web_comm) {
			Ok(()) => (),
			Err(keep_connection) => if !keep_connection {
				return;
			}
		}
	}
}

fn pack_web_data(content : MessageFormat) -> MessageSource
{
	MessageSource::WebData(content)
}

pub fn web_data_client(mut client : Client<std::net::TcpStream>,
		listeners : Arc<Mutex<ClientDispatcher>>,
		msg_channel_tx : mpsc::Sender<MessageContext>)
{
	let ip = client.peer_addr().unwrap();

	// send initial message
	let message = OwnedMessage::Text("Hello, I'm RTLS server".to_string());
	client.send_message(&message).unwrap();

	// Create sender for this client
	let (mut receiver, unlocked_sender) = client.split().unwrap();
	let message_sender = MessageSender::WS(unlocked_sender);
	let shared_sender = Arc::new(Mutex::new(message_sender));

	// Add to proper event listeners lists
	{
		let list = &mut *listeners.lock().unwrap();
		list.web_data.push(Arc::downgrade(&shared_sender));
	}

	// handle received messages
	for rcv_message in receiver.incoming_messages() {
		match handle_client_message(rcv_message, ip, &shared_sender, &msg_channel_tx, pack_web_data) {
			Ok(()) => (),
			Err(keep_connection) => if !keep_connection {
				return;
			}
		}
	}
}