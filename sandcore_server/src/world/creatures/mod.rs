pub(crate) mod creature;

use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;
use creature::Creature;
use crate::world::creatures::creature::message::Message as CreatureMessage;

#[derive(Default)]
pub struct Creatures {
    creatures: HashMap<usize, Creature>,
    last: usize,
}

impl Creatures {
    pub fn spawn(&mut self) -> Sender<CreatureMessage> {
        let creature = Creature::new();
        let sender = creature.get_sender();

        self.creatures.insert(self.last, creature);
        self.last += 1;
        sender
    }
}