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
use bevy_egui::egui::{CentralPanel, Color32, Frame, Margin, Rounding, Vec2};
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
            ui.vertical(|ui| {

                let width = ui.available_width();
                let size = [width * 0.6, 20.0];
                let size_compact = [width * 0.2, 20.0];


                let mut garbage = Vec::new();

                let frame = egui::Frame{
                    fill: egui::Color32::from_rgb(0,255,0),
                    inner_margin: Margin::same(width * 0.05),
                    rounding: Rounding::same(15.0),
                    .. Default::default()
                };

                for (index, server) in servers.servers.iter().enumerate() {
                    //frame.show(ui, |ui|{
                        ui.horizontal(|ui|{
                            if ui.add_sized(size, egui::widgets::Button::new(format!("{} [{}]", &server.name, &server.address))).clicked() {
                                connection_events.send(ConnectionEvent::new(server.address.clone()));
                                next_application_state.set(ApplicationState::Gameplay);
                            }

                            if ui.add_sized(size_compact, egui::widgets::Button::new("settings")).clicked() {
                                settings.index = Some(index);
                                settings.server = server.clone();
                            }

                            if ui.add_sized(size_compact, egui::widgets::Button::new("delete")).clicked() {
                                garbage.push(index);
                            }
                        });
                    //});
                }

                for index in garbage {
                    servers.servers.remove(index);
                }
            });
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

    egui::SidePanel::right("right_panel").exact_width(ctx.screen_rect().width() * 0.2).show(ctx, |ui| {
        let size = [ui.available_width() * 0.95, 20.0];

        ui.vertical_centered(|ui|{
            ui.label("server name");
        });
        ui.vertical(|ui| {
            ui.add_sized(size, egui::widgets::TextEdit::singleline(&mut settings.server.name));
        });

        ui.vertical_centered(|ui| {
            ui.label("server address");
        });
        ui.vertical(|ui|{
            ui.add_sized(size, egui::widgets::TextEdit::singleline(&mut settings.server.address));
        });

        ui.vertical_centered(|ui| {
            ui.label("username");
        });
        ui.vertical(|ui|{
            ui.add_sized(size, egui::widgets::TextEdit::singleline(&mut settings.server.username));
        });

        ui.vertical_centered(|ui| {
            ui.label("password");
        });
        ui.vertical(|ui|{
            ui.add_sized(size, egui::widgets::TextEdit::singleline(&mut settings.server.password).password(true));
        });

        ui.vertical(|ui|{
            if ui.add_sized(size, egui::Button::new("save server")).clicked() {
                if let Some(index) = settings.index {
                    servers.servers[index] = settings.server.clone();
                } else {
                    let mut server = Default::default();
                    mem::swap(&mut server, &mut settings.server);
                    servers.servers.push(server);
                }

            }

            if ui.add_sized(size, egui::Button::new("clean")).clicked() {
                settings.server = Default::default();
                settings.index = None;
            }
        });

        ui.vertical(|ui|{
            ui.add_space(ui.available_height() - size[1]);

            if ui.add_sized(size, egui::Button::new("Save'n'back")).clicked() {
                next_menu_state.set(MenuState::MainMenu);
            }
        });





    });
}