use std::sync::mpsc::Sender;
use egui::Context;
use macroquad::miniquad::window::order_quit;
use crate::app::scenes::multiplayer_menu::MultiplayerMenu;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::settings_menu::SettingsMenu;
use crate::app::scenes::message::Message as SceneMessage;

pub struct MainMenu{
}

impl MainMenu {
	pub fn new() -> Self {
		Self {
		}
	}
}

impl Scene for MainMenu {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {

	}
	
	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &Context) {
		update_panels(ctx, sender);
	}
}

fn update_panels(ctx: &Context, sender: &mut Sender<SceneMessage>) {
	egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
		ui.vertical_centered(|ui|{
			ui.heading("Main Menu");
		});
	});

	egui::CentralPanel::default().show(ctx, |ui|{
		let button_size = [ui.available_width() * 0.35, 20.0];

		ui.vertical_centered(|ui|{
			if ui.add_sized(button_size, egui::widgets::Button::new("Multiplayer")).clicked() {
				sender.send(SceneMessage::ChangeScene(Box::new(MultiplayerMenu::default()))).unwrap();
			}

			if ui.add_sized(button_size, egui::widgets::Button::new("Settings")).clicked() {
				sender.send(SceneMessage::ChangeScene(Box::new(SettingsMenu::new()))).unwrap();
			}

			if ui.add_sized(button_size, egui::widgets::Button::new("Exit")).clicked() {
				order_quit();
			}
		});
	});
}