use macroquad::prelude::*;

pub struct Assets {
	pub helmet: Texture2D,
}

impl Default for Assets {
	fn default() -> Self {
		fn load_texture(bytes: &[u8]) -> Texture2D {
			let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
			texture.set_filter(FilterMode::Nearest);
			texture
		}

		let helmet = load_texture(include_bytes!("../../assets/images/icons/helmet.png"));

		Self {
			helmet
		}
	}
}