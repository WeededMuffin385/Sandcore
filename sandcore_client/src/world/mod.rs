mod blocks;

use sandcore_protocol::message_server::Position;
use crate::world::blocks::Blocks;

#[derive(Default)]
pub struct World{
	pub creature: Position,
	pub blocks: Blocks,
}