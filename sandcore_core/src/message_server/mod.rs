use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessageServer {
    Ok,
    Err(String),
}