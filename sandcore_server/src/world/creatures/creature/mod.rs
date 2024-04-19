pub mod message;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;
use euclid::default::{Point2D, Vector2D};
use message::*;
use crate::world::creatures::Creatures;
use crate::world::message::{Message as WorldMessage, Request as WorldRequest, Response as WorldResponse};
use crate::world::World;

pub struct Creature{
	sender: Sender<Message>,
	receiver: Receiver<Message>,

	position: Point2D<f32>,
	direction: Vector2D<f32>,
}

impl Creature {
	pub fn new() -> Self {
		let (sender, receiver) = channel::<Message>();

		Self {
			sender,
			receiver,

			position: Default::default(),
			direction: Default::default(),
		}
	}


	pub fn update_position(&mut self, duration: &Duration) {
		let delta = self.direction * duration.as_secs_f32();
		self.position += delta;
	}

	pub fn get_position(&self) -> &Point2D<f32> {
		&self.position
	}

	pub fn update(&mut self, creatures: &Vec<Creature>) {
		self.update_receiver();
	}

	fn update_receiver(&mut self) {
		if let Ok(message) = self.receiver.try_recv() {
			match message.request {
				Request::SetDirection(_) => {}
			}
		}
	}

	pub fn get_sender(&self) -> Sender<Message> {
		self.sender.clone()
	}
}