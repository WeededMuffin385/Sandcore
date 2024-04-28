use std::sync::mpsc::Sender;
use euclid::default::*;
use sandcore_dialogue::dialogue::Dialogue;
use crate::world::creatures::creature::message::Message as CreatureMessage;
use crate::world::blocks::chunk::Chunk;

/// TODO: Add split by region, so only visible creatures will be returned
	/// However, most visibility processing will happen in creature/client logics.
#[derive(Debug)]
pub enum Request {
	Spawn,
	Chunk(Point3D<isize>),
}

#[derive(Debug)]
pub enum Response {
	Spawn(Sender<CreatureMessage>),
	Chunk(Option<Chunk>),
}

pub type Message = Dialogue<Request, Response>;