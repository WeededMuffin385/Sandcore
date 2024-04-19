pub mod event;
mod servers;

use egui::Rect;
use std::mem;
use bevy::app::{App, Startup, Update};
use bevy::asset::AssetContainer;
use bevy::prelude::{EventWriter, in_state, IntoSystemConfigs, NextState, Plugin, Res, ResMut, Resource, State};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align, CentralPanel, Color32, Frame, Layout, Margin, Rounding, Slider, Ui, Vec2};
use crate::application::menu::state::MenuState;
use crate::application::menu::multiplayer_menu::event::ConnectionEvent;
use crate::application::state::ApplicationState;
use servers::Servers;
use crate::application::menu::multiplayer_menu::servers::Server;


pub struct MultiplayerMenu;

impl Plugin for MultiplayerMenu {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Servers>()
            .init_resource::<Settings>()
            .add_event::<ConnectionEvent>()

            .add_systems(Update, (
                    update_ui_server_settings,
                    update_ui_server_list,
            ).chain().run_if(in_state(ApplicationState::Menu)).run_if(in_state(MenuState::MultiplayerMenu)));
    }
}

#[derive(Resource, Default)]
struct Settings {
    index: Option<usize>,
    server: Server,
}

fn update_ui_server_list(
    mut contexts: EguiContexts,
    mut connection_events: EventWriter<ConnectionEvent>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,

    mut servers: ResMut<Servers>,
    mut settings: ResMut<Settings>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            let mut garbage = Vec::new();

            for (index, server) in servers.servers.iter().enumerate() {
                Frame::group(ui.style()).fill(server.color).rounding(5.0).show(ui, |ui|{
                    ui.horizontal(|ui|{
                        let size = [ui.available_width() * 0.6, 20.0];

                        if ui.add_sized(size, egui::widgets::Button::new(format!("{} [{}]", &server.name, &server.address))).clicked() {
                            connection_events.send(ConnectionEvent::new(server.address.clone()));
                            next_application_state.set(ApplicationState::Gameplay);
                        }

                        let size = [ui.available_width(), 20.0];

                        if ui.add_sized(size, egui::widgets::Button::new("settings")).clicked() {
                            settings.index = Some(index);
                            settings.server = server.clone();
                        }
                    });
                    ui.add_space(40.0);
                });
            }


            for index in garbage {
                servers.servers.remove(index);
            }
        });
    });
}

fn update_ui_server_settings(
    mut contexts: EguiContexts,

    mut servers: ResMut<Servers>,
    mut settings: ResMut<Settings>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    let ctx = contexts.ctx_mut();
    let width = ctx.available_rect().width() * 0.2;

    egui::SidePanel::right("right_panel").exact_width(width).show(ctx, |ui| {
        let rect = Rect::from_min_size(ui.next_widget_position(), Vec2::new(width - 15.0, ui.available_height()));
        let mut ui = ui.child_ui(rect, *ui.layout());

        let mut save_changes = {
            if let Some(index) = settings.index {
                servers.servers[index] = settings.server.clone();
            }
        };

        ui.vertical_centered_justified(|ui|{
            ui.label("server name");
            if ui.text_edit_singleline(&mut settings.server.name).changed() {
                if let Some(index) = settings.index {
                    servers.servers[index] = settings.server.clone();
                }
            }

            ui.label("server address");
            if ui.text_edit_singleline(&mut settings.server.address).changed() {
                if let Some(index) = settings.index {
                    servers.servers[index] = settings.server.clone();
                }
            }

            ui.label("username");
            if ui.text_edit_singleline(&mut settings.server.username).changed() {
                if let Some(index) = settings.index {
                    servers.servers[index] = settings.server.clone();
                }
            }

            ui.label("password");
            if ui.add(egui::widgets::TextEdit::singleline(&mut settings.server.password).password(true)).changed() {
                if let Some(index) = settings.index {
                    servers.servers[index] = settings.server.clone();
                }
            }

            match settings.index {
                None => {
                    if ui.button("create").clicked() {
                        servers.servers.push(settings.server.clone());
                        settings.server = Default::default();
                    }
                }

                Some(index) => {
                    ui.columns(2, |columns|{
                        columns[0].vertical_centered_justified(|ui|{
                            if ui.button("delete").clicked() {
                                servers.servers.remove(index);
                                settings.server = Default::default();
                                settings.index = None;
                            }
                        });

                        columns[1].vertical_centered_justified(|ui|{
                           if ui.button("close").clicked() {
                               settings.server = Default::default();
                               settings.index = None;
                           }
                        });

                    });
                }
            }

            ui.color_edit_button_srgba(&mut settings.server.color);
        });


        ui.add_space(ui.available_height() - 20.0);

        ui.vertical_centered_justified(|ui|{
            if ui.button("Save'n'back").clicked() {
                next_menu_state.set(MenuState::MainMenu);
            }
        })
    });
}