use crate::ui::*;
use crate::game::State;
use crate::puzzle::Square;
use macroquad::prelude::*;

pub fn check_puzzle_finished(state: &State, answers: &Square) -> bool {
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

pub struct CrosswordGrid {
    id: WidgetId,
    bounds: UIRect,
    state: Option<State>,
    answers: Option<Square>,
    cell_size: f32,
}

impl CrosswordGrid {
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 300.0, 300.0),
            state: None,
            answers: None,
            cell_size: 60.0,
        }
    }

    pub fn set_game_data(&mut self, state: &State, answers: &Square) {
        self.state = Some(state.clone());
        self.answers = Some(answers.clone());
    }

    fn handle_grid_click(&mut self, mouse_x: f32, mouse_y: f32) -> Option<WidgetEvent> {
        if let Some(ref mut state) = self.state {
            let relative_x = mouse_x - self.bounds.x;
            let relative_y = mouse_y - self.bounds.y;
            
            let grid_x = (relative_x / self.cell_size) as usize;
            let grid_y = (relative_y / self.cell_size) as usize;
            
            if grid_x < 5 && grid_y < 5 {
                if state.selection.x == grid_x && state.selection.y == grid_y {
                    state.selection.across = !state.selection.across;
                } else {
                    state.selection.x = grid_x;
                    state.selection.y = grid_y;
                }
                return Some(WidgetEvent::Custom("grid_selection_changed".to_string(), self.id));
            }
        }
        None
    }

    fn render_grid(&self) {
        if let Some(ref state) = self.state {
            for row in 0..5 {
                for column in 0..5 {
                    let x = self.bounds.x + (row as f32 * self.cell_size);
                    let y = self.bounds.y + (column as f32 * self.cell_size);
                    let cell_size = self.cell_size * 0.9;
                    
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
                    
                    draw_rectangle(x, y, cell_size, cell_size, color);
                    draw_rectangle_lines(x, y, cell_size, cell_size, 2.0, WHITE);
                    
                    let text_color = if selected { BLACK } else { WHITE };
                    
                    if row == 0 && column != 0 {
                        draw_text(
                            &(column + 5).to_string(),
                            x + cell_size * 0.1,
                            y + cell_size * 0.3,
                            cell_size * 0.25,
                            text_color,
                        );
                    } else if column == 0 {
                        draw_text(
                            &(row + 1).to_string(),
                            x + cell_size * 0.1,
                            y + cell_size * 0.3,
                            cell_size * 0.25,
                            text_color,
                        );
                    }
                    
                    if let Some(character) = state.board[column][row] {
                        draw_text(
                            &character.to_string().to_uppercase(),
                            x + cell_size * 0.35,
                            y + cell_size * 0.7,
                            cell_size * 0.6,
                            text_color,
                        );
                    }
                }
            }
        }
    }

    pub fn handle_input(&mut self, input: &Input) -> Option<WidgetEvent> {
        if let Some(ref mut state) = self.state {
            for &key in &input.keys_pressed {
                match key {
                    KeyCode::Backspace => {
                        if state.board[state.selection.y][state.selection.x].is_none() {
                            if state.selection.across {
                                state.selection.x = std::cmp::max(state.selection.x, 1) - 1;
                            } else {
                                state.selection.y = std::cmp::max(state.selection.y, 1) - 1;
                            }
                        }
                        state.board[state.selection.y][state.selection.x] = None;
                        state.completed = false;
                        return Some(WidgetEvent::Custom("board_changed".to_string(), self.id));
                    }
                    KeyCode::Enter => {
                        if state.selection.across {
                            state.selection.x = 0;
                            state.selection.y = std::cmp::min(state.selection.y + 1, 4);
                        } else {
                            state.selection.x = std::cmp::min(state.selection.x + 1, 4);
                            state.selection.y = 0;
                        }
                        return Some(WidgetEvent::Custom("selection_changed".to_string(), self.id));
                    }
                    KeyCode::Tab => {
                        state.selection.across = !state.selection.across;
                        return Some(WidgetEvent::Custom("selection_changed".to_string(), self.id));
                    }
                    KeyCode::Left => {
                        state.selection.x = std::cmp::max(state.selection.x, 1) - 1;
                        return Some(WidgetEvent::Custom("selection_changed".to_string(), self.id));
                    }
                    KeyCode::Right => {
                        state.selection.x = std::cmp::min(state.selection.x + 1, 4);
                        return Some(WidgetEvent::Custom("selection_changed".to_string(), self.id));
                    }
                    KeyCode::Up => {
                        state.selection.y = std::cmp::max(state.selection.y, 1) - 1;
                        return Some(WidgetEvent::Custom("selection_changed".to_string(), self.id));
                    }
                    KeyCode::Down => {
                        state.selection.y = std::cmp::min(state.selection.y + 1, 4);
                        return Some(WidgetEvent::Custom("selection_changed".to_string(), self.id));
                    }
                    _ => {}
                }
            }

            for &character in &input.chars_pressed {
                if "abcdefghijklmnopqrstuvwxyz".contains(character) {
                    state.board[state.selection.y][state.selection.x] = Some(character);
                    if state.selection.across {
                        state.selection.x = std::cmp::min(state.selection.x + 1, 4);
                    } else {
                        state.selection.y = std::cmp::min(state.selection.y + 1, 4);
                    }
                    
                    if let Some(answers) = &self.answers {
                        state.completed = check_puzzle_finished(&state, answers);
                    }
                    
                    return Some(WidgetEvent::Custom("board_changed".to_string(), self.id));
                }
            }
        }
        None
    }

    pub fn get_state(&self) -> Option<&State> {
        self.state.as_ref()
    }
}

