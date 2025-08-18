//! Layout system for organizing widgets in containers.

use crate::core::*;
use macroquad::prelude::*;

/// Direction for flex layout
#[derive(Debug, Clone)]
pub enum FlexDirection {
    Row,
    Column,
}

/// Alignment options for layout
#[derive(Debug, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}

/// Layout types for containers
#[derive(Debug, Clone)]
pub enum Layout {
    /// Flex layout with direction, gap, and alignment
    Flex {
        direction: FlexDirection,
        gap: f32,
        align: Alignment,
    },
    /// Grid layout with columns, rows, and gap
    Grid {
        cols: usize,
        rows: usize,
        gap: f32,
    },
    /// Absolute positioning with offset from parent
    Absolute {
        x: f32,
        y: f32,
    },
}

/// Container widget that can hold and layout other widgets
pub struct UIContainer {
    id: WidgetId,
    bounds: UIRect,
    layout: Layout,
    children: Vec<Box<dyn Widget>>,
    padding: f32,
}

impl UIContainer {
    /// Create a new container with the specified layout
    pub fn new(id: WidgetId, layout: Layout) -> Self {
        Self {
            id,
            bounds: UIRect::new(0.0, 0.0, 0.0, 0.0),
            layout,
            children: Vec::new(),
            padding: 0.0,
        }
    }

    /// Add padding to the container
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Add a child widget to the container
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
                    Alignment::Center => bounds.y,
                    Alignment::End => bounds.y,
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
                    Alignment::Center => bounds.x,
                    Alignment::End => bounds.x,
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