use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use tokio::net::TcpStream;
use sandcore_core::message::Message;
use sandcore_core::message_client::MessageClient;
use sandcore_world::world::creatures::creature::message as creature_message;
use sandcore_world::world::message as world_message;
use sandcore_world::world::message::Response;

pub struct Client {
	run: bool,
	stream: TcpStream,
	sender_world: Sender<world_message::Message>,
	sender_creature: Option<Sender<creature_message::Message>>,
}


impl Client {
	pub fn new(connection: (TcpStream, SocketAddr), sender_world: Sender<world_message::Message>) -> Self {
		let (stream, addr) = connection;
		println!("client connected from: {:?}", addr);
		let run = true;

		Self {
			run,
			stream,
			sender_world,
			sender_creature: None,
		}
	}

	pub async fn run(mut self) {
		while self.run {
			self.update().await;
		}
	}

	async fn update(&mut self) {
		self.update_stream().await;
	}

	async fn update_stream(&mut self) {
		if let Ok(message) = MessageClient::read(&mut self.stream).await {
			match message {
				MessageClient::MoveCreature {direction, speed} => {
					if let Some(sender_creature) = &mut self.sender_creature {
						println!("received");

						creature_message::Message::request(sender_creature, creature_message::Request::SetMove{direction, speed}).unwrap();
					}
				}

				MessageClient::RequestCreatures => {

				}

				MessageClient::Spawn => {
					if let Ok(response) =  world_message::Message::request(&mut self.sender_world, world_message::Request::Spawn) {
						if let Ok(Response::Spawn(sender_creature)) = response.await {
							self.sender_creature = Some(sender_creature);
							println!("spawned!");
						}
					}
				}
			}
		}
	}
}