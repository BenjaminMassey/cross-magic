use macroquad::prelude::*;

pub fn loading() {
    clear_background(DARKGRAY);
    draw_text(
        "Loading Content...",
        400.0,
        400.0,
        160.0,
        WHITE,
    );
}

pub fn letter_square(answers: &crate::puzzle::Square) {
    for row in 0..5 {
        for column in 0..5 {
            let x = (row as f32 * 80.0) + 40.0;
            let y = (column as f32 * 80.0) + 40.0;
            draw_rectangle(x, y, 65.0, 65.0, BLACK);
            draw_text(
                &(answers.across)[column].chars().nth(row).unwrap().to_string(),
                x + 20.0,
                y + 45.0,
                42.0,
                WHITE,
            );
        }
    }
}

pub fn hints(questions: &crate::puzzle::Square) {
    draw_text("Across", 600.0, 40.0, 42.0, WHITE);
    for i in 0..5 {
        draw_text(
            &questions.across[i],
            600.0,
            (i as f32 * 26.0) + 74.0,
            22.0,
            WHITE,
        );
    }

    draw_text("Down", 600.0, 340.0, 42.0, WHITE);
    for i in 0..5 {
        draw_text(
            &questions.down[i],
            600.0,
            (i as f32 * 26.0) + 374.0,
            22.0,
            WHITE,
        );
    }
}