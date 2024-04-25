use euclid::default::Vector2D;
use serde::{Deserialize, Serialize};
use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub enum MessageClient {
	Creatures,
	MoveCreature{
		direction: Vector2D<f32>,
		speed: f32,
	},
	Spawn,
}

impl Message for MessageClient{}