use creatures::Creatures;

mod creatures;

#[derive(Default)]
pub struct World {
    creatures: Creatures,
}