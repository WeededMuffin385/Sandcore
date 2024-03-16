use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Settings {
    pub font_size: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_size: 12.0,
        }
    }
}