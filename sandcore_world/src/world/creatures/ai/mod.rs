mod mode;

use std::sync::mpsc;
use crate::world::creatures::ai::mode::Mode;
use crate::world::message::Message as CreatureMessage;


struct Ai {
	sender: mpsc::Sender<CreatureMessage>,
	mode: Mode,
}

impl Ai {
	pub fn update(&mut self) {

	}
}