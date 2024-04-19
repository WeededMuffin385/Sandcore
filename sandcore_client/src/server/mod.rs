use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc;
use sandcore_core::message::Message;
use sandcore_core::message_client::MessageClient;
use sandcore_core::message_server::MessageServer;

pub struct Server {
	connected: bool,
	stream: TcpStream,
}

impl Server {
	pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
		Ok(Self {
			connected: true,
			stream: TcpStream::connect(addr)?,
		})
	}

	pub fn update(&mut self) {
		if let Ok(message) = MessageServer::read(&mut self.stream) {
			self.update_message(message);
		} else {
			self.connected = false;
		}
	}

	fn update_message(&mut self, message: MessageServer) {
		match message {
			MessageServer::SpawnSuccess => {
				println!("[SERVER] Creature spawned");
			}
			MessageServer::SpawnFailed => {
				println!("[SERVER] Creature spawn failed");
			}
			MessageServer::Creatures(creatures) => {
				println!("creatures: ");

				for (index, creature) in creatures.iter().enumerate() {
					println!("[{index}]: ({},{})", creature.x, creature.y);
				}
			}
		}
	}
}