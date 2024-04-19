pub mod gameplay;
pub mod menu;
pub mod state;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, Res};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::application::gameplay::Gameplay;
use crate::application::menu::Menu;
use crate::application::state::ApplicationState;

pub struct Application;

impl Plugin for Application {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin)

            .add_systems(Startup, startup)
            .add_systems(Update, update)

            .add_plugins(Gameplay)
            .add_plugins(Menu)

            .init_state::<ApplicationState>();
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

}

fn update() {

}
