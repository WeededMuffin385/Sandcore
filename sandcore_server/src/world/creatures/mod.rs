pub(crate) mod creature;
mod intelligence;

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, mpsc};
use euclid::default::Point2D;
use creature::Creature;
use crate::world::creatures::creature::message::Message as CreatureMessage;
use crate::world::message::Message as WorldMessage;
use crate::world::World;

pub struct Creatures {
    sender_world: mpsc::Sender<WorldMessage>,
    creatures: Vec<Creature>,
}

impl Creatures {
    pub fn new(sender_world: mpsc::Sender<WorldMessage>) -> Self {
        Self {
            creatures: Default::default(),
            sender_world,
        }
    }

    pub fn update(&mut self) {
        for creature in &mut self.creatures {

        }
    }

    pub fn get_creatures(&self) -> Vec<Point2D<f32>> {
        self.creatures.iter().map(|x| *x.get_position()).collect()
    }

    pub fn spawn(&mut self) -> mpsc::Sender<CreatureMessage> {
        let creature = Creature::new();
        let sender = creature.get_sender();

        self.creatures.push(creature);
        sender
    }
}