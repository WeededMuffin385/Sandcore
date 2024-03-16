use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use crate::application::{Application};

pub mod client;
mod world;
mod application;

fn main() {
    App::new()
        .add_plugins(Application)
        .run();
}