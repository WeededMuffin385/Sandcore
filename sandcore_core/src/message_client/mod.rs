use euclid::default::Vector2D;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessageClient {
    SetDirection(Vector2D<f32>)
}