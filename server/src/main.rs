// Author: Karol Trzcinski
// email: k.trzcinski95@gmail.com
//
// RTLS sever implementation
//

use engine;
use env_logger::Env;
use log::{error, info};
use std::thread;
use std::sync::{Arc, mpsc, Mutex};
use websocket::sync::Server;

mod ws_handler;
mod zone_wrapper;

pub use crate::zone_wrapper::messages::*;

fn main() {
	env_logger::Builder::from_env(Env::default().default_filter_or("info"))
		.format_timestamp(None).init();
	// settle websocket server
	let server = Server::bind("127.0.0.1:2794").unwrap();

	// run client dispatcher thread
	let (dispatcher_cmd_putter, dispatcher_cmd_reader) = mpsc::channel::<MessageTarget>();
	let listeners = Arc::new(Mutex::new(ClientDispatcher::new()));
	let listener = listeners.clone();
	thread::Builder::new().name("dispatcher".to_string()).spawn(move || {
		loop {
			let msg = dispatcher_cmd_reader.recv().unwrap();
			let list = listener.lock().unwrap();
			list.dispatch(msg);
		}
	}).unwrap();

	// Create a shared channel that can be sent along from many clinets threads
	let (engine_cmd_putter, engine_cmd_reader) = mpsc::channel::<MessageContext>();

	// run RTLS engine in separate thread
	thread::Builder::new().name("engine".to_string()).spawn(move || {
		// use only single zone at this moment
		let mut zone = engine::zone::Zone::new(0);
		loop {
			let msg = engine_cmd_reader.recv().unwrap();
			let response = zone_wrapper::parse(&mut zone, msg);
			match response {
				Some(r) => dispatcher_cmd_putter.send(r).unwrap(),
				None => (),
			};
		}
	}).unwrap();

	// handle requests to server
	for request in server.filter_map(Result::ok) {
		let cmd_putter = engine_cmd_putter.clone();
		let listener = listeners.clone();

		// Spawn a new thread for each connection.
		thread::Builder::new().name("client handler".to_string()).spawn(move || {
			let uri = request.uri();
			let client = request.accept().unwrap();
			let ip = client.peer_addr().unwrap();
			info!("Connection from {} to '{}'", ip, uri);
			match uri.as_str() {
				"/dev_comm" => ws_handler::dev_comm_client(client, listener, cmd_putter),
				"/dev_data" => ws_handler::dev_data_client(client, listener, cmd_putter),
				"/web_comm" => ws_handler::web_comm_client(client, listener, cmd_putter),
				"/web_data" => ws_handler::web_data_client(client, listener, cmd_putter),
				_ => {
					error!("Client {} has specify invalid endpoint '{}', try one of {{/dev_comm, /dev_data, /web_comm, /web_data}}",
						 client.peer_addr().unwrap(), uri);
					return;
				}
			}
		}).unwrap();
	}
}
