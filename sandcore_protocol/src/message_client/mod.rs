use euclid::default::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessageClient {
	Chunk(Point3D<isize>),
	Move{
		speed: f32,
		direction: Vector3D<f32>,
	},
	Position,
	Spawn,
}