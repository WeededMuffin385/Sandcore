use std::sync::mpsc::Sender;
use euclid::default::Point2D;
use sandcore_core::dialogue::Dialogue;
use crate::world::creatures::creature::message::Message as CreatureMessage;

#[derive(Debug)]
pub enum Request {
	Spawn,

	/// TODO: Add split by region, so only visible creatures will be returned
	/// However, most visibility processing will happen in creature/client logics.
	GetCreatures,
}

#[derive(Debug)]
pub enum Response {
	Spawn(Sender<CreatureMessage>),
	GetCreatures(Vec<Point2D<f32>>)
}

pub type Message = Dialogue<Request, Response>;