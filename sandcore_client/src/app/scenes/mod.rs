pub mod main_menu;
pub mod multiplayer_menu;
pub mod settings_menu;
pub mod scene;
pub mod connection_menu;
mod gameplay;
mod message;

use std::sync::mpsc;
use macroquad::color::BLUE;
use macroquad::prelude::{clear_background, next_frame};
use crate::app::scenes::main_menu::MainMenu;
use crate::app::scenes::multiplayer_menu::MultiplayerMenu;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::settings_menu::SettingsMenu;
use crate::app::scenes::message::Message;

pub struct Scenes {
	scene: Box<dyn Scene>,
	sender: mpsc::Sender<Message>,
	receiver: mpsc::Receiver<Message>,
}

impl Scenes {
	pub async fn update(&mut self) {
		clear_background(BLUE);
		self.scene.update(&mut self.sender);
		egui_macroquad::ui(|ctx| {
			self.scene.update_ui(&mut self.sender, ctx);
		});
		egui_macroquad::draw();
		next_frame().await;

		if let Ok(message) = self.receiver.try_recv() {
			match message {
				Message::ChangeScene(scene) => {self.scene = scene}
			}
		}
	}

	pub fn push<T: Scene + 'static>(&mut self, scene: T) {
		self.scene = Box::new(scene);
	}
}


impl Default for Scenes {
	fn default() -> Self {
		let (sender, receiver) = mpsc::channel();
		let scene = Box::new(MainMenu::new());

		Self {
			scene,
			sender,
			receiver,
		}
	}
}