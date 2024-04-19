use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use crate::application::{Application};

pub mod application;
pub mod server;

fn main() {
    App::new()
        .add_plugins(Application)
        .run();
}