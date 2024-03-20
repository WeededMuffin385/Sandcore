use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    None,
    MainMenu,
    SettingsMenu,
    MultiplayerMenu,
}