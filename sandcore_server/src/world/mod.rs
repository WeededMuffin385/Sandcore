use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use euclid::default::Vector2D;
use creatures::Creatures;
use message::{Message, Request, Response};


pub mod creatures;
pub mod message;

pub struct World {
    pub creatures: Creatures,

    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
}

impl World {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let creatures = Creatures::new(sender.clone());

        Self{
            sender,
            receiver,
            creatures,
        }
    }

    pub fn run(mut self) {
        loop {
            self.update();
        }
    }
    pub fn get_sender(&self) -> mpsc::Sender<Message> {
        self.sender.clone()
    }

    pub fn update(&mut self) {
        self.creatures.update();
        self.update_receiver();
    }

    fn update_receiver(&mut self) {
        if let Ok(message) = self.receiver.try_recv() {
            match message.request {
                Request::Spawn => {
                    let sender_creature = self.creatures.spawn();
                    message.response(Response::Spawn(sender_creature));
                }

                Request::GetCreatures => {
                    let creatures = self.creatures.get_creatures();
                    message.response(Response::GetCreatures(creatures));
                }
            }
        }
    }
}