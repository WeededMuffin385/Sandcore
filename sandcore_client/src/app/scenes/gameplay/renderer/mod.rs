use std::ops::Not;
use macroquad::camera::set_camera;
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::*;
use macroquad::window::{screen_height, screen_width};
use crate::assets::Assets;
use crate::world::World;

pub struct Renderer {
	assets: Assets,
	camera: Camera2D,
	centered: bool,
	magnification: f32,
}

impl Renderer {
	pub fn update(&mut self, world: &World) {
		self.update_input();
		self.update_camera();
		self.update_draw(world);
	}

	fn update_camera(&mut self) {
		let width = 1.0 / self.magnification;
		let height = width * screen_height() / screen_width();

		if self.centered {
			self.camera = Camera2D::from_display_rect(Rect::new(-width / 2.0, height / 2.0, width, -height));
		} else {
			self.camera = Camera2D::from_display_rect(Rect::new(0.0, height, width, -height));
		}
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
	}

	fn update_draw(&self, world: &World) {
		set_camera(&self.camera);

		for creature in &world.creatures {
			draw_texture_ex(&self.assets.helmet, creature.x, creature.y, WHITE, DrawTextureParams{
				dest_size: Some(Vec2::new(1.0, 1.0)),
				.. Default::default()
			});
		}
	}
}

impl Default for Renderer {
	fn default() -> Self {
		Self {
			assets: Default::default(),
			camera: Default::default(),
			centered: Default::default(),
			magnification: 1.0,
		}
	}
}