use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SettingsMenuState{
    #[default]
    General,
    Interface,
    Languages,
}