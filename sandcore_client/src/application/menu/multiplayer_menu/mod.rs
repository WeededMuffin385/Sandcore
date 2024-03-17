mod state;
mod event;

use bevy::app::{App, Startup, Update};
use bevy::prelude::{EventWriter, in_state, IntoSystemConfigs, NextState, Plugin, Res, ResMut, Resource, State};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Ui;
use crate::application::menu::state::MenuState;
use state::MultiplayerMenuState;
use crate::application::menu::multiplayer_menu::event::ConnectionEvent;


pub struct MultiplayerMenu;

impl Plugin for MultiplayerMenu {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MultiplayerMenuState>()
            .init_resource::<Input>()
            .add_event::<ConnectionEvent>()
            .add_systems(Update, update_ui.run_if(in_state(MenuState::MultiplayerMenu)))

        ;
    }
}

#[derive(Resource, Default)]
struct Input {
    server_name: String,
    server_address: String,
}

fn update_ui(
    mut contexts: EguiContexts,
    mut next_menu_state: ResMut<NextState<MenuState>>,

    current_state: Res<State<MultiplayerMenuState>>,
    mut next_state: ResMut<NextState<MultiplayerMenuState>>,

    mut input: ResMut<Input>,

    mut connection_events: EventWriter<ConnectionEvent>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel").exact_width(ctx.available_rect().width() * 0.2).show(ctx, |ui|{
        let button_size = [ui.available_width(), 20.0];

        if ui.add_sized(button_size, egui::Button::new("Server List")).clicked() {
            next_state.set(MultiplayerMenuState::ServerList);
        }

        if ui.add_sized(button_size, egui::Button::new("Add New Server")).clicked() {
            next_state.set(MultiplayerMenuState::AddNewServer);
        }

        ui.add_space(ui.available_height() - button_size[1]);

        if ui.add_sized(button_size, egui::Button::new("Save'n'back")).clicked() {
            next_menu_state.set(MenuState::MainMenu);
        }
    });

    egui::CentralPanel::default().show(ctx, |ui|{
       match current_state.get() {
           MultiplayerMenuState::ServerList => {show_server_list(ui); }
           MultiplayerMenuState::AddNewServer => {show_add_new_server(ui, &mut input)}
       }
    });
}

fn show_server_list(
    ui: &mut Ui,
) {
    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui|{
        ui.vertical(|ui|{
            let width = ui.available_width();
            let server_button_size = [width * 0.5, 20.0];
            let setting_button_size = [width * 0.2, 20.0];

            for i in 1..=50 {
                ui.horizontal(|ui|{
                    if ui.add_sized(server_button_size, egui::widgets::Button::new(format!("server name {i}"))).clicked() {
                        // connection_events.send()
                    }

                    ui.add_sized(setting_button_size, egui::widgets::Button::new(format!("settings")));
                    ui.add_sized(setting_button_size, egui::widgets::Button::new(format!("delete")));
                });
            }
        });
    });
}

fn show_add_new_server(
    ui: &mut Ui,
    input: &mut ResMut<Input>,
) {
    let width = ui.available_width();
    let form_size = [width, 20.0];

    ui.vertical_centered(|ui|{
        ui.label("server name");
        ui.add_sized(form_size, egui::widgets::TextEdit::singleline(&mut input.server_name));

        ui.label("server address");
        ui.add_sized(form_size, egui::widgets::TextEdit::singleline(&mut input.server_address));
    });
}