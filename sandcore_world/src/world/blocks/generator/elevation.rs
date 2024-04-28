use std::collections::HashMap;
use euclid::default::{Point2D, Point3D};
use noise::{Fbm, NoiseFn, Simplex};
use crate::world::blocks::chunk::block::Block;
use crate::world::blocks::CHUNK_SIZE;

use crate::world::blocks::chunk::Chunk as WorldChunk;

pub struct Elevation {
	noise: Fbm<Simplex>,
	chunks: HashMap<Point2D<isize>, Chunk>,
}

#[derive(Default)]
struct Chunk {
	data: [[f64;CHUNK_SIZE]; CHUNK_SIZE]
}

impl Elevation {
	pub fn generate_ground(&mut self, index: &Point3D<isize>, world_chunk: &mut WorldChunk) {
		let point = index.xy();
		let chunk = self.chunks.entry(point).or_insert_with(||{generate(&self.noise, &point)});

		let next = &mut world_chunk.data;

		for (x, next) in next.iter_mut().enumerate() {
			for (y, next) in next.iter_mut().enumerate() {
				for (z, block) in next.iter_mut().enumerate() {
					let height = z as isize + (index.z * CHUNK_SIZE as isize);
					let noise = chunk.data[x][y] as isize;

					if height < noise  {
						*block = Block::Dirt;
					}

					if height == noise {
						*block = Block::Grass;
					}
				}
			}
		}
	}
}

fn generate(noise: &Fbm<Simplex>, index: &Point2D<isize>) -> Chunk {
	let mut chunk = Chunk::default();
	for (x, next) in chunk.data.iter_mut().enumerate() {
		for (y, value) in next.iter_mut().enumerate() {
			let x = x as isize + (index.x * CHUNK_SIZE as isize);
			let y = y as isize + (index.y * CHUNK_SIZE as isize);

			*value = noise.get([(x as f64) / 32.0, (y as f64) / 32.0]) * 32.0;
		}
	}
	chunk
}

impl Default for Elevation {
	fn default() -> Self {
		let mut noise = Fbm::new(1337);
		noise.octaves = 5;

		Self {
			noise,
			chunks: Default::default(),
		}
	}
}