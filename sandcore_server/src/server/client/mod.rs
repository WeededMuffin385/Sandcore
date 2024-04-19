use std::net::TcpStream;
use std::sync::mpsc;

use sandcore_world::world::creature::message::Message as CreatureMessage;
use sandcore_world::world::message::Message as WorldMessage;

pub struct Client {
    sender_creature: Option<mpsc::Sender<CreatureMessage>>,
    sender_world: mpsc::Sender<WorldMessage>,
}

impl Client {
    pub fn new(sender_world: mpsc::Sender<WorldMessage>) -> Self {
        Self {
            sender_creature: None,
            sender_world,
        }
    }
}