use std::sync::mpsc;
use crate::app::scenes::Scenes;
use crate::app::scenes::state::State;

pub trait Scene{
	fn update(&mut self, state: &mut State);
	fn update_ui(&mut self, state: &mut State, ctx: &egui::Context);
}