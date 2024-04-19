use crate::message::*;

use std::io::{Write};
use std::net::TcpStream;
use euclid::default::Vector2D;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessageClient {
    SetDirection(Vector2D<f32>),
    GetCreatures,
    Spawn,
}


impl Message for MessageClient {}
