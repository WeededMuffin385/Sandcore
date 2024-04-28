use egui::ahash::{HashMap, HashSet};
use euclid::default::Point3D;
use sandcore_protocol::message_server::Chunk;

#[derive(Default)]
pub struct Blocks{
	pub chunks: HashMap<Point3D<isize>, Chunk>,
	pub requested: HashSet<Point3D<isize>>,
}