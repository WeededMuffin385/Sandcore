use std::sync::mpsc;
use crate::world::creatures::creature::Creature;

pub mod creature;
mod ai;

use creature::message::Message as CreatureMessage;

#[derive(Default)]
pub struct Creatures {
	pub creatures: Vec<Creature>
}

impl Creatures {
	pub fn spawn(&mut self) -> mpsc::Sender<CreatureMessage> {
		self.creatures.push(Creature::new());
		self.creatures.last().unwrap().get_sender()
	}
}