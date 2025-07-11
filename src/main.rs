mod generate;
mod llm;
mod puzzle;
mod render;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Cross Magic".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    render::loading();
    next_frame().await;
    let (questions, answers) = generate::run();
    loop {
        clear_background(DARKGRAY);
        render::letter_square(&answers);
        render::hints(&questions);
        next_frame().await
    }
}
