use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Block {
	#[default]
	Vacuum,
	Water,
	Grass,
	Wood,
	Dirt,
}