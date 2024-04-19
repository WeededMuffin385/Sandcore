use tokio::sync::watch::Sender;

use crate::world::creatures::creature::message::{Message as CreatureMessage, Request as CreatureRequest, Response as CreatureResponse};

struct Intelligence{
	sender_creature: Sender<CreatureMessage>,
}


impl Intelligence{
	pub fn new(sender_creature: Sender<CreatureMessage>) -> Self {
		Self {
			sender_creature,
		}
	}

	pub fn update(&mut self){

	}
}