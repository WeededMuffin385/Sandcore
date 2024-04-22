mod server;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use egui::{CentralPanel, Color32, Context, Frame, Rect, Ui, Vec2};
use crate::app::scenes::connection_menu::ConnectionMenu;
use crate::app::scenes::main_menu::MainMenu;
use crate::app::scenes::multiplayer_menu::server::Server;
use crate::app::scenes::scene::Scene;
use crate::app::scenes::Scenes;
use crate::app::scenes::state::State;

pub struct MultiplayerMenu{
	servers: Vec<Server>,
	selected: Option<usize>,
}

impl MultiplayerMenu {
	pub fn new() -> Self {
		Self{
			servers: Default::default(),
			selected: Default::default(),
		}
	}
}

impl Scene for  MultiplayerMenu {
	fn update(&mut self, state: &mut State) {

	}
	fn update_ui(&mut self, state: &mut State, ctx: &Context) {
		update_side_panel(ctx, state, &mut self.servers, &mut self.selected);
		update_central_panel(ctx, state, &mut self.servers, &mut self.selected);
	}
}

fn update_central_panel(ctx: &egui::Context, state: &mut State, servers: &mut Vec<Server>, selected: &mut Option<usize>) {
	CentralPanel::default().show(ctx, |ui|{
		egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
			for (index, server) in servers.iter().enumerate() {
				Frame::group(ui.style()).fill(server.color).rounding(5.0).show(ui, |ui| {
					ui.horizontal(|ui| {
						let size = [ui.available_width() * 0.6, 20.0];
						if ui.add_sized(size, egui::widgets::Button::new(format!("{} [{}]", &server.name, &server.address))).clicked() {
							state.next_scene = Some(Box::new(ConnectionMenu::new(server.address.clone())))
						}

						let size = [ui.available_width(), 20.0];
						if ui.add_sized(size, egui::widgets::Button::new("settings")).clicked() {
							*selected = Some(index);
						}
					});
					ui.add_space(40.0);
				});
			}
		});
	});
}

fn update_side_panel(ctx: &egui::Context, state: &mut State, servers: &mut Vec<Server>, selected: &mut Option<usize>) {
	let width = ctx.available_rect().width() * 0.2;

	egui::SidePanel::right("right_panel").exact_width(width).show(ctx, |ui| {
		let rect = Rect::from_min_size(ui.next_widget_position(), Vec2::new(width - 15.0, ui.available_height()));
		let mut ui = ui.child_ui(rect, *ui.layout());

		ui.vertical_centered_justified(|ui|{
			if let Some(current) = selected.clone() {
				let mut close = false;
				let mut delete = false;

				update_selected_settings(ui, &mut servers[current], &mut close, &mut delete);

				if close {*selected = None}
				if delete {
					*selected = None;
					servers.remove(current);
				}
			} else {
				if ui.button("create").clicked() {
					servers.push(Default::default());
					*selected = Some(servers.len() - 1);
				}
			}
		});


		ui.add_space(ui.available_height() - 20.0);

		ui.vertical_centered_justified(|ui|{
			if ui.button("Save'n'back").clicked() {
				state.next_scene = Some(Box::new(MainMenu::new()));
			}
		})
	});
}

fn update_selected_settings(ui: &mut Ui, server: &mut Server, close: &mut bool, delete: &mut bool) {
	ui.label("server name");
	ui.text_edit_singleline(&mut server.name);



	ui.label("server address");
	ui.text_edit_singleline(&mut server.address);


	ui.label("username");
	ui.text_edit_singleline(&mut server.username);



	ui.label("password");
	ui.add(egui::widgets::TextEdit::singleline(&mut server.password).password(true));


	ui.columns(2, |columns|{
		columns[0].vertical_centered_justified(|ui|{
			if ui.button("delete").clicked() {
				*delete = true;
			}
		});

		columns[1].vertical_centered_justified(|ui|{
			if ui.button("close").clicked() {
				*close = true;
			}
		});
	});

	ui.color_edit_button_srgba(&mut server.color);
}