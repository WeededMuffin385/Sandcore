use crate::world::creature::Creature;
pub mod creature;
pub mod ai;
pub mod message;

pub struct World {
	creatures: Vec<Creature>
}

impl World {
	pub fn new() -> Self {
		Self {
			creatures: Default::default(),
		}
	}

	pub fn update(&mut self) {

	}
}