impl Widget for CrosswordGrid {
    fn update(&mut self, input: &Input) -> Option<WidgetEvent> {
        if input.mouse_pressed && self.bounds.contains_point(input.mouse_pos.0, input.mouse_pos.1) {
            if let Some(event) = self.handle_grid_click(input.mouse_pos.0, input.mouse_pos.1) {
                return Some(event);
            }
        }
        
        self.handle_input(input)
    }

    fn render(&self) {
        self.render_grid();
    }

    fn bounds(&self) -> UIRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: UIRect) {
        self.bounds = bounds;
        self.cell_size = f32::min(bounds.width / 5.0, bounds.height / 5.0);
    }

    fn id(&self) -> WidgetId {
        self.id
    }
}

pub struct HintsPanel {
    id: WidgetId,
    bounds: UIRect,
    questions: Option<Square>,
}

impl HintsPanel {
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 400.0, 500.0),
            questions: None,
        }
    }

    pub fn set_questions(&mut self, questions: &Square) {
        self.questions = Some(questions.clone());
    }

    fn render_hints(&self) {
        if let Some(ref questions) = self.questions {
            let font_size_title = self.bounds.height * 0.04;
            let font_size_hint = self.bounds.height * 0.025;
            let line_height = self.bounds.height * 0.035;
            
            draw_text(
                "Across",
                self.bounds.x,
                self.bounds.y + font_size_title,
                font_size_title,
                WHITE,
            );
            
            for i in 0..5 {
                let y = self.bounds.y + font_size_title + (i as f32 + 1.0) * line_height;
                draw_text(
                    &format!(
                        "{}. {}",
                        if i == 0 { 1 } else { i + 5 },
                        &questions.across[i]
                    ),
                    self.bounds.x,
                    y,
                    font_size_hint,
                    WHITE,
                );
            }

            let down_start_y = self.bounds.y + font_size_title + 7.0 * line_height;
            draw_text(
                "Down",
                self.bounds.x,
                down_start_y,
                font_size_title,
                WHITE,
            );
            
            for i in 0..5 {
                let y = down_start_y + (i as f32 + 1.0) * line_height;
                draw_text(
                    &format!("{}. {}", i + 1, &questions.down[i]),
                    self.bounds.x,
                    y,
                    font_size_hint,
                    WHITE,
                );
            }
        }
    }
}

impl Widget for HintsPanel {
    fn update(&mut self, _input: &Input) -> Option<WidgetEvent> {
        None
    }

    fn render(&self) {
        self.render_hints();
    }

    fn bounds(&self) -> UIRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: UIRect) {
        self.bounds = bounds;
    }

    fn id(&self) -> WidgetId {
        self.id
    }
}

pub struct StatusPanel {
    id: WidgetId,
    bounds: UIRect,
    completed: bool,
}

impl StatusPanel {
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 200.0, 50.0),
            completed: false,
        }
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}

impl Widget for StatusPanel {
    fn update(&mut self, _input: &Input) -> Option<WidgetEvent> {
        None
    }

    fn render(&self) {
        let (text, color) = if self.completed {
            ("Complete!", GREEN)
        } else {
            ("Incomplete", RED)
        };
        
        let font_size = self.bounds.height * 0.6;
        draw_text(
            text,
            self.bounds.x,
            self.bounds.y + self.bounds.height * 0.8,
            font_size,
            color,
        );
    }

    fn bounds(&self) -> UIRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: UIRect) {
        self.bounds = bounds;
    }

    fn id(&self) -> WidgetId {
        self.id
    }
}