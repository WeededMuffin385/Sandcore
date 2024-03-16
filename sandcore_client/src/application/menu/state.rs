use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    None,
    #[default]
    MainMenu,
    SettingsMenu,
    MultiplayerMenu,
}