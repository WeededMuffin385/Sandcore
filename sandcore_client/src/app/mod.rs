pub mod scenes;

use std::time::Instant;
use macroquad::prelude::{get_fps, get_frame_time};
use crate::app::scenes::Scenes;

pub struct App {
	scenes: Scenes,
	instant: Instant,
}

impl App {
	async fn update(&mut self) {
		self.scenes.update().await;
	}

	pub async fn run(mut self) {
		loop {
			if self.instant.elapsed().as_secs() >= 1 {
				println!("fps: {}", get_fps());
				self.instant = Instant::now();
			}

			self.update().await;
		}
	}
}

impl Default for App {
	fn default() -> Self {
		egui_macroquad::cfg(|cfg|{
			cfg.set_pixels_per_point(3.0);
		});



		Self {
			scenes: Default::default(),
			instant: Instant::now(),
		}
	}
}