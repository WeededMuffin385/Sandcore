use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Servers {
    pub servers: Vec<Server>
}

#[derive(Default, Clone)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub username: String,
    pub password: String,
}

impl Server {
    pub fn new(name: String, address: String, username: String, password: String) -> Self {
        Self {
            name,
            address,
            username,
            password,
        }
    }
}


impl Default for Servers {
    fn default() -> Self {
        let mut servers = Vec::new();

        servers.push(Server::new("localhost".to_string(), "127.0.0.1:4000".to_string(), "user".to_string(), "pass".to_string()));

        Self {
            servers
        }
    }
}