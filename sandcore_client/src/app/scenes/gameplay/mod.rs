mod renderer;

use std::ops::Not;
use macroquad::prelude::*;
use std::sync::mpsc::Sender;
use std::time::Instant;
use egui::{Context};
use euclid::default::Vector3D;
use sandcore_protocol::message_client::MessageClient;
use sandcore_protocol::message_server::MessageServer;
use crate::app::scenes::gameplay::renderer::Renderer;
use crate::app::scenes::scene::{Scene, SceneMessage};
use crate::server::Server;
use crate::world::World;

pub struct Gameplay {
	chunks_pending: usize,
	position_requested: bool,
	position_instant: Instant,

	direction: Vector3D<f32>,

	renderer: Renderer,
	server: Server,
	world: World,
}

impl Gameplay {
	pub fn new(server: Server) -> Self {
		server.try_send(MessageClient::Spawn).unwrap();

		let mut world = World::default();

		Self {
			chunks_pending: 0,
			position_requested: false,
			position_instant: Instant::now(),

			direction: Default::default(),
			renderer: Default::default(),
			server,
			world,
		}
	}

	fn update_server(&mut self) {
		if let Ok(message) = self.server.try_recv() {
			match message {
				MessageServer::Position(position) => {
					self.world.creature = position;
					self.position_requested = false;
				}

				MessageServer::Chunk(index, chunk) => {
					if let Some(chunk) = chunk {
						self.world.blocks.chunks.insert(index, chunk);
					} else {
						self.world.blocks.requested.remove(&index);
					}

					self.chunks_pending -= 1;
				}
			}
		}
	}

	fn update_creatures_request(&mut self) {
		if self.position_requested.not() && (self.position_instant.elapsed().as_secs_f32() >= (1.0 / 60.0)) {
			self.server.try_send(MessageClient::Position).unwrap();
			self.position_requested = true;
			self.position_instant = Instant::now();
		}
	}

	fn update_chunk_request(&mut self) {
		for r in 0..self.renderer.radius {
			for z in -1..=0 {
				for x in -r..=r {
					for y in -r..=r {
						if self.chunks_pending == 1 { return }

						let offset = Vector3D::new(x, y, z);
						let position_world = self.renderer.camera.position.position_world + offset;

						if self.world.blocks.requested.contains(&position_world).not() {
							self.server.try_send(MessageClient::Chunk(position_world)).unwrap();
							self.world.blocks.requested.insert(position_world);
							self.chunks_pending += 1;
						}
					}
				}
			}
		}
	}

	fn update_input(&mut self) {
		let mut next_direction: Vector3D<f32> = Default::default();

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
			self.direction = next_direction;
			self.server.try_send(MessageClient::Move {direction: self.direction.clone(), speed: 1.0}).unwrap();
		}
	}
}

impl Scene for Gameplay {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {
		self.update_input();
		self.update_creatures_request();
		self.update_chunk_request();
		self.update_server();
		self.renderer.set_camera_position(self.world.creature.clone());
		self.renderer.update(&self.world);
	}

	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &Context) {
		egui::SidePanel::left("left_panel").default_width(ctx.available_rect().width() * 0.2).resizable(true).show(ctx, |ui|{
			ui.label(format!("{:?}", self.renderer.camera.position.position_chunk));
		});
	}
}