use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl UIRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub mouse_pos: (f32, f32),
    pub mouse_pressed: bool,
    pub mouse_released: bool,
    pub keys_pressed: Vec<KeyCode>,
    pub chars_pressed: Vec<char>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pos: (0.0, 0.0),
            mouse_pressed: false,
            mouse_released: false,
            keys_pressed: Vec::new(),
            chars_pressed: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.mouse_pos = mouse_position();
        self.mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.mouse_released = is_mouse_button_released(MouseButton::Left);
        
        self.keys_pressed.clear();
        self.chars_pressed.clear();
        
        // Only track specific keys for UI, don't consume character input
        for key in [KeyCode::Enter, KeyCode::Backspace, KeyCode::Tab, KeyCode::Left, 
                   KeyCode::Right, KeyCode::Up, KeyCode::Down] {
            if is_key_pressed(key) {
                self.keys_pressed.push(key);
            }
        }
        
        // Don't consume get_char_pressed() here - let the game handle it
    }
}

pub type WidgetId = usize;

pub trait Widget {
    fn update(&mut self, input: &Input) -> Option<WidgetEvent>;
    fn render(&self);
    fn bounds(&self) -> UIRect;
    fn set_bounds(&mut self, bounds: UIRect);
    fn id(&self) -> WidgetId;
}

#[derive(Debug, Clone)]
pub enum WidgetEvent {
    ButtonClicked(WidgetId),
    TextChanged(WidgetId, String),
    Custom(String, WidgetId),
}

#[derive(Debug, Clone)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone)]
pub enum Layout {
    Flex {
        direction: FlexDirection,
        gap: f32,
        align: Alignment,
    },
    Grid {
        cols: usize,
        rows: usize,
        gap: f32,
    },
    Absolute {
        x: f32,
        y: f32,
    },
}

pub struct UIContainer {
    id: WidgetId,
    bounds: UIRect,
    layout: Layout,
    children: Vec<Box<dyn Widget>>,
    padding: f32,
}

impl UIContainer {
    pub fn new(id: WidgetId, layout: Layout) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 0.0, 0.0),
            layout,
            children: Vec::new(),
            padding: 0.0,
        }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }

    fn layout_children(&mut self) {
        let inner_bounds = UIRect::new(
            self.bounds.x + self.padding,
            self.bounds.y + self.padding,
            self.bounds.width - 2.0 * self.padding,
            self.bounds.height - 2.0 * self.padding,
        );

        let layout = self.layout.clone();
        match layout {
            Layout::Flex { direction, gap, align } => {
                self.layout_flex(&inner_bounds, &direction, gap, &align);
            }
            Layout::Grid { cols, rows, gap } => {
                self.layout_grid(&inner_bounds, cols, rows, gap);
            }
            Layout::Absolute { x, y } => {
                for child in &mut self.children {
                    let mut child_bounds = child.bounds();
                    child_bounds.x = inner_bounds.x + x;
                    child_bounds.y = inner_bounds.y + y;
                    child.set_bounds(child_bounds);
                }
            }
        }
    }

    fn layout_flex(&mut self, bounds: &UIRect, direction: &FlexDirection, gap: f32, align: &Alignment) {
        if self.children.is_empty() {
            return;
        }

        let total_gap = gap * (self.children.len() as f32 - 1.0);
        
        match direction {
            FlexDirection::Row => {
                let child_width = (bounds.width - total_gap) / self.children.len() as f32;
                let start_y = match align {
                    Alignment::Start => bounds.y,
                    Alignment::Center => bounds.y + (bounds.height - bounds.height) / 2.0,
                    Alignment::End => bounds.y + bounds.height - bounds.height,
                };

                for (i, child) in self.children.iter_mut().enumerate() {
                    let x = bounds.x + i as f32 * (child_width + gap);
                    child.set_bounds(UIRect::new(x, start_y, child_width, bounds.height));
                }
            }
            FlexDirection::Column => {
                let child_height = (bounds.height - total_gap) / self.children.len() as f32;
                let start_x = match align {
                    Alignment::Start => bounds.x,
                    Alignment::Center => bounds.x + (bounds.width - bounds.width) / 2.0,
                    Alignment::End => bounds.x + bounds.width - bounds.width,
                };

                for (i, child) in self.children.iter_mut().enumerate() {
                    let y = bounds.y + i as f32 * (child_height + gap);
                    child.set_bounds(UIRect::new(start_x, y, bounds.width, child_height));
                }
            }
        }
    }

    fn layout_grid(&mut self, bounds: &UIRect, cols: usize, rows: usize, gap: f32) {
        if self.children.is_empty() {
            return;
        }

        let cell_width = (bounds.width - gap * (cols as f32 - 1.0)) / cols as f32;
        let cell_height = (bounds.height - gap * (rows as f32 - 1.0)) / rows as f32;

        for (i, child) in self.children.iter_mut().enumerate() {
            let col = i % cols;
            let row = i / cols;
            if row >= rows {
                break;
            }

            let x = bounds.x + col as f32 * (cell_width + gap);
            let y = bounds.y + row as f32 * (cell_height + gap);
            child.set_bounds(UIRect::new(x, y, cell_width, cell_height));
        }
    }
}

impl Widget for UIContainer {
    fn update(&mut self, input: &Input) -> Option<WidgetEvent> {
        self.layout_children();
        
        for child in &mut self.children {
            if let Some(event) = child.update(input) {
                return Some(event);
            }
        }
        None
    }

    fn render(&self) {
        for child in &self.children {
            child.render();
        }
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

pub struct UIManager {
    root: UIContainer,
    input: Input,
    events: Vec<WidgetEvent>,
    next_widget_id: WidgetId,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            root: UIContainer::new(0, Layout::Flex { 
                direction: FlexDirection::Column, 
                gap: 0.0, 
                align: Alignment::Start 
            }),
            input: Input::new(),
            events: Vec::new(),
            next_widget_id: 1,
        }
    }

    pub fn next_id(&mut self) -> WidgetId {
        let id = self.next_widget_id;
        self.next_widget_id += 1;
        id
    }

    pub fn set_root(&mut self, root: UIContainer) {
        self.root = root;
    }

    pub fn update(&mut self) {
        self.input.update();
        self.root.set_bounds(UIRect::new(0.0, 0.0, screen_width(), screen_height()));
        
        self.events.clear();
        if let Some(event) = self.root.update(&self.input) {
            self.events.push(event);
        }
    }

    pub fn render(&self) {
        self.root.render();
    }

    pub fn get_events(&self) -> &[WidgetEvent] {
        &self.events
    }

    pub fn input(&self) -> &Input {
        &self.input
    }
}