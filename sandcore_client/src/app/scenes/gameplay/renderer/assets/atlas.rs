use std::collections::HashMap;
use macroquad::prelude::{Image, ImageFormat, WHITE};
use macroquad::texture::Texture2D;
use sandcore_protocol::message_server::Block;

struct Atlas{
	tile: usize,
	amount: usize,


	texture: Texture2D,
	mapping: HashMap<Block, [f32; 2]>
}

impl Atlas {
	pub fn new() -> Self {
		let tile = 64;
		let amount = 16;

		let mut atlas = Image::gen_image_color(tile as u16, (tile * amount) as u16, WHITE);
		let mut offset = 0;

		/*
		blocks.insert(Block::Vacuum, load_texture(include_bytes!("../../assets/images/blocks/vacuum.png")));
		blocks.insert(Block::Water, load_texture(include_bytes!("../../assets/images/blocks/water_3.png")));
		blocks.insert(Block::Grass, load_texture(include_bytes!("../../assets/images/blocks/grass_top.png")));
		blocks.insert(Block::Wood, load_texture(include_bytes!("../../assets/images/blocks/wood_top.png")));
		blocks.insert(Block::Dirt, load_texture(include_bytes!("../../assets/images/blocks/dirt.png")));

		 */


		let load_image = |bytes: &[u8]| {
			let image = Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap();

			for x in 0..tile {
				for y in 0..tile {
					let pixel = image.get_pixel(x, y);

					atlas.set_pixel(x, y + offset * tile, pixel);
				}
			}

			//let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
			//texture.set_filter(FilterMode::Nearest);
			//texture
		};



		Self{
			tile: 0,
			amount: 0,
			texture: Texture2D::empty(),
			mapping: Default::default(),
		}
	}
}