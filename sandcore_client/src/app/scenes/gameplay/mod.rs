use std::sync::mpsc::Sender;
use egui::Context;
use egui::Key::S;
use crate::app::scenes::scene::{Scene, SceneMessage};
use crate::server::Server;
use crate::world::World;

pub struct Gameplay {
	server: Server,
	world: World,
}

impl Gameplay {
	pub fn new(server: Server) -> Self {
		let world = World::default();

		Self {
			server,
			world,
		}
	}
}

impl Scene for Gameplay {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {

	}

	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &Context) {

	}
}

pub fn run() {

}