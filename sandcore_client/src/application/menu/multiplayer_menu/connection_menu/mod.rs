mod event;

use bevy::{prelude::*, window::WindowResized};
use bevy::app::{App, Plugin, Startup, Update};
use bevy_egui::{egui, EguiContexts, EguiSettings};
use crate::application::event::ConnectionEvent;
use crate::application::menu::state::MenuState;
use crate::application::state::ApplicationState;

pub struct ConnectionMenu;

impl Plugin for ConnectionMenu {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ServerName>()
            .init_resource::<ServerAddress>()
            .add_systems(OnEnter(ApplicationState::ConnectionMenu), on_enter)
            .add_systems(OnExit(MenuState::), systems)
            .add_event::<ConnectionEvent>()
            
            .add_systems(Startup, startup)
            .add_systems(Update, update_ui.run_if(in_state(ApplicationState::ConnectionMenu)))
            .add_systems(Update, update_ui_scale.run_if(in_state(ApplicationState::ConnectionMenu)));
    }
}

#[derive(Resource, Default)]
struct ServerName{
    data: String,
}

#[derive(Resource, Default)]
struct ServerAddress{
    data: String,
}

fn startup(
    mut contexts: EguiContexts
) {
    let ctx = contexts.ctx_mut();
    let mut current_style = (*ctx.style()).clone();
    //current_style.text_styles.insert(Heading, FontId::new(30.0, Proportional));


    for (text_style, mut font_id) in &mut current_style.text_styles {
        font_id.size *= 2.0;
    }

    ctx.set_style(current_style);
}

fn update_ui(
    mut next_state: ResMut<NextState<ApplicationState>>,
    mut connection_events: EventWriter<ConnectionEvent>,


    mut contexts: EguiContexts,
    mut server_name: ResMut<ServerName>,
    mut server_address: ResMut<ServerAddress>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Connect to server");
        });
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.columns(3, |columns| {
            columns[0].vertical_centered_justified(|ui| {
                if ui.button("Connect!").clicked() {
                    connection_events.send(ConnectionEvent::new(server_address.data.clone()));
                    next_state.set(ApplicationState::Gameplay);
                }
            });

            columns[1].vertical_centered_justified(|ui| {
                if ui.button("Save!").clicked() {

                }
            });

            columns[2].vertical_centered_justified(|ui| {
                if ui.button("Back!").clicked() {

                }
            });
        });
    });


    egui::CentralPanel::default().show(ctx, |ui| {
        ui.columns(3, |columns| {
            columns[1].vertical_centered_justified(|ui| {
                
                ui.label("server name");
                ui.text_edit_singleline(&mut server_name.data);
                ui.label("server address");
                ui.text_edit_singleline(&mut server_address.data);
            });
        });
    });

}

fn update_ui_scale(
    mut resize_reader: EventReader<WindowResized>,
    mut egui_settings: ResMut<EguiSettings>,
) {
}