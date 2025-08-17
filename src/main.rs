mod game;
mod generate;
mod puzzle;
mod render;

use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

fn conf() -> Conf {
    Conf {
        window_title: "Cross Magic".to_owned(),
        window_width: 1800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let result = Arc::new(Mutex::new(generate::Result::default()));

    let result_for_generation = Arc::clone(&result);
    let generation = std::thread::spawn(move || {
        generate::run(result_for_generation);
    });

    let mut i = 0;
    while !result.lock().unwrap().complete() {
        let count = result.lock().unwrap().count;
        render::loading(i, count).await;
        i = (i + 1) % 4;
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    generation.join().expect("Error with loading screen.");

    let answers = result.lock().unwrap().clone().answers.unwrap();
    let questions = result.lock().unwrap().clone().questions.unwrap();

    let mut state = game::State::new();

    loop {
        clear_background(DARKGRAY);
        render::letter_square(&state);
        render::hints(&questions);
        render::finished_state(&state);
        game::update(&mut state, &answers);
        next_frame().await
    }
}