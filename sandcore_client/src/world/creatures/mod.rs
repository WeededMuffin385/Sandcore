mod creature;

use std::collections::HashMap;
use crate::world::creatures::creature::Creature;

#[derive(Default)]
pub struct Creatures {
    creatures: HashMap<usize, Creature>,
}