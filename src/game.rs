use macroquad::prelude::*;

pub struct State {
    pub completed: bool,
    pub board: Vec<Vec<Option<char>>>,
    pub selection: Selection,
}
impl State {
    pub fn new() -> Self {
        let none = vec![None, None, None, None, None];
        Self {
            completed: false,
            board: vec![
                none.clone(),
                none.clone(),
                none.clone(),
                none.clone(),
                none.clone(),
            ],
            selection: Selection {
                x: 0,
                y: 0,
                across: true,
            },
        }
    }
}

pub struct Selection {
    pub x: usize,
    pub y: usize,
    pub across: bool,
}

pub fn update(state: &mut State, answers: &crate::puzzle::Square) {
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(pos) = mouse_pos_to_grid_pos(mouse_position()) {
            if pos == (state.selection.x, state.selection.y) {
                state.selection.across = !state.selection.across;
            } else {
                state.selection.x = pos.0;
                state.selection.y = pos.1;
            }
        }
    }
    if is_key_pressed(KeyCode::Backspace) {
        if state.board[state.selection.y][state.selection.x].is_none() {
            if state.selection.across {
                if state.selection.x > 0 {
                    state.selection.x -= 1;
                }
            } else if state.selection.y > 0 {
                state.selection.y -= 1;
            }
        }
        state.board[state.selection.y][state.selection.x] = None;
        state.completed = false;
    } else if is_key_pressed(KeyCode::Enter) {
        if state.selection.across {
            state.selection.x = 0;
            state.selection.y = std::cmp::min(state.selection.y + 1, 4);
        } else {
            state.selection.x = std::cmp::min(state.selection.x + 1, 4);
            state.selection.y = 0;
        }
    } else if let Some(character) = get_char_pressed() {
        if "abcdefghijklmnopqrstuvwxyz".contains(character) {
            state.board[state.selection.y][state.selection.x] = Some(character);
            if state.selection.across {
                state.selection.x = std::cmp::min(state.selection.x + 1, 4);
            } else {
                state.selection.y = std::cmp::min(state.selection.y + 1, 4);
            }
            state.completed = puzzle_finished(state, answers);
        }
    }
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::C) {
        for word in &answers.across {
            println!("{word}");
        }
    }
}

fn mouse_pos_to_grid_pos(mouse: (f32, f32)) -> Option<(usize, usize)> {
    let w = 65.0;
    let h = 65.0;
    for row in 0..5 {
        for column in 0..5 {
            let x = (row as f32 * 80.0) + 80.0;
            let y = (column as f32 * 80.0) + 80.0;
            if mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h {
                return Some((row, column));
            }
        }
    }
    None
}

fn puzzle_finished(state: &State, answers: &crate::puzzle::Square) -> bool {
    let target = &answers.across;
    let mut current: Vec<String> = vec![];
    for x in 0..5 {
        let mut word = String::new();
        for y in 0..5 {
            let tile = state.board[x][y];
            if let Some(c) = tile {
                word += &c.to_string();
            } else {
                return false;
            }
        }
        current.push(word);
    }
    target == &current
}
