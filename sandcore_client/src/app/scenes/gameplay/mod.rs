mod renderer;

use macroquad::prelude::*;
use std::sync::mpsc::Sender;
use std::time::Instant;
use egui::{Context};
use euclid::default::{Point2D, Vector2D};
use sandcore_core::message_client::MessageClient;
use sandcore_core::message_server::MessageServer;
use crate::app::scenes::gameplay::renderer::Renderer;
use crate::app::scenes::scene::{Scene, SceneMessage};
use crate::assets::Assets;
use crate::server::Server;
use crate::world::World;

pub struct Gameplay {
	creatures_requested: bool,
	direction: Vector2D<f32>,
	instant: Instant,



	renderer: Renderer,
	server: Server,
	world: World,
}

impl Gameplay {
	pub fn new(server: Server) -> Self {
		server.try_send(MessageClient::Spawn).unwrap();

		let mut world = World::default();
		world.creatures.push(Default::default());

		Self {
			creatures_requested: false,
			direction: Default::default(),
			instant: Instant::now(),


			renderer: Default::default(),
			server,
			world,
		}
	}

	fn update_server(&mut self) {
		if let Ok(message) = self.server.try_recv() {
			match message {
				MessageServer::Creatures(creatures) => {
					self.world.creatures = creatures;
					self.creatures_requested = false;
				}
			}
		}
	}

	fn update_creatures_request(&mut self) {
		if !self.creatures_requested && (self.instant.elapsed().as_secs_f32() >= (1.0 / 60.0)) {
			self.server.try_send(MessageClient::Creatures).unwrap();
			self.creatures_requested = true;
		}
	}

	fn update_input(&mut self) {
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

		if self.direction != next_direction {
			println!("changed!");
			self.direction = next_direction;
			self.server.try_send(MessageClient::MoveCreature {direction: self.direction.clone(), speed: 1.0}).unwrap();
		}
	}
}

impl Scene for Gameplay {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {
		self.update_input();
		self.update_creatures_request();
		self.update_server();

		self.renderer.update(&self.world);
	}

	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &Context) {

	}
}