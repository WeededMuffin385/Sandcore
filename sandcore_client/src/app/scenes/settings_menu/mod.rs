mod tab;
mod settings;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use egui::{Context, Ui};
use crate::app::scenes::main_menu::MainMenu;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::settings_menu::settings::Settings;
use crate::app::scenes::settings_menu::tab::Tab;
use crate::app::scenes::state::State;

pub struct SettingsMenu{
	tab: Tab,
	settings: Settings,
}

impl SettingsMenu {
	pub fn new() -> Self {
		Self {
			tab: Default::default(),
			settings: Default::default(),
		}
	}
}

impl Scene for SettingsMenu {
	fn update(&mut self, state: &mut State) {

	}

	fn update_ui(&mut self, state: &mut State, ctx: &Context) {
		update_side_panel(ctx, state, &mut self.tab);
		update_central_panel(ctx, &self.tab, &mut self.settings);
	}
}


fn update_central_panel(ctx: &Context, tab: &Tab, settings: &mut Settings) {
	egui::CentralPanel::default().show(ctx, |ui| {
		match *tab {
			Tab::General => {update_general_settings(ui, settings)}
			Tab::Interface => {ui.label("interface settings");}
			Tab::Languages => {ui.label("languages settings");}
		}
	});
}

fn update_side_panel(ctx: &Context, state: &mut State, tab: &mut Tab) {
	egui::SidePanel::left("left_panel").exact_width(ctx.available_rect().width() * 0.2).show(ctx, |ui|{
		let button_size = [ui.available_width(), 20.0];

		if ui.add_sized(button_size, egui::Button::new("General")).clicked() {
			*tab = Tab::General;
		}

		if ui.add_sized(button_size, egui::Button::new("Interface")).clicked() {
			*tab = Tab::Interface;
		}

		if ui.add_sized(button_size, egui::Button::new("Languages")).clicked() {
			*tab = Tab::Languages;
		}

		ui.add_space(ui.available_height() - button_size[1]);

		if ui.add_sized(button_size, egui::Button::new("Save'n'back")).clicked() {
			state.next_scene = Some(Box::new(MainMenu::new()));
		}
	});
}

fn update_general_settings(ui: &mut Ui, settings: &mut Settings) {
	ui.style_mut().spacing.slider_width = ui.available_width() * 0.5;

	ui.horizontal(|ui| {
		let slider = egui::widgets::Slider::new(&mut settings.font_size, 0.0..=60.0).text("font size");
		ui.add(slider);
	});
}