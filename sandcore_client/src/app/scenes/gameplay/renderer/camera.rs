use std::ops::Not;
use macroquad::camera::set_camera;
use macroquad::color::WHITE;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::Vec2;
use macroquad::prelude::*;
use sandcore_protocol::message_server::{CHUNK_SIZE, Position};

pub struct Camera {
	pub position: Position,
	camera: Camera2D,

	magnification: f32,
	centered: bool,
	buffer: RenderTarget,
}

impl Camera {
	pub fn update(&mut self) {
		self.update_input();
		self.update_camera();
	}

	pub fn set_position(&mut self, position: Position) {
		self.position.position_world.x = position.position_world.x;
		self.position.position_world.y = position.position_world.y;

		self.position.position_chunk.x = position.position_chunk.x;
		self.position.position_chunk.y = position.position_chunk.y;
	}

	pub fn draw_buffer(&self) {
		set_default_camera();
		draw_texture(&self.buffer.texture, 0.0, 0.0, WHITE);
	}

	fn update_camera(&mut self) {
		let zoom = if screen_height() < screen_width() {
			vec2(screen_height() / screen_width(), 1.0)
		} else {
			vec2(1.0, screen_width() / screen_height())
		};

		if self.buffer.texture.size() != vec2(screen_width(), screen_height()) {
			self.buffer = render_target(screen_width() as u32, screen_height() as u32);
		}

		self.camera.render_target = Some(self.buffer.clone());
		self.camera.zoom = zoom * self.magnification;
		self.camera.target = Vec2::from_slice(&self.position.position_chunk.xy().to_array());
	}

	fn update_input(&mut self) {
		if is_key_pressed(KeyCode::I) {
			self.centered = self.centered.not();
		}

		if is_key_down(KeyCode::LeftControl) {
			if is_key_pressed(KeyCode::Equal) {
				self.magnification *= 2.0;
			}

			if is_key_pressed(KeyCode::Minus) {
				self.magnification /= 2.0;
			}
		}

		if is_key_down(KeyCode::LeftShift) {
			if is_key_pressed(KeyCode::Equal) {
				self.position.position_chunk.z += 1.0;

				while self.position.position_chunk.z >= CHUNK_SIZE as f32 {
					self.position.position_chunk.z -= CHUNK_SIZE as f32;
					self.position.position_world.z += 1;
				}
			}

			if is_key_pressed(KeyCode::Minus) {
				self.position.position_chunk.z -= 1.0;

				while self.position.position_chunk.z < 0.0 {
					self.position.position_chunk.z += CHUNK_SIZE as f32;
					self.position.position_world.z -= 1;
				}
			}
		}
	}

	pub fn set_camera(&self) {
		set_camera(&self.camera);
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			buffer: render_target(screen_width() as u32, screen_height() as u32),
			camera: Default::default(),
			position: Default::default(),

			centered: false,
			magnification: 1.0,
		}
	}
}