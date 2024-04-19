use bevy::prelude::{Component, Entity, Resource};

use crate::server::Server;

#[derive(Resource)]
pub struct ServerResource {
    pub server: Server,
}

impl ServerResource {
    pub fn new(server: Server) -> Self {
        Self {
            server,
        }
    }
}