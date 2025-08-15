mod game;
mod generate;
mod llm;
mod puzzle;
mod render;

use macroquad::prelude::*;
use serde::Deserialize;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use toml::from_str as to_toml;

#[derive(Deserialize)]
struct LlmConfig {
    model: String,
}
 
#[derive(Deserialize)]
struct DisplayConfig {
    width: usize,
    height: usize,
}

#[derive(Deserialize)]
struct CmConfig {
    display: DisplayConfig,
    llm: LlmConfig,
}

#[macroquad::main("Cross Magic")]
async fn main() {
    let toml = read_to_string("crossmagic.toml").unwrap();
    let config: CmConfig = to_toml(&toml).unwrap();

    request_new_screen_size(
        config.display.width as f32,
        config.display.height as f32,
    );

    let result = Arc::new(Mutex::new(generate::Result::default()));

    let result_for_generation = Arc::clone(&result);
    let generation = std::thread::spawn(move || {
        generate::run(&config.llm.model, result_for_generation);
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
