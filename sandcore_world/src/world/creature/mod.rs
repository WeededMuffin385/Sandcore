pub mod message;

use std::sync::*;
use crate::world::creature::message::Message;

pub struct Creature {
	sender: mpsc::Sender<Message>,
	receiver: mpsc::Receiver<Message>,
}

impl Creature {
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::channel();

		Self {
			sender,
			receiver,
		}
	}

	pub fn get_sender(&self) -> mpsc::Sender<Message> {
		self.sender.clone()
	}

	pub fn update(&mut self) {

	}
}