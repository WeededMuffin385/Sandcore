use euclid::default::Point2D;

#[derive(Default)]
pub struct World{
	pub creatures: Vec<Point2D<f32>>,
}