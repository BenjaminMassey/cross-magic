# Cross Magic UI

A widget-based UI library for macroquad games, providing a comprehensive framework for building interactive user interfaces.

## Features

- **Widget System**: Component-based architecture with reusable widgets
- **Layout Engine**: Flexible layout system (flex, grid, absolute positioning)
- **Event Handling**: Clean event-driven interaction model
- **Built-in Widgets**: Button, Text, Panel, and extensible widget system
- **Responsive Design**: Automatic layout adaptation to different screen sizes

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
cross-magic-ui = { path = "path/to/ui-lib" }
macroquad = "0.4.14"
```

## Basic Usage

```rust
use cross_magic_ui::*;
use macroquad::prelude::*;

#[macroquad::main("UI Demo")]
async fn main() {
    let mut ui_manager = UIManager::new();
    
    // Create a simple button
    let button = Button::new(1, "Click Me!")
        .with_size(120.0, 40.0);
    
    let root = UIContainer::new(
        0,
        Layout::Absolute { x: 100.0, y: 100.0 }
    )
    .add_child(Box::new(button));
    
    ui_manager.set_root(Box::new(root));
    
    loop {
        clear_background(DARKGRAY);
        
        ui_manager.update();
        ui_manager.render();
        
        // Handle events
        for event in ui_manager.get_events() {
            match event {
                WidgetEvent::ButtonClicked(id) => {
                    println!("Button {} clicked!", id);
                }
                _ => {}
            }
        }
        
        next_frame().await;
    }
}
```

## Widgets

### Button

Interactive button with hover states and click events.

```rust
let button = Button::new(1, "My Button")
    .with_size(200.0, 50.0)
    .enabled(true);
```

### Text

Text display with alignment and styling options.

```rust
let text = Text::new(2, "Hello World!")
    .with_font_size(24.0)
    .with_color(WHITE)
    .with_alignment(TextAlignment::Center);
```

### Panel

Background panel with optional borders.

```rust
let panel = Panel::new(3)
    .with_background(Color::new(0.2, 0.2, 0.2, 0.8))
    .with_border(WHITE, 2.0);
```

## Layout System

### Flex Layout

Arrange widgets in rows or columns with gap and alignment.

```rust
let container = UIContainer::new(
    0,
    Layout::Flex {
        direction: FlexDirection::Column,
        gap: 10.0,
        align: Alignment::Center,
    }
);
```

### Grid Layout

Arrange widgets in a grid pattern.

```rust
let container = UIContainer::new(
    0,
    Layout::Grid {
        cols: 3,
        rows: 2,
        gap: 5.0,
    }
);
```

### Absolute Layout

Position widgets at specific offsets.

```rust
let container = UIContainer::new(
    0,
    Layout::Absolute { x: 50.0, y: 100.0 }
);
```

## Examples

Run the examples to see the library in action:

```bash
cargo run --example simple_button
```

## Architecture

The library is built around a few core concepts:

- **Widget**: The base trait for all UI elements
- **UIManager**: Central coordinator for the entire UI system
- **UIContainer**: Layout container that can hold and arrange other widgets
- **Events**: Type-safe event system for widget interactions

## Extending

Create custom widgets by implementing the `Widget` trait:

```rust
struct MyWidget {
    id: WidgetId,
    bounds: UIRect,
    // ... your fields
}

impl Widget for MyWidget {
    fn update(&mut self, input: &Input) -> Option<WidgetEvent> {
        // Handle input and return events
    }
    
    fn render(&self) {
        // Render your widget
    }
    
    fn bounds(&self) -> UIRect { self.bounds }
    fn set_bounds(&mut self, bounds: UIRect) { self.bounds = bounds; }
    fn id(&self) -> WidgetId { self.id }
}
```

## License

MIT OR Apache-2.0