use serde::{Deserialize, Serialize};
use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub enum MessageServer {
	ResponseCreatures(Vec<(f64, f64)>)
}

impl Message for MessageServer {}