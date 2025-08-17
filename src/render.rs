use macroquad::prelude::*;

pub async fn loading(periods: usize, count: usize) {
    clear_background(DARKGRAY);
    draw_text(
        &("Loading Content".to_owned() + &(String::from(".").repeat(periods))),
        screen_width() * 0.167,
        screen_height() * 0.375,
        screen_width() * 0.089,
        WHITE,
    );
    draw_text(
        &format!("{count} out of 10 generated."),
        screen_width() * 0.111,
        screen_height() * 0.625,
        screen_width() * 0.089,
        WHITE,
    );
    next_frame().await;
}

pub fn letter_square(state: &crate::game::State) {
    let size = screen_width() * 0.044;
    for row in 0..5 {
        for column in 0..5 {
            let x = (row as f32 * size) + size;
            let y = (column as f32 * size) + size;
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
            draw_rectangle(x, y, size * 0.813, size * 0.813, color);
            let text_color = if selected { BLACK } else { WHITE };
            if row == 0 && column != 0 {
                draw_text(
                    &(column + 5).to_string(),
                    x + (size * 0.063),
                    y + (size * 0.188),
                    size * 0.3,
                    text_color,
                );
            } else if column == 0 {
                draw_text(
                    &(row + 1).to_string(),
                    x + (size * 0.063),
                    y + (size * 0.188),
                    size * 0.3,
                    text_color,
                );
            }
            if let Some(character) = state.board[column][row] {
                draw_text(
                    &character.to_string(),
                    x + (size * 0.25),
                    y + (size * 0.563),
                    size * 0.525,
                    text_color,
                );
            }
        }
    }
}

pub fn hints(questions: &crate::puzzle::Square) {
    draw_text(
        "Across",
        screen_width() * 0.333,
        screen_height() * 0.067,
        screen_width() * 0.023,
        WHITE,
    );
    for i in 0..5 {
        draw_text(
            &format!(
                "{}. {}",
                if i == 0 { 1 } else { i + 5 },
                &questions.across[i]
            ),
            screen_width() * 0.333,
            (i as f32 * (screen_height() * 0.043)) + (screen_height() * 0.123),
            screen_width() * 0.012,
            WHITE,
        );
    }

    draw_text(
        "Down",
        screen_width() * 0.333,
        screen_height() * 0.567,
        screen_width() * 0.023,
        WHITE,
    );
    for i in 0..5 {
        draw_text(
            &format!("{}. {}", i + 1, &questions.down[i]),
            screen_width() * 0.333,
            (i as f32 * (screen_height() * 0.043)) + (screen_height() * 0.623),
            screen_width() * 0.012,
            WHITE,
        );
    }
}

pub fn finished_state(state: &crate::game::State) {
    let (text, color) = if state.completed {
        ("Complete!", GREEN)
    } else {
        ("Incomplete", RED)
    };
    draw_text(
        text,
        screen_width() * 0.089,
        screen_height() * 0.9,
        screen_width() * 0.029,
        color,
    );
}

pub fn new_game_button() -> bool {
    let button_x = screen_width() * 0.8;
    let button_y = screen_height() * 0.85;
    let button_width = screen_width() * 0.15;
    let button_height = screen_height() * 0.08;
    
    let mouse_pos = mouse_position();
    let mouse_over = mouse_pos.0 >= button_x && mouse_pos.0 <= button_x + button_width 
                    && mouse_pos.1 >= button_y && mouse_pos.1 <= button_y + button_height;
    
    let button_color = if mouse_over { LIGHTGRAY } else { GRAY };
    let text_color = if mouse_over { BLACK } else { WHITE };
    
    draw_rectangle(button_x, button_y, button_width, button_height, button_color);
    draw_rectangle_lines(button_x, button_y, button_width, button_height, 2.0, WHITE);
    
    draw_text(
        "New Game",
        button_x + button_width * 0.15,
        button_y + button_height * 0.65,
        screen_width() * 0.02,
        text_color,
    );
    
    mouse_over && is_mouse_button_pressed(MouseButton::Left)
}
