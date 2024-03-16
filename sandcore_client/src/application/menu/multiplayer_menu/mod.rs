mod connection_menu;

use bevy::app::{App, Startup, Update};
use bevy::prelude::Plugin;

pub struct MultiplayerMenu;

impl Plugin for MultiplayerMenu {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update)



        ;
    }
}

fn startup() {

}

fn update() {

}
