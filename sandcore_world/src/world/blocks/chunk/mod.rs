use std::ops::Index;
use euclid::default::Point3D;
use serde::{Deserialize, Serialize};
use crate::world::blocks::chunk::block::Block;
use crate::world::blocks::CHUNK_SIZE;

pub mod block;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Chunk{
	pub data: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Index<Point3D<usize>> for Chunk {
	type Output = Block;

	fn index(&self, index: Point3D<usize>) -> &Self::Output {
		&self.data[index.x][index.y][index.z]
	}
}