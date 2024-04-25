mod server;

use std::fs::{create_dir_all, File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use egui::{Button, CentralPanel, Color32, Context, Frame, Rect, Ui, Vec2};
use egui::epaint::tessellator::Path;
use crate::app::scenes::connection_menu::ConnectionMenu;
use crate::app::scenes::main_menu::MainMenu;
use crate::app::scenes::multiplayer_menu::server::Server;
use crate::app::scenes::scene::{Scene, SceneMessage};

pub struct MultiplayerMenu{
	servers: Vec<Server>,
	selected: Option<usize>,

	last_button_height: f32,
}

impl Scene for  MultiplayerMenu {
	fn update(&mut self, sender: &mut Sender<SceneMessage>) {

	}
	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &Context) {
		update_side_panel(ctx, sender, &mut self.servers, &mut self.selected, &mut self.last_button_height);
		update_central_panel(ctx, sender, &mut self.servers, &mut self.selected);
	}
}

fn update_central_panel(ctx: &Context, sender: &mut Sender<SceneMessage>, servers: &mut Vec<Server>, selected: &mut Option<usize>) {
	CentralPanel::default().show(ctx, |ui|{
		egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
			for (index, server) in servers.iter().enumerate() {
				Frame::group(ui.style()).fill(server.color).rounding(5.0).show(ui, |ui| {
					ui.horizontal(|ui| {
						let size = [ui.available_width() * 0.6, 20.0];
						if ui.add_sized(size, egui::widgets::Button::new(format!("{} [{}]", &server.name, &server.address))).clicked() {
							sender.send(SceneMessage::ChangeScene(Box::new(ConnectionMenu::new(server.address.clone())))).unwrap();
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

fn update_side_panel(ctx: &Context, sender: &mut Sender<SceneMessage>, servers: &mut Vec<Server>, selected: &mut Option<usize>, last_button_height: &mut f32) {
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

		ui.add_space(ui.available_height() - *last_button_height);

		ui.vertical_centered_justified(|ui|{
			let begin = ui.next_widget_position();
			if ui.button("Save'n'back").clicked() {
				sender.send(SceneMessage::ChangeScene(Box::new(MainMenu::new()))).unwrap();
			}
			*last_button_height = (ui.next_widget_position() - begin).y;
		});


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

impl Drop for MultiplayerMenu {
	fn drop(&mut self) {
		if !PathBuf::from("assets/").exists() { create_dir_all("assets/").unwrap(); }

		let data = serde_json::to_string(&self.servers).unwrap();
		File::create("assets/servers.json").unwrap().write_all(data.as_ref()).unwrap();
	}
}

impl Default for MultiplayerMenu {
	fn default() -> Self {
		let servers = if let Ok(data) = read_to_string("assets/servers.json") {
			serde_json::from_str(&data).unwrap()
		} else {
			Default::default()
		};

		Self{
			servers,
			selected: Default::default(),
			last_button_height: Default::default(),
		}
	}
}