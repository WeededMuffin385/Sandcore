use euclid::default::*;
use serde::{Deserialize, Serialize};

pub use sandcore_world::world::blocks::chunk::block::Block;
pub use sandcore_world::world::blocks::CHUNK_SIZE;
pub use sandcore_world::world::blocks::chunk::Chunk;
pub use sandcore_world::world::creatures::creature::position::Position;

#[derive(Serialize, Deserialize)]
pub enum MessageServer {
	Chunk(Point3D<isize>, Option<Chunk>),
	Position(Position),
}