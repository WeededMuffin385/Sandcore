use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};
use creatures::Creatures;
use message::Message;

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

	pub fn run(mut self) {

	}

	pub fn update(&mut self, duration: &Duration) {
		self.creatures.update(duration);
	}
}