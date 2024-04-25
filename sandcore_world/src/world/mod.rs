use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};
use creatures::Creatures;
use message::Message;
use crate::world::message::{Request, Response};

pub mod message;
pub mod creatures;


pub struct World {
	pub sender: Sender<Message>,
	pub receiver: Receiver<Message>,
	pub creatures: Creatures,
}

impl World {
	pub fn new() -> Self {
		let (sender, receiver) = channel();
		Self {
			sender,
			receiver,
			creatures: Default::default(),
		}
	}

	pub fn get_sender(&self) -> Sender<Message> {
		self.sender.clone()
	}

	pub fn update(&mut self, duration: &Duration) {
		self.update_receiver();
		self.creatures.update(duration);
	}

	fn update_receiver(&mut self) {
		for message in self.receiver.try_iter() {
			match message.request {
				Request::Spawn => {
					message.response(Response::Spawn(self.creatures.spawn())).unwrap();
				}
				Request::Creatures => {
					message.response(Response::Creatures(self.creatures.get_creatures())).unwrap();
				}
			}
		}
	}
}