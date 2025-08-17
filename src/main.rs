mod game;
mod generate;
mod puzzle;
mod render;

use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use cross_magic_ui::*;

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
    loop {
        let (answers, questions) = generate_new_puzzle().await;
        let mut state = game::State::new();
        let mut ui_manager = build_game_ui();
        
        loop {
            clear_background(DARKGRAY);
            
            ui_manager.update();
            
            let mut should_restart = false;
            for event in ui_manager.get_events() {
                match event {
                    WidgetEvent::ButtonClicked(id) if *id == 100 => {
                        should_restart = true;
                    }
                    _ => {}
                }
            }
            
            if should_restart {
                break;
            }
            
            render::letter_square(&state);
            render::hints(&questions);
            render::finished_state(&state);
            ui_manager.render();
            
            // Use the original game update logic directly instead of through UI system
            game::update(&mut state, &answers);
            
            next_frame().await
        }
    }
}

async fn generate_new_puzzle() -> (crate::puzzle::Square, crate::puzzle::Square) {
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
    
    (answers, questions)
}

fn build_game_ui() -> UIManager {
    let mut ui_manager = UIManager::new();
    
    // Position button in lower-right corner with padding from edges
    let button_width = 100.0;
    let button_height = 35.0;
    let padding = 20.0;
    
    let root = UIContainer::new(
        ui_manager.next_id(),
        Layout::Absolute { 
            x: screen_width() - button_width - padding, 
            y: screen_height() - button_height - padding 
        }
    )
    .add_child(Box::new(Button::new(100, "New Game")
        .with_size(button_width, button_height)));
    
    ui_manager.set_root(Box::new(root));
    ui_manager
}