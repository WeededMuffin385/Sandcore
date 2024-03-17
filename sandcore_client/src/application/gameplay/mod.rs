mod renderer;
mod client;

use bevy::app::{App, Update};
use bevy::prelude::{Commands, Component, EventReader, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Query};
use bevy_egui::{egui, EguiContexts};
// use crate::application::event::ConnectionEvent;
use crate::application::state::ApplicationState;

pub struct Gameplay;

impl Plugin for Gameplay {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ApplicationState::Gameplay), on_enter)
            .add_systems(OnExit(ApplicationState::Gameplay), on_exit)
            .add_systems(Update, update.run_if(in_state(ApplicationState::Gameplay)))
        ;
    }
}

fn on_enter(
    mut commands: Commands,
    // mut connection_events: EventReader<ConnectionEvent>,

    mut client_query: Query<&mut ClientComponent>
) {
    // let address = connection_events.read().last().expect("connection address wasn't sent");


    ;
}

fn on_exit() {

}

fn update(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.label("text");
    });
}

#[derive(Component)]
struct ClientComponent {

}