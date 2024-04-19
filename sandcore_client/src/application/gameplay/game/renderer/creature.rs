use bevy::asset::{AssetServer, ErasedAssetLoader};
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Bundle, Component, Res};
use bevy::sprite::{Sprite, SpriteBundle};
use euclid::default::Point2D;

#[derive(Component)]
pub struct Creature{
	pub index: usize,
	pub position: Point2D<f32>,
}

impl Creature {
	pub fn new(index: usize) -> Self {
		Self {
			index,
			position: Default::default(),
		}
	}
}

#[derive(Bundle)]
pub struct CreatureBundle {
	creature: Creature,
	sprite: SpriteBundle,
}

impl CreatureBundle {
	pub fn new(index: usize, asset_server: &Res<AssetServer>) -> Self {
		let sprite = SpriteBundle {
			texture: asset_server.load("textures/walk.png"),
			sprite: Sprite{
				rect: Some(Rect{
					min: Vec2{x: 0.0, y: 0.0},
					max: Vec2{x: 128.0, y: 128.0},
				}),
				.. Default::default()
			},
			.. Default::default()
		};

		let creature = Creature::new(index);

		Self {
			sprite,
			creature,
		}
	}
}