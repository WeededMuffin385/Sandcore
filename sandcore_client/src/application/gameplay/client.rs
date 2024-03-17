use std::io;
use std::net::ToSocketAddrs;
use bevy::prelude::Component;
use crate::client::Client;

#[derive(Component)]
pub struct ClientComponent {
    pub client: Client,
}

impl ClientComponent {
    pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let client = Client::new(addr)?;

        Ok(Self { client })
    }
}