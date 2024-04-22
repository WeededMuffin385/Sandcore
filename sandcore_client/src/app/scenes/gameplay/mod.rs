use egui::Context;
use egui::Key::S;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::state::State;
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
	fn update(&mut self, state: &mut State) {

	}

	fn update_ui(&mut self, state: &mut State, ctx: &Context) {

	}
}

pub fn run() {

}