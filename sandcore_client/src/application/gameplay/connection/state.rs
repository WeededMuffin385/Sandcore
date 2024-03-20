use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ConnectionState {
    #[default]
    Waiting,
    Progress,
    Success,
    Failure,
}