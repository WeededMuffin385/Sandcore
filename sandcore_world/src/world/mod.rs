use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use creatures::Creatures;
use message::Message;
use crate::world::blocks::Blocks;
use crate::world::message::{Request, Response};

pub mod message;
pub mod creatures;
pub mod blocks;


pub struct World {
	pub sender: Sender<Message>,
	pub receiver: Receiver<Message>,

	pub blocks: Blocks,
	pub creatures: Creatures,
}

impl World {
	pub fn new() -> Self {
		let (sender, receiver) = channel();
		Self {
			sender,
			receiver,

			blocks: Default::default(),
			creatures: Default::default(),
		}
	}

	pub fn get_sender(&self) -> Sender<Message> {
		self.sender.clone()
	}

	pub fn update(&mut self, duration: &Duration) {
		self.update_receiver();

		self.blocks.update();
		self.creatures.update(duration);
	}

	fn update_receiver(&mut self) {
		if let Ok(message) = self.receiver.try_recv() {
			match message.request {
				Request::Spawn => {
					message.response(Response::Spawn(self.creatures.spawn())).unwrap();
				}

				Request::Chunk(index) => {
					let chunk = if let Some(chunk) = self.blocks.get(&index) {Some(chunk.clone())} else {None};
					message.response(Response::Chunk(chunk)).unwrap();
				}
			}
		}
	}
}