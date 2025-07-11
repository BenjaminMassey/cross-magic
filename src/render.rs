use macroquad::prelude::*;

pub async fn loading(periods: usize) {
    clear_background(DARKGRAY);
    draw_text(
        &("Loading Content".to_owned() + &(String::from(".").repeat(periods))),
        300.0,
        300.0,
        160.0,
        WHITE,
    );
    next_frame().await;
}

pub fn letter_square(state: &crate::game::State) {
    for row in 0..5 {
        for column in 0..5 {
            let x = (row as f32 * 80.0) + 40.0;
            let y = (column as f32 * 80.0) + 40.0;
            let inline = if state.selection.across {
                state.selection.y == column
            } else {
                state.selection.x == row
            };
            let selected = state.selection.x == row && state.selection.y == column;
            let color = if selected {
                YELLOW
            } else if inline {
                BEIGE
            } else {
                BLACK
            };
            draw_rectangle(x, y, 65.0, 65.0, color);
            if let Some(character) = state.board[column][row] {
                draw_text(
                    &character.to_string(),
                    x + 20.0,
                    y + 45.0,
                    42.0,
                    WHITE,
                );
            }
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

pub fn finished_state(state: &crate::game::State) {
    if state.completed {
        draw_text(
            "Complete!",
            120.0,
            500.0,
            52.0,
            DARKGREEN,
        );
    } else {
        draw_text(
            "Incomplete",
            120.0,
            500.0,
            52.0,
            MAROON,
        );
    }
}