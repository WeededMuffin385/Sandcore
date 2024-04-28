use euclid::default::Point3D;
use crate::world::blocks::chunk::Chunk;

pub type Request = Point3D<isize>;
pub type Response = (Point3D<isize>, Chunk);
