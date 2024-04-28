use std::ops::{AddAssign, SubAssign};
use euclid::default::{Point3D, Vector3D};
use serde::{Deserialize, Serialize};
use crate::world::blocks::CHUNK_SIZE;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Position {
	pub position_chunk: Point3D<f32>,
	pub position_world: Point3D<isize>,
}

impl Position {
	fn update(&mut self) {
		update_axis(&mut self.position_chunk.x, &mut  self.position_world.x);
		update_axis(&mut self.position_chunk.y, &mut  self.position_world.y);
		update_axis(&mut self.position_chunk.z, &mut  self.position_world.z);
	}

}
fn update_axis(chunk: &mut f32, world: &mut isize) {
	while *chunk >= CHUNK_SIZE as f32 {
		*chunk -= CHUNK_SIZE as f32;
		*world += 1;
	}

	while *chunk < 0.0 {
		*chunk += CHUNK_SIZE as f32;
		*world -= 1;
	}
}

impl AddAssign<Vector3D<f32>> for Position {
	fn add_assign(&mut self, rhs: Vector3D<f32>) {
		self.position_chunk += rhs;
		self.update();
	}
}

impl SubAssign<Vector3D<f32>> for Position {
	fn sub_assign(&mut self, rhs: Vector3D<f32>) {
		self.position_chunk -= rhs;
		self.update();
	}
}

