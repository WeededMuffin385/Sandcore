mod renderer;

use std::collections::HashMap;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Entity, in_state, IntoSystemConfigs, OnEnter, OnExit, ResMut, Resource};
use bevy_egui::{egui, EguiContexts};
use crate::application::gameplay::state::GameplayState;
use crate::application::state::ApplicationState;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameplayState::Game), on_enter)
            .add_systems(OnExit(GameplayState::Game), on_exit)
            .add_systems(Update, update_ui
                .run_if(in_state(ApplicationState::Gameplay))
                .run_if(in_state(GameplayState::Game))
            )

        ;
    }
}

fn on_enter() {

}

fn on_exit(
    mut commands: Commands,
    mut creatures: ResMut<Creatures>,
) {
    for (id, creature) in &mut creatures.creatures {
        commands.entity(*creature).despawn();
    }
}

#[derive(Resource)]
struct Creatures {
    creatures: HashMap<usize, Entity>,
}

fn update_ui(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel").exact_width(ctx.available_rect().width() * 0.2).show(ctx, |ui|{
        ui.label("some text");
    });
}

fn update_renderer(

) {

}