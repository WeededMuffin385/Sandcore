mod renderer;

use macroquad::prelude::*;
use std::sync::mpsc::Sender;
use egui::{Context};
use euclid::default::Vector2D;
use sandcore_core::message_client::MessageClient;
use crate::app::scenes::gameplay::renderer::Renderer;
use crate::app::scenes::scene::{Scene, SceneMessage};
use crate::assets::Assets;
use crate::server::Server;
use crate::world::World;

pub struct Gameplay {
	direction: Vector2D<f32>,
	renderer: Renderer,
	server: Server,
	world: World,
}

impl Gameplay {
	pub fn new(server: Server) -> Self {
		server.try_send(MessageClient::Spawn).unwrap();

		let mut world = World::default();
		world.creatures.push((0.0, 0.0));

		Self {
			direction: Default::default(),
			renderer: Default::default(),
			server,
			world,
		}
	}
}

impl Scene for Gameplay {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {
		update_input(&mut self.server, &mut self.direction);

		self.renderer.update();
		self.renderer.draw(&self.world);
	}

	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &Context) {

	}
}

fn update_input(server: &mut Server, direction: &mut Vector2D<f32>) {
	let mut next_direction: Vector2D<f32> = Vector2D::default();

	if is_key_down(KeyCode::W) {
		next_direction.y -= 1.0;
	}

	if is_key_down(KeyCode::S) {
		next_direction.y += 1.0;
	}

	if is_key_down(KeyCode::A) {
		next_direction.x -= 1.0;
	}

	if is_key_down(KeyCode::D) {
		next_direction.x += 1.0;
	}

	if *direction != next_direction {
		println!("changed!");
		*direction = next_direction;
		server.try_send(MessageClient::MoveCreature {direction: direction.clone(), speed: 1.0}).unwrap();
	}
}