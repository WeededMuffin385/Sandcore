mod connection_state;

use std::sync::mpsc::Sender;
use tokio::io;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use crate::app::scenes::connection_menu::connection_state::ConnectionState;
use crate::app::scenes::gameplay::Gameplay;
use crate::app::scenes::multiplayer_menu::MultiplayerMenu;
use crate::app::scenes::scene::{Scene, SceneMessage};
use crate::server::Server;

pub struct ConnectionMenu {
	receiver: oneshot::Receiver<io::Result<Server>>,
	connection_state: ConnectionState,
}

impl ConnectionMenu {
	pub fn new(addr: String) -> Self {
		let (sender, receiver) = oneshot::channel();

		std::thread::spawn(move ||{
			sender.send(Server::new(addr)).unwrap();
		});

		Self {
			receiver,
			connection_state: Default::default(),
		}
	}
}

impl Scene for ConnectionMenu {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {
		if let Ok(server) = self.receiver.try_recv() {
			if let Ok(server) = server {
				sender.send(SceneMessage::ChangeScene(Box::new(Gameplay::new(server)))).unwrap();
				self.connection_state = ConnectionState::Success
			} else {
				self.connection_state = ConnectionState::Failure
			}
		}
	}
	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &egui::Context) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
			ui.vertical_centered(|ui|{
				ui.heading("Connection");
			});
		});

		egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui|{
			ui.vertical_centered(|ui|{
				if ui.button("cancel").clicked() {
					sender.send(SceneMessage::ChangeScene(Box::new(MultiplayerMenu::default()))).unwrap();
				}
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			let height = ui.available_height();

			ui.vertical_centered(|ui| {
				match self.connection_state {
					ConnectionState::Idle | ConnectionState::Process => {
						let spinner = egui::widgets::Spinner::new().size(height).color(egui::Color32::from_rgb(0,0,255));
						ui.add(spinner);
					}
					ConnectionState::Success => {
						ui.add_space(height / 2.0);
						ui.label("connection succeed");
					}
					ConnectionState::Failure => {
						ui.add_space(height / 2.0);
						ui.label("connection failed");
					}
				}
			});
		});
	}
}