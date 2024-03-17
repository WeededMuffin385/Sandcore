use bevy::prelude::Event;

#[derive(Event)]
pub struct ConnectionEvent {
    pub address: String,
}

impl ConnectionEvent {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    } 
}