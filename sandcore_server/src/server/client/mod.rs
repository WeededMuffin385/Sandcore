use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use tokio::net::TcpStream;
use sandcore_protocol::message::Message;
use sandcore_protocol::message_client::MessageClient;
use sandcore_protocol::message_server::MessageServer;


use sandcore_world::world::creatures::creature::message as creature_message;
use sandcore_world::world::message as world_message;


pub struct Client {
	stream: TcpStream,
	connected: bool,
	sender_world: Sender<world_message::Message>,
	sender_creature: Option<Sender<creature_message::Message>>,
}


impl Client {
	pub fn new(connection: (TcpStream, SocketAddr), sender_world: Sender<world_message::Message>) -> Self {
		let (stream, addr) = connection;
		println!("client connected from: {:?}", addr);
		let connected = true;

		Self {
			stream,
			connected,
			sender_world,
			sender_creature: None,
		}
	}

	pub async fn run(mut self) {
		while self.connected {
			self.update().await;
		}
	}

	async fn update(&mut self) {
		self.update_stream().await;
	}

	async fn update_stream(&mut self) {
		if let Ok(message) = MessageClient::read(&mut self.stream).await {
			match message {
				MessageClient::Move {direction, speed} => {
					if let Some(sender_creature) = &mut self.sender_creature {
						creature_message::Message::request(sender_creature, creature_message::Request::Move{direction, speed}).unwrap();
					}
				}

				MessageClient::Spawn => {
					if let Ok(response) =  world_message::Message::request(&mut self.sender_world, world_message::Request::Spawn) {
						if let Ok(world_message::Response::Spawn(sender_creature)) = response.await {
							self.sender_creature = Some(sender_creature);
							println!("spawned!");
						}
					}
				}

				MessageClient::Position => {
					if let Some(sender_creature) = &mut self.sender_creature {
						if let Ok(response) = creature_message::Message::request(sender_creature, creature_message::Request::Position) {
							if let Ok(creature_message::Response::Position(position)) = response.await {
								MessageServer::Position(position).write(&mut self.stream).await.unwrap();
							}
						}
					}
				}

				MessageClient::Chunk(index) => {
					if let Ok(response) = world_message::Message::request(&mut self.sender_world, world_message::Request::Chunk(index)) {
						if let Ok(world_message::Response::Chunk(chunk)) = response.await {
							MessageServer::Chunk(index, chunk).write(&mut self.stream).await.unwrap();
						}
					}
				}
			}
		} else {
			self.connected = false;
		}
	}
}