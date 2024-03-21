pub mod event;
mod servers;

use egui::Rect;
use egui::Pos2;
use std::collections::LinkedList;
use std::mem;
use bevy::app::{App, Startup, Update};
use bevy::asset::AssetContainer;
use bevy::prelude::{EventWriter, in_state, IntoSystemConfigs, NextState, Plugin, Res, ResMut, Resource, State};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align, CentralPanel, Color32, Frame, Layout, Margin, Rounding, Vec2};
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
            let width = ui.available_width();
            let size = [width * 0.6, 20.0];
            let mut garbage = Vec::new();

            for (index, server) in servers.servers.iter().enumerate() {
                Frame::group(ui.style()).fill(server.color).rounding(5.0).show(ui, |ui|{
                    ui.horizontal(|ui|{
                        if ui.add_sized(size, egui::widgets::Button::new(format!("{} [{}]", &server.name, &server.address))).clicked() {
                            connection_events.send(ConnectionEvent::new(server.address.clone()));
                            next_application_state.set(ApplicationState::Gameplay);
                        }

                        if ui.button("settings").clicked() {
                            settings.index = Some(index);
                            settings.server = server.clone();
                        }

                        if ui.button("delete").clicked() {
                            garbage.push(index);
                        }

                        ui.add_space(ui.available_width());
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

        ui.vertical_centered_justified(|ui|{
            ui.label("server name");
            ui.text_edit_singleline(&mut settings.server.name);

            ui.label("server address");
            ui.text_edit_singleline(&mut settings.server.address);

            ui.label("username");
            ui.text_edit_singleline(&mut settings.server.username);

            ui.label("password");
            ui.add(egui::widgets::TextEdit::singleline(&mut settings.server.password).password(true));

            ui.color_edit_button_srgba(&mut settings.server.color);
        });

        ui.columns(2, |columns|{
            columns[0].vertical_centered_justified(|ui|{
                if ui.button("save server").clicked() {
                    if let Some(index) = settings.index {
                        servers.servers[index] = settings.server.clone();
                    } else {
                        let mut server = Default::default();
                        mem::swap(&mut server, &mut settings.server);
                        servers.servers.push(server);
                    }
                }
            });

            columns[1].vertical_centered_justified(|ui|{
                if ui.button("clean").clicked() {
                    settings.server = Default::default();
                    settings.index = None;
                }
            });
        });

        ui.add_space(ui.available_height() - 20.0);

        ui.vertical_centered_justified(|ui|{
            if ui.button("Save'n'back").clicked() {
                next_menu_state.set(MenuState::MainMenu);
            }
        })
    });
}