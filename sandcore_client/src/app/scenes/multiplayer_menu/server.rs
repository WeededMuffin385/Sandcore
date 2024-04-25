use egui::Color32;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::app::scenes::multiplayer_menu::MultiplayerMenu;

#[derive(Serialize, Deserialize)]
pub struct Server {
	pub name: String,
	pub address: String,
	pub username: String,
	pub password: String,
	pub color: Color32,
}

impl Server {
	pub fn new(name: &str, address: &str, username: &str, password: &str, color: Color32) -> Self {
		Self {
			name: name.to_string(),
			address: address.to_string(),
			username: username.to_string(),
			password: password.to_string(),
			color,
		}
	}
}

impl Default for Server {
	fn default() -> Self {
		let mut rng = rand::thread_rng();

		let r = rng.gen();
		let g = rng.gen();
		let b = rng.gen();

		Self {
			name: Default::default(),
			address: Default::default(),
			username: Default::default(),
			password: Default::default(),
			color: Color32::from_rgb(r, g, b),
		}
	}
}