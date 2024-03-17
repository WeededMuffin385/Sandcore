pub mod state;
mod gameplay;
mod menu;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, Res};
use bevy_egui::EguiPlugin;
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

            .init_state::<ApplicationState>()


        ;
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let color = Color::rgb(0.4, 0.7, 0.9);

    commands.insert_resource(ClearColor(color));
    commands.spawn(Camera2dBundle::default());
}

fn update() {

}