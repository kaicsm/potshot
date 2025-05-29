
use macroquad::prelude::*;

mod game;
mod types;
mod constants;
mod colors;
mod config;
mod ball;
mod pocket;
mod cue;
mod drawing;
mod physics;
mod rules;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "PotShot".to_owned(),
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_instance = Game::new().await;
    let mut last_screen_width = screen_width();
    let mut last_screen_height = screen_height();

    loop {
        if screen_width() != last_screen_width || screen_height() != last_screen_height {
            game_instance.resize_and_init(); // Assume Game has this public method
            last_screen_width = screen_width();
            last_screen_height = screen_height();
        }

        game_instance.process_input();
        game_instance.update();
        game_instance.render();

        next_frame().await
    }
}

