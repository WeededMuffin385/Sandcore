use euclid::default::Vector2D;

pub enum Message {
    SetDirection(Vector2D<f32>),
}