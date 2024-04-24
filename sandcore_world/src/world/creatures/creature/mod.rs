pub mod message;

use std::sync::*;
use std::time::{Duration, Instant};
use euclid::default::{Point2D, Vector2D};
use message::Message;
use crate::world::creatures::creature::message::Request;

pub struct Creature {
	speed: f32,
	position: Point2D<f32>,
	direction: Vector2D<f32>,

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
		for message in &self.receiver {
			match message.request {
				Request::SetMove{direction, speed} => {
					let direction = direction.normalize();
					let speed = if speed.abs() > 1.0 {1.0} else {speed.abs()};

					self.direction = direction;
					self.speed = speed;
				}
			}
		}
	}

	fn update_move(&mut self, duration: &Duration) {
		let delta = self.direction * duration.as_secs_f32() * self.speed;
		self.position += delta;
	}
}