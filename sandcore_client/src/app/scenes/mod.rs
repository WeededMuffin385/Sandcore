pub mod main_menu;
pub mod multiplayer_menu;
pub mod settings_menu;
pub mod scene;
pub mod connection_menu;
mod gameplay;
mod state;

use std::sync::mpsc;
use macroquad::color::BLUE;
use macroquad::prelude::{clear_background, next_frame};
use crate::app::scenes::main_menu::MainMenu;
use crate::app::scenes::multiplayer_menu::MultiplayerMenu;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::settings_menu::SettingsMenu;
use crate::app::scenes::state::State;

pub struct Scenes {
	scene: Option<Box<dyn Scene>>,
}

impl Scenes {
	pub async fn update(&mut self) {
		let mut state = State::default();

		clear_background(BLUE);
		if let Some(scene) = &mut self.scene {
			scene.update(&mut state);
			egui_macroquad::ui(|ctx| {
				scene.update_ui(&mut state, ctx);
			});
			egui_macroquad::draw();
		}
		next_frame().await;

		if let Some(next_scene) = state.next_scene {
			self.scene = Some(next_scene);
		}
	}

	pub fn push<T: Scene + 'static>(&mut self, scene: T) {
		self.scene = Some(Box::new(scene));
	}
}


impl Default for Scenes {
	fn default() -> Self {
		Self {
			scene: None,
		}
	}
}