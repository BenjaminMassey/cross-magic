//! Core UI system components including basic types, input handling, and the widget trait.

use macroquad::prelude::*;

/// A rectangle type for UI bounds, using width/height instead of macroquad's w/h
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl UIRect {
    /// Create a new UIRect
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if a point is inside this rectangle
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Get the center point of this rectangle
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// Input state for the UI system
#[derive(Debug, Clone)]
pub struct Input {
    pub mouse_pos: (f32, f32),
    pub mouse_pressed: bool,
    pub mouse_released: bool,
    pub keys_pressed: Vec<KeyCode>,
    pub chars_pressed: Vec<char>,
}

impl Input {
    /// Create a new Input instance
    pub fn new() -> Self {
        Self {
            mouse_pos: (0.0, 0.0),
            mouse_pressed: false,
            mouse_released: false,
            keys_pressed: Vec::new(),
            chars_pressed: Vec::new(),
        }
    }

    /// Update input state from macroquad
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

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for widgets
pub type WidgetId = usize;

/// Events that can be emitted by widgets
#[derive(Debug, Clone)]
pub enum WidgetEvent {
    /// A button was clicked
    ButtonClicked(WidgetId),
    /// Text content changed
    TextChanged(WidgetId, String),
    /// Custom event with string data
    Custom(String, WidgetId),
}

/// Core trait that all UI widgets must implement
pub trait Widget {
    /// Update the widget with current input, returning any events
    fn update(&mut self, input: &Input) -> Option<WidgetEvent>;
    
    /// Render the widget to the screen
    fn render(&self);
    
    /// Get the current bounds of the widget
    fn bounds(&self) -> UIRect;
    
    /// Set the bounds of the widget (called by layout system)
    fn set_bounds(&mut self, bounds: UIRect);
    
    /// Get the unique ID of this widget
    fn id(&self) -> WidgetId;
}

/// Central manager for the UI system
pub struct UIManager {
    root: Box<dyn Widget>,
    input: Input,
    events: Vec<WidgetEvent>,
    next_widget_id: WidgetId,
}

impl UIManager {
    /// Create a new UI manager with a dummy root
    pub fn new() -> Self {
        Self {
            root: Box::new(DummyWidget::new(0)),
            input: Input::new(),
            events: Vec::new(),
            next_widget_id: 1,
        }
    }

    /// Get the next available widget ID
    pub fn next_id(&mut self) -> WidgetId {
        let id = self.next_widget_id;
        self.next_widget_id += 1;
        id
    }

    /// Set the root widget
    pub fn set_root(&mut self, root: Box<dyn Widget>) {
        self.root = root;
    }

    /// Update the UI system
    pub fn update(&mut self) {
        self.input.update();
        self.root.set_bounds(UIRect::new(0.0, 0.0, screen_width(), screen_height()));
        
        self.events.clear();
        if let Some(event) = self.root.update(&self.input) {
            self.events.push(event);
        }
    }

    /// Render the UI system
    pub fn render(&self) {
        self.root.render();
    }

    /// Get events from this frame
    pub fn get_events(&self) -> &[WidgetEvent] {
        &self.events
    }

    /// Get current input state
    pub fn input(&self) -> &Input {
        &self.input
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Dummy widget used as default root
struct DummyWidget {
    id: WidgetId,
    bounds: UIRect,
}

impl DummyWidget {
    fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl Widget for DummyWidget {
    fn update(&mut self, _input: &Input) -> Option<WidgetEvent> {
        None
    }

    fn render(&self) {
        // Do nothing
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