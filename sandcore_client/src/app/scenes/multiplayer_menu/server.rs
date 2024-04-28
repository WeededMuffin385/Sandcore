use egui::Color32;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Server {
	pub name: String,
	pub address: String,
	pub username: String,
	pub password: String,
	pub color: Color32,
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