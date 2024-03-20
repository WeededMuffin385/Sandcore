use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameplayState {
    #[default]
    None,
    Game,
    Connection,
}