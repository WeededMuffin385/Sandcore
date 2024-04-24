use euclid::default::Vector2D;
use sandcore_core::dialogue::Dialogue;

#[derive(Debug)]
pub enum Request{
	SetMove {
		direction: Vector2D<f32>,
		speed: f32,
	},
}

#[derive(Debug)]
pub enum Response{
	Ok,
	Err,
}

pub type Message = Dialogue<Request, Response>;