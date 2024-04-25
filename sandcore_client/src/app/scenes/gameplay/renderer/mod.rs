use macroquad::camera::set_camera;
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{Camera2D, draw_texture_ex, DrawTextureParams};
use macroquad::window::{screen_height, screen_width};
use crate::assets::Assets;
use crate::world::World;

pub struct Renderer {
	assets: Assets,
	camera: Camera2D,
}

impl Renderer {
	pub fn update(&mut self) {
		let width = 4.0;
		let height = width * screen_height() / screen_width();

		self.camera = Camera2D::from_display_rect(Rect::new(0.0, height, width, -height));
	}
	pub fn draw(&self, world: &World) {
		set_camera(&self.camera);

		for (x, y) in &world.creatures {
			draw_texture_ex(&self.assets.helmet, *x, *y, WHITE, DrawTextureParams{
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
		}
	}
}