pub mod message;
pub mod position;

use std::sync::*;
use std::time::Duration;
use euclid::default::*;
use message::Message;
use crate::world::creatures::creature::message::{Request, Response};
use crate::world::creatures::creature::position::Position;

const WALK_SPEED: f32 = 1.42;
const RUN_SPEED: f32 = 7.15;

pub struct Creature {
	pub speed: f32,
	pub position: Position,
	pub direction: Vector3D<f32>,

	sender: mpsc::Sender<Message>,
	receiver: mpsc::Receiver<Message>,
}

impl Creature {
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::channel();

		Self {
			sender,
			receiver,

			speed: Default::default(),
			position: Default::default(),
			direction: Default::default(),
		}
	}

	pub fn get_sender(&self) -> mpsc::Sender<Message> {
		self.sender.clone()
	}

	pub fn update(&mut self, duration: &Duration) {
		self.update_receiver();
		self.update_move(duration);
	}

	fn update_receiver(&mut self) {
		for message in self.receiver.try_iter() {
			match message.request {
				Request::Move{direction, speed} => {
					self.direction = if let Some(direction) = direction.try_normalize() {direction} else {Default::default()};
					self.speed = if speed.abs() <= 1.0 {speed.abs()} else {1.0};
				}
				Request::Position => {
					message.response(Response::Position(self.position.clone())).unwrap()
				}
			}
		}
	}

	fn update_move(&mut self, duration: &Duration) {
		let delta = self.direction * duration.as_secs_f32() * self.speed * RUN_SPEED;
		self.position += delta;
	}
}