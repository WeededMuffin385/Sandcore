use crate::app::scenes::scene::Scene;

#[derive(Default)]
pub struct State {
	pub next_scene: Option<Box<dyn Scene>>
}