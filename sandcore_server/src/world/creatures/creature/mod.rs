pub mod message;

use std::sync::mpsc::{channel, Sender, Receiver};
use euclid::default::Vector2D;
use message::Message;

pub struct Creature{
    sender: Sender<Message>,
    receiver: Receiver<Message>,

    position: Vector2D<f32>,
    direction: Vector2D<f32>,
}

impl Creature {
    pub fn new() -> Self {
        let (sender, receiver) = channel::<Message>();

        Self {
            sender,
            receiver,

            position: Default::default(),
            direction: Default::default(),
        }
    }

    pub fn update(&mut self) {

    }

    pub fn update_receiver(&mut self) {
        for message in &self.receiver {
            match message {
                Message::SetDirection(direction) => {
                    self.direction = direction.normalize();
                }
            }
        }
    }

    pub fn get_sender(&self) -> Sender<Message> {
        self.sender.clone()
    }
}