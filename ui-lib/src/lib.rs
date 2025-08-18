//! # Cross Magic UI
//! 
//! A widget-based UI library for macroquad games, providing a comprehensive
//! framework for building interactive user interfaces.
//! 
//! ## Features
//! 
//! - **Widget System**: Component-based architecture with reusable widgets
//! - **Layout Engine**: Flexible layout system (flex, grid, absolute positioning)
//! - **Event Handling**: Clean event-driven interaction model
//! - **Built-in Widgets**: Button, Text, Panel, and extensible widget system
//! - **Responsive Design**: Automatic layout adaptation to different screen sizes
//! 
//! ## Quick Start
//! 
//! ```rust
//! use cross_magic_ui::*;
//! use macroquad::prelude::*;
//! 
//! #[macroquad::main("UI Demo")]
//! async fn main() {
//!     let mut ui_manager = UIManager::new();
//!     
//!     // Create a simple button
//!     let button = Button::new(1, "Click Me!")
//!         .with_size(120.0, 40.0);
//!     
//!     let root = UIContainer::new(
//!         0,
//!         Layout::Absolute { x: 100.0, y: 100.0 }
//!     )
//!     .add_child(Box::new(button));
//!     
//!     ui_manager.set_root(root);
//!     
//!     loop {
//!         clear_background(DARKGRAY);
//!         
//!         ui_manager.update();
//!         ui_manager.render();
//!         
//!         // Handle events
//!         for event in ui_manager.get_events() {
//!             match event {
//!                 WidgetEvent::ButtonClicked(id) => {
//!                     println!("Button {} clicked!", id);
//!                 }
//!             }
//!         }
//!         
//!         next_frame().await;
//!     }
//! }
//! ```

pub mod core;
pub mod widgets;
pub mod layout;

// Re-export main types for convenience
pub use core::*;
pub use widgets::*;
pub use layout::*;