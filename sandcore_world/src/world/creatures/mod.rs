use std::sync::mpsc;
use std::time::Duration;
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

	pub fn update(&mut self, duration: &Duration) {
		for creature in &mut self.creatures {
			creature.update(duration);
		}
	}
}