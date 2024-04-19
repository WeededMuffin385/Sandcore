use std::net::TcpStream;
use std::sync::mpsc;
use sandcore_core::message::Message;
use crate::world::creatures::creature::message::{Message as CreatureMessage, Request as CreatureRequest, Response as CreatureResponse};
use crate::world::message::{Message as WorldMessage, Request as WorldRequest, Response as WorldResponse};


use sandcore_core::message_client::MessageClient;
use sandcore_core::message_server::MessageServer;

pub struct Client {
    sender_creature: Option<mpsc::Sender<CreatureMessage>>,
    sender_world: mpsc::Sender<WorldMessage>,

    connected: bool,
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream, sender_world: mpsc::Sender<WorldMessage>) -> Self {
        Self {
            sender_creature: None,
            sender_world,

            stream,
            connected: true,
        }
    }

    pub async fn run(mut self) {
        while self.connected {
            self.update().await;
        }
    }

    async fn update(&mut self) {
        if let Ok(message) = MessageClient::read(&mut self.stream) {
            self.update_message(message).await;
        } else {
            self.connected = false;
        }
    }

    async fn update_message(&mut self, message: MessageClient){
        match message {
            MessageClient::SetDirection(direction) => {
                if let Some(sender_creature) = &mut self.sender_creature {
                    CreatureMessage::request(sender_creature, CreatureRequest::SetDirection(direction)).unwrap();
                } else {
                    println!("creature doesn't exists");
                }
            }
            MessageClient::GetCreatures => {
                if let Ok(WorldResponse::GetCreatures(creatures)) = WorldMessage::request(&mut self.sender_world, WorldRequest::GetCreatures).unwrap().await {
                    MessageServer::Creatures(creatures).write(&mut self.stream).unwrap();
                }
            }
            MessageClient::Spawn => {
                if let Ok(WorldResponse::Spawn(sender_creature)) = WorldMessage::request(&mut self.sender_world, WorldRequest::Spawn).unwrap().await {
                    self.sender_creature = Some(sender_creature);
                } else {
                    self.sender_creature = None;
                }
            }
        }
    }
}