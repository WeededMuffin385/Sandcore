use bevy::prelude::Resource;
use bevy_egui::egui::Color32;

#[derive(Resource)]
pub struct Servers {
    pub servers: Vec<Server>
}

#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub color: Color32,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            name: Default::default(),
            address: Default::default(),
            username: Default::default(),
            password: Default::default(),
            color: Color32::LIGHT_GREEN,
        }
    }
}

impl Server {
    pub fn new(name: String, address: String, username: String, password: String, color: Color32) -> Self {
        Self {
            name,
            address,
            username,
            password,
            color,
        }
    }
}


impl Default for Servers {
    fn default() -> Self {
        let mut servers = Vec::new();

        servers.push(Server::new("localhost".to_string(), "127.0.0.1:4000".to_string(), "user".to_string(), "pass".to_string(), Color32::LIGHT_BLUE));

        Self {
            servers
        }
    }
}