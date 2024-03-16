use creatures::Creatures;

pub mod creatures;

#[derive(Default)]
pub struct World {
    pub creatures: Creatures,
}