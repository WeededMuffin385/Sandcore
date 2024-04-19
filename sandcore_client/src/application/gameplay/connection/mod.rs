mod state;

use std::io;
use std::sync::mpsc::{channel, Receiver};
use bevy::app::{App, Update};
use bevy::prelude::{Commands, Entity, EventReader, in_state, IntoSystemConfigs, NextState, OnEnter, OnExit, Plugin, Res, ResMut, Resource, State};
use bevy_egui::{egui, EguiContexts};
use tokio::sync::oneshot;
use crate::application::gameplay::connection::state::ConnectionState;
use crate::application::gameplay::server::ServerResource;
use crate::application::gameplay::state::GameplayState;
use crate::application::menu::multiplayer_menu::event::ConnectionEvent;
use crate::application::menu::state::MenuState;
use crate::application::state::ApplicationState;
use crate::server::Server;

pub struct Connection;

impl Plugin for Connection {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ConnectionState>()
            .init_resource::<Runtime>()
            .add_systems(OnEnter(GameplayState::Connection), on_enter)
            .add_systems(OnExit(GameplayState::Connection), on_exit)
            .add_systems(Update, (
                update_ui,
                update_receiver.run_if(in_state(ConnectionState::Process)),
                update_connection.run_if(in_state(ConnectionState::Idle)),
            ).run_if(in_state(ApplicationState::Gameplay)).run_if(in_state(GameplayState::Connection)));
    }
}

#[derive(Resource)]
struct Runtime {
    runtime: tokio::runtime::Runtime,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

#[derive(Resource)]
struct ConnectionReceiver {
    receiver: oneshot::Receiver<io::Result<Server>>,
}

fn on_enter() {

}

fn on_exit(
    mut commands: Commands,
    mut next_state: ResMut<NextState<ConnectionState>>,
) {
    next_state.set(ConnectionState::Idle);
    commands.remove_resource::<ConnectionReceiver>();
}

fn update_ui(
    mut contexts: EguiContexts,
    state: Res<State<ConnectionState>>,

    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            ui.heading("Connection");
        });
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            if ui.button("cancel").clicked() {
                next_menu_state.set(MenuState::MultiplayerMenu);
                next_application_state.set(ApplicationState::Menu);
            }
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        let height = ui.available_height();

        ui.vertical_centered(|ui| {
            match *state.get() {
                ConnectionState::Idle | ConnectionState::Process => {
                    let spinner = egui::widgets::Spinner::new().size(height).color(egui::Color32::from_rgb(0,0,255));
                    ui.add(spinner);
                }
                ConnectionState::Success => {
                    ui.add_space(height / 2.0);
                    ui.label("connection succeed");
                }
                ConnectionState::Failure => {
                    ui.add_space(height / 2.0);
                    ui.label("connection failed");
                }
            }
        });
    });
}

fn update_connection(
    mut commands: Commands,
    mut runtime: ResMut<Runtime>,
    mut connection_events: EventReader<ConnectionEvent>,

    state: Res<State<ConnectionState>>,
    mut next_state: ResMut<NextState<ConnectionState>>,
) {
    if let Some(address) = connection_events.read().last() {
        let (sender, receiver) = oneshot::channel();
        let address = address.address.clone();

        runtime.runtime.spawn(async move {
            let client = Server::new(address);
            sender.send(client);
        });

        commands.insert_resource(ConnectionReceiver{receiver});
        next_state.set(ConnectionState::Process);
    }
}

fn update_receiver(
    mut commands: Commands,
    mut next_state: ResMut<NextState<ConnectionState>>,
    mut connection_receiver: ResMut<ConnectionReceiver>,
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
) {
    if let Ok(server) = connection_receiver.receiver.try_recv() {
        if let Ok(server) = server {
            commands.insert_resource(ServerResource::new(server));
            next_gameplay_state.set(GameplayState::Game);
        } else {
            next_state.set(ConnectionState::Failure);
        }
    }
}