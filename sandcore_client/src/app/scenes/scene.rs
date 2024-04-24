use std::sync::mpsc;
use std::sync::mpsc::Sender;
use crate::app::scenes::Scenes;
pub use crate::app::scenes::message::Message as SceneMessage;

pub trait Scene{
	fn update(&mut self, sender: &mut Sender<SceneMessage>);
	fn update_ui(&mut self, sender: &mut Sender<SceneMessage>, ctx: &egui::Context);
}