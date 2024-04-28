pub mod chunk;
mod generator;

use std::collections::HashMap;
use euclid::default::Point3D;
use crate::world::blocks::chunk::Chunk;
use crate::world::blocks::generator::Generator;

pub const CHUNK_SIZE: usize = 16;

#[derive(Default)]
pub struct Blocks {
	generator: Generator,
	chunks: HashMap<Point3D<isize>, Chunk>,
}

impl Blocks {
	pub fn get(&mut self, index: &Point3D<isize>) -> Option<&Chunk> {
		self.generator.generate(index);
		self.chunks.get(index)
	}

	pub fn update(&mut self) {
		for (index, chunk) in self.generator.try_recv() {
			self.chunks.insert(index, chunk);
		}
	}
}