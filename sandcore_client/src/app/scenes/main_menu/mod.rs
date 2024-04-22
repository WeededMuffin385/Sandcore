use std::sync::mpsc;
use std::sync::mpsc::Sender;
use egui::Context;
use macroquad::miniquad::window::{order_quit, quit};
use crate::app::scenes::multiplayer_menu::MultiplayerMenu;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::Scenes;
use crate::app::scenes::settings_menu::SettingsMenu;
use crate::app::scenes::state::State;

pub struct MainMenu{
}

impl MainMenu {
	pub fn new() -> Self {
		Self {
		}
	}
}

impl Scene for MainMenu {
	fn update(&mut self, state: &mut State) {

	}
	
	fn update_ui(&mut self, state: &mut State, ctx: &Context) {
		update_panels(ctx, state);
	}
}

fn update_panels(ctx: &Context, state: &mut State) {
	egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
		ui.vertical_centered(|ui|{
			ui.heading("Main Menu");
		});
	});

	egui::CentralPanel::default().show(ctx, |ui|{
		let button_size = [ui.available_width() * 0.35, 20.0];

		ui.vertical_centered(|ui|{
			if ui.add_sized(button_size, egui::widgets::Button::new("Multiplayer")).clicked() {
				state.next_scene = Some(Box::new(MultiplayerMenu::new()));
			}

			if ui.add_sized(button_size, egui::widgets::Button::new("Settings")).clicked() {
				state.next_scene = Some(Box::new(SettingsMenu::new()));
			}

			if ui.add_sized(button_size, egui::widgets::Button::new("Exit")).clicked() {
				order_quit();
			}
		});
	});
}