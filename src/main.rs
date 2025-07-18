mod game;
mod generate;
mod llm;
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
    let questions = Arc::new(Mutex::new(None::<puzzle::Square>));
    let answers = Arc::new(Mutex::new(None::<puzzle::Square>));

    let questions_clone = Arc::clone(&questions);
    let answers_clone = Arc::clone(&answers);
    let generation = std::thread::spawn(move || {
        let (q, a) = generate::run();
        let mut questions_lock = questions_clone.lock().unwrap();
        *questions_lock = Some(q);
        let mut answers_lock = answers_clone.lock().unwrap();
        *answers_lock = Some(a);
    });

    let mut i = 0;
    while questions.lock().unwrap().is_none() || answers.lock().unwrap().is_none() {
        render::loading(i).await;
        i = (i + 1) % 4;
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    generation.join().expect("Error with loading screen.");

    let answers = answers
        .lock()
        .expect("Thread error.")
        .take()
        .expect("Answers error.");
    let questions = questions
        .lock()
        .expect("Thread error.")
        .take()
        .expect("Questions error.");

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
