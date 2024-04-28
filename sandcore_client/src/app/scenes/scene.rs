use std::sync::mpsc::Sender;
pub use crate::app::scenes::message::Message as SceneMessage;

pub trait Scene{
	fn update(&mut self, sender: &mut Sender<SceneMessage>);
	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &egui::Context);
}