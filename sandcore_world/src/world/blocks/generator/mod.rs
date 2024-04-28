use std::collections::HashSet;
use std::ops::Not;
use std::sync::mpsc::*;
use message::*;
use euclid::default::*;
use crate::world::blocks::chunk::Chunk;
use crate::world::blocks::generator::elevation::Elevation;

mod elevation;
mod message;

pub struct Generator {
	receiver: Receiver<Response>,
	sender: Sender<Request>,

	requested: HashSet<Point3D<isize>>,
}

impl Generator {
	pub fn generate(&mut self, index: &Point3D<isize>) {
		if self.requested.contains(index).not() {
			self.requested.insert(index.clone());
			self.sender.send(index.clone()).unwrap()
		}
	}

	pub fn try_recv(&mut self) -> TryIter<'_, Response> {
		self.receiver.try_iter()
	}
}

impl Default for Generator {
	fn default() -> Self {
		let (sender, receiver_other) = channel();
		let (sender_other, receiver) = channel();

		std::thread::spawn(||{handle_generator(sender_other, receiver_other)});

		Self {
			sender,
			receiver,
			requested: Default::default(),
		}
	}
}

fn handle_generator(sender: Sender<Response>, receiver: Receiver<Request>) {
	let mut elevation = Elevation::default();


	for index in receiver.iter() {
		let mut chunk = Chunk::default();
		elevation.generate_ground(&index, &mut chunk);
		sender.send((index, chunk)).unwrap();
	}
}