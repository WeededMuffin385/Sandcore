mod camera;
mod assets;

use std::collections::HashMap;
use std::ops::Not;
use euclid::default::{Point3D, Vector3D};
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::*;
use macroquad::models::Vertex;
use crate::world::World;
use camera::Camera;
use sandcore_protocol::message_server::{Block, Chunk, CHUNK_SIZE, Position};
use crate::app::scenes::gameplay::renderer::assets::Assets;

pub struct Renderer {
	assets: Assets,
	pub camera: Camera,

	pub radius: isize,
}

impl Renderer {
	pub fn update(&mut self, world: &World) {
		self.camera.update();
		self.draw(world);
	}

	pub fn set_camera_position(&mut self, position: Position) {
		self.camera.set_position(position);
	}

	fn draw(&self, world: &World) {
		self.camera.set_camera();
		clear_background(BLUE);
		self.draw_blocks(world);
		self.draw_creatures(world);
		self.camera.draw_buffer();
	}

	fn draw_creatures(&self, world: &World) {
		let creature = &world.creature;

		draw_texture_ex(&self.assets.helmet, creature.position_chunk.x - 0.5, creature.position_chunk.y - 0.5, WHITE, DrawTextureParams{
			dest_size: Some(Vec2::new(1.0, 1.0)),
			.. Default::default()
		});
	}

	fn draw_blocks(&self, world: &World) {
		for x in -self.radius..=self.radius {
			for y in -self.radius..=self.radius {
				let offset = Vector3D::new(x, y, 0);
				let position_world = self.camera.position.position_world + offset;

				if let Some(chunk) = world.blocks.chunks.get(&position_world) {
					self.draw_chunk(&offset, chunk);
				}
			}
		}
	}

	fn draw_chunk(&self, offset: &Vector3D<isize>, chunk: &Chunk) {
		let next = &chunk.data;

		let offset_x = (offset.x * CHUNK_SIZE as isize) as f32;
		let offset_y = (offset.y * CHUNK_SIZE as isize) as f32;

		let mut meshes: [Mesh; CHUNK_SIZE] = [
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
			Mesh{ vertices: vec![], indices: vec![], texture: None},
		];

		for (x, next) in next.iter().enumerate() {
			for (y, next) in next.iter().enumerate() {
				for (z, block) in next.iter().enumerate().rev() {
					if z != self.camera.position.position_chunk.z as usize {continue}

					if *block == Block::Vacuum {continue}
					let entry = &mut meshes[z];
					let x = x as f32 + offset_x;
					let y = y as f32 + offset_y;

					let offset = entry.vertices.len();

					entry.vertices.extend([
						Vertex{ position: vec3(0.0 + x, 0.0 + y, 0.0), uv: vec2(0.0, 0.0), color: WHITE},
						Vertex{ position: vec3(0.0 + x, 1.0 + y, 0.0), uv: vec2(0.0, 1.0), color: WHITE},

						Vertex{ position: vec3(1.0 + x, 1.0 + y, 0.0), uv: vec2(1.0, 1.0), color: WHITE},
						Vertex{ position: vec3(1.0 + x, 0.0 + y, 0.0), uv: vec2(1.0, 0.0), color: WHITE},
					]);

					entry.indices.extend([
						(0 + offset) as u16, (1 + offset) as u16, (2 + offset) as u16,
						(2 + offset) as u16, (3 + offset) as u16, (0 + offset) as u16,
					]);
				}
			}
		}


		for mesh in &meshes {
			draw_mesh(mesh);
		}
	}
}

impl Default for Renderer {
	fn default() -> Self {
		Self {
			assets: Default::default(),
			camera: Default::default(),

			radius: 8,
		}
	}
}