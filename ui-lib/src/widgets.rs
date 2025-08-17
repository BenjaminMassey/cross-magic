//! Built-in widgets including Button, Text, and Panel.

use crate::core::*;
use macroquad::prelude::*;

/// Button state for visual feedback
#[derive(Debug, Clone)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

/// A clickable button widget with text
pub struct Button {
    id: WidgetId,
    bounds: UIRect,
    text: String,
    state: ButtonState,
    enabled: bool,
}

impl Button {
    /// Create a new button with the given ID and text
    pub fn new(id: WidgetId, text: impl Into<String>) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 120.0, 40.0),
            text: text.into(),
            state: ButtonState::Normal,
            enabled: true,
        }
    }

    /// Set the size of the button
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.bounds.width = width;
        self.bounds.height = height;
        self
    }

    /// Set whether the button is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    fn get_colors(&self) -> (Color, Color) {
        if !self.enabled {
            return (DARKGRAY, GRAY);
        }

        match self.state {
            ButtonState::Normal => (GRAY, WHITE),
            ButtonState::Hovered => (LIGHTGRAY, BLACK),
            ButtonState::Pressed => (DARKGRAY, WHITE),
        }
    }
}

impl Widget for Button {
    fn update(&mut self, input: &Input) -> Option<WidgetEvent> {
        if !self.enabled {
            return None;
        }

        let mouse_over = self.bounds.contains_point(input.mouse_pos.0, input.mouse_pos.1);
        
        if mouse_over {
            if input.mouse_pressed {
                self.state = ButtonState::Pressed;
                return Some(WidgetEvent::ButtonClicked(self.id));
            } else {
                self.state = ButtonState::Hovered;
            }
        } else {
            self.state = ButtonState::Normal;
        }

        None
    }

    fn render(&self) {
        let (bg_color, text_color) = self.get_colors();
        
        draw_rectangle(self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height, bg_color);
        draw_rectangle_lines(self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height, 2.0, WHITE);
        
        let font_size = self.bounds.height * 0.4;
        let text_width = measure_text(&self.text, None, font_size as u16, 1.0).width;
        let text_x = self.bounds.x + (self.bounds.width - text_width) / 2.0;
        let text_y = self.bounds.y + self.bounds.height / 2.0 + font_size / 3.0;
        
        draw_text(&self.text, text_x, text_y, font_size, text_color);
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

/// Text alignment options
#[derive(Debug, Clone)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

/// A text display widget
pub struct Text {
    id: WidgetId,
    bounds: UIRect,
    text: String,
    color: Color,
    font_size: f32,
    alignment: TextAlignment,
}

impl Text {
    /// Create a new text widget
    pub fn new(id: WidgetId, text: impl Into<String>) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 200.0, 30.0),
            text: text.into(),
            color: WHITE,
            font_size: 20.0,
            alignment: TextAlignment::Left,
        }
    }

    /// Set the text color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the text alignment
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Update the text content
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

impl Widget for Text {
    fn update(&mut self, _input: &Input) -> Option<WidgetEvent> {
        None
    }

    fn render(&self) {
        let text_width = measure_text(&self.text, None, self.font_size as u16, 1.0).width;
        
        let text_x = match self.alignment {
            TextAlignment::Left => self.bounds.x,
            TextAlignment::Center => self.bounds.x + (self.bounds.width - text_width) / 2.0,
            TextAlignment::Right => self.bounds.x + self.bounds.width - text_width,
        };
        
        let text_y = self.bounds.y + self.bounds.height / 2.0 + self.font_size / 3.0;
        
        draw_text(&self.text, text_x, text_y, self.font_size, self.color);
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

/// A rectangular panel widget with background and optional border
pub struct Panel {
    id: WidgetId,
    bounds: UIRect,
    background_color: Color,
    border_color: Option<Color>,
    border_width: f32,
}

impl Panel {
    /// Create a new panel
    pub fn new(id: WidgetId) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 100.0, 100.0),
            background_color: Color::new(0.2, 0.2, 0.2, 0.8),
            border_color: Some(WHITE),
            border_width: 1.0,
        }
    }

    /// Set the background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the border color and width
    pub fn with_border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    /// Remove the border
    pub fn no_border(mut self) -> Self {
        self.border_color = None;
        self
    }
}

impl Widget for Panel {
    fn update(&mut self, _input: &Input) -> Option<WidgetEvent> {
        None
    }

    fn render(&self) {
        draw_rectangle(
            self.bounds.x, 
            self.bounds.y, 
            self.bounds.width, 
            self.bounds.height, 
            self.background_color
        );
        
        if let Some(border_color) = self.border_color {
            draw_rectangle_lines(
                self.bounds.x, 
                self.bounds.y, 
                self.bounds.width, 
                self.bounds.height, 
                self.border_width, 
                border_color
            );
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