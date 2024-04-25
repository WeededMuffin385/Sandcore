use euclid::default::Point2D;
use serde::{Deserialize, Serialize};
use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub enum MessageServer {
	Creatures(Vec<Point2D<f32>>)
}

impl Message for MessageServer {}