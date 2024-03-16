mod state;
mod main_menu;
mod settings_menu;

use bevy::prelude::*;
use bevy::app::{App, Startup, Update};
use bevy::ecs::schedule::{OnEnter, OnExit};
use crate::application::menu::state::MenuState;
use super::state::ApplicationState;


use settings_menu::SettingsMenu;
use main_menu::MainMenu;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()
            .add_systems(OnEnter(ApplicationState::Menu), on_enter)
            .add_systems(OnExit(ApplicationState::Menu), on_exit)

            .add_plugins(MainMenu)
            .add_plugins(SettingsMenu)
        ;
    }
}

fn on_enter(
    mut next_state: ResMut<NextState<MenuState>>,
) {
    next_state.set(MenuState::MainMenu);
}

fn on_exit(
    mut next_state: ResMut<NextState<MenuState>>,
) {
    next_state.set(MenuState::None);
}