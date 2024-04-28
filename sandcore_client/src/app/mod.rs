pub mod scenes;

use std::time::Instant;
use crate::app::scenes::Scenes;

pub struct App {
	scenes: Scenes,

	tps: f64,
	instant: Instant,
}

impl App {
	async fn update(&mut self) {
		self.scenes.update().await;
	}

	pub async fn run(mut self) {
		loop {
			let elapsed = self.instant.elapsed().as_secs_f64();
			if  elapsed >= 1.0 {
				println!("tps: {}", self.tps / elapsed);
				self.instant = Instant::now();
				self.tps = 0.0;
			}
			self.tps += 1.0;


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
			tps: 0.0,
			instant: Instant::now(),
		}
	}
}