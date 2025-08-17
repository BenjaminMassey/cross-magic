use cross_magic_ui::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "UI Library Example".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ui_manager = UIManager::new();
    
    // Create a container with multiple widgets
    let root = UIContainer::new(
        ui_manager.next_id(),
        Layout::Flex {
            direction: FlexDirection::Column,
            gap: 20.0,
            align: Alignment::Center,
        }
    )
    .with_padding(50.0)
    .add_child(Box::new(
        Text::new(ui_manager.next_id(), "Cross Magic UI Library Demo")
            .with_font_size(32.0)
            .with_color(WHITE)
            .with_alignment(TextAlignment::Center)
    ))
    .add_child(Box::new(
        Button::new(1, "Click Me!")
            .with_size(200.0, 50.0)
    ))
    .add_child(Box::new(
        Button::new(2, "Another Button")
            .with_size(200.0, 50.0)
    ))
    .add_child(Box::new(
        Panel::new(ui_manager.next_id())
            .with_background(Color::new(0.3, 0.3, 0.8, 0.5))
            .with_border(BLUE, 3.0)
    ));
    
    ui_manager.set_root(Box::new(root));
    
    loop {
        clear_background(DARKGRAY);
        
        ui_manager.update();
        ui_manager.render();
        
        // Handle events
        for event in ui_manager.get_events() {
            match event {
                WidgetEvent::ButtonClicked(id) => {
                    println!("Button {} was clicked!", id);
                }
                WidgetEvent::TextChanged(id, text) => {
                    println!("Text widget {} changed to: {}", id, text);
                }
                WidgetEvent::Custom(event_type, id) => {
                    println!("Custom event '{}' from widget {}", event_type, id);
                }
            }
        }
        
        next_frame().await;
    }
}