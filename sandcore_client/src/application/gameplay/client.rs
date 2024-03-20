use bevy::prelude::Resource;
use crate::client::Client;

#[derive(Resource)]
pub struct GameplayClient {
    pub client: Client,
}

impl GameplayClient {
    pub fn new(client: Client) -> Self {
        Self {
            client,
        }
    }
}