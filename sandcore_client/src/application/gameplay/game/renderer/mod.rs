mod creature;

use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, OnEnter, Plugin, Res};
use crate::application::gameplay::game::renderer::creature::CreatureBundle;
use crate::application::gameplay::state::GameplayState;

pub struct Renderer;

impl Plugin for Renderer {
    fn build(&self, app: &mut App) {
        app
         .add_systems(OnEnter(GameplayState::Game), on_enter);
    }
}

fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 1.0)));
    commands.spawn(Camera2dBundle::default());
    commands.spawn(CreatureBundle::new(1, &asset_server));
}