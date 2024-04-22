mod server;
mod assets;
mod world;
mod app;

use macroquad::prelude::*;
use crate::app::App;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sandcore client".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    App::default().run().await;
}