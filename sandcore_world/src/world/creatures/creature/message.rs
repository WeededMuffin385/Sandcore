use euclid::default::*;
use sandcore_dialogue::dialogue::Dialogue;
use crate::world::creatures::creature::position::Position;

#[derive(Debug)]
pub enum Request{
	Move {
		direction: Vector3D<f32>,
		speed: f32,
	},

	Position,
}

#[derive(Debug)]
pub enum Response{
	Position(Position),
}

pub type Message = Dialogue<Request, Response>;