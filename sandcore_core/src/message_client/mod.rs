use serde::{Deserialize, Serialize};
use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub enum MessageClient {
	RequestCreatures,
}

impl Message for MessageClient{}