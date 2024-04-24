pub mod scenes;

use crate::app::scenes::Scenes;
use crate::assets::Assets;
use scenes::main_menu::MainMenu;

pub struct App {
	assets: Assets,
	scenes: Scenes,
}

impl App {
	async fn update(&mut self) {
		self.scenes.update().await;
	}

	pub async fn run(mut self) {
		loop {
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
			assets: Default::default(),
			scenes: Default::default(),
		}
	}
}