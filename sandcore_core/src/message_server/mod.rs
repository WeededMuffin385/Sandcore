use std::collections::HashMap;
use std::net::TcpStream;
use euclid::default::{Point2D, Vector2D};
use serde::{Deserialize, Serialize};
use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub enum MessageServer {
    SpawnSuccess,
    SpawnFailed,
    Creatures(Vec<Point2D<f32>>),
}

impl Message for MessageServer {}