use crate::world::creatures::Creatures;

pub mod message;
mod creatures;


#[derive(Default)]
pub struct World {
	pub creatures: Creatures
}

impl World {
	pub fn update(&mut self) {

	}
}