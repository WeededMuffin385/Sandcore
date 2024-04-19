use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ConnectionState {
    #[default]
    Idle,
    Process,
    Success,
    Failure,
}