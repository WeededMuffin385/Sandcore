use std::sync::mpsc;
use crate::world::message::Message as CreatureMessage;


struct Ai {
	sender: mpsc::Sender<CreatureMessage>,
}

impl Ai {
	pub fn update(&mut self) {

	}
}