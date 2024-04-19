mod state;
mod connection;
mod game;
mod server;

use bevy::app::{App, Update};
use bevy::prelude::{Color, Commands, Component, EventReader, in_state, IntoSystemConfigs, NextState, OnEnter, OnExit, Plugin, Query, ResMut};
use bevy_egui::{egui, EguiContexts};
use crate::application::gameplay::connection::Connection;
use crate::application::gameplay::game::Game;
use crate::application::gameplay::server::ServerResource;
use crate::application::gameplay::state::GameplayState;
use crate::application::state::ApplicationState;
use crate::application::menu::multiplayer_menu::event::ConnectionEvent;

pub struct Gameplay;

impl Plugin for Gameplay {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameplayState>()
            .add_systems(OnEnter(ApplicationState::Gameplay), on_enter)
            .add_systems(OnExit(ApplicationState::Gameplay), on_exit)
            .add_plugins(Connection)
            .add_plugins(Game)
        ;
    }
}

fn on_enter(
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    next_state.set(GameplayState::Connection);
}

fn on_exit(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    next_state.set(GameplayState::None);
    commands.remove_resource::<ServerResource>();
}

