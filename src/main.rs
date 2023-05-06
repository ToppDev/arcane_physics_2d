use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rong".to_owned(),
        window_width: 1280,
        window_height: 800,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> arcane_physics_2d::GameResult {
    arcane_physics_2d::entry_point().await
}
