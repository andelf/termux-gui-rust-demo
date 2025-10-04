//! Enhanced Features Demo
//! 
//! Demonstrates newly added high-priority methods:
//! - View::set_background_color()
//! - TextView::get_text()
//! - TabLayout::select_tab()
//! - View::set_grid_layout_params()

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Enhanced Features Demo ===\n");
    
    let mut activity = Activity::new(false)?;
    println!("✓ Connection established\n");
    
    // Create root scroll view
    let scroll = activity.create_nested_scroll_view(None)?;
    let root = activity.create_linear_layout(Some(scroll.id()))?;
    root.view().set_margin(&mut activity, 20)?;
    
    // === 1. Background Color Demo ===
    println!("1. Background Color Demo");
    let bg_title = activity.create_text_view("1. Background Colors", Some(root.id()))?;
    bg_title.set_text_size(&mut activity, 22)?;
    bg_title.view().set_height_wrap_content(&mut activity)?;
    bg_title.view().set_margin(&mut activity, 10)?;
    
    // Color boxes layout
    let color_layout = activity.create_linear_layout_horizontal(Some(root.id()))?;
    color_layout.view().set_height_wrap_content(&mut activity)?;
    color_layout.view().set_margin(&mut activity, 5)?;
    
    // Red box
    let red_box = activity.create_text_view("  Red  ", Some(color_layout.id()))?;
    red_box.set_text_size(&mut activity, 16)?;
    red_box.view().set_height_wrap_content(&mut activity)?;
    red_box.view().set_margin(&mut activity, 5)?;
    red_box.view().set_background_color(&mut activity, 0xFFFF0000u32 as i32)?;
    red_box.set_text_color(&mut activity, 0xFFFFFFFFu32 as i32)?; // White text
    
    // Green box
    let green_box = activity.create_text_view(" Green ", Some(color_layout.id()))?;
    green_box.set_text_size(&mut activity, 16)?;
    green_box.view().set_height_wrap_content(&mut activity)?;
    green_box.view().set_margin(&mut activity, 5)?;
    green_box.view().set_background_color(&mut activity, 0xFF00FF00u32 as i32)?;
    
    // Blue box
    let blue_box = activity.create_text_view("  Blue  ", Some(color_layout.id()))?;
    blue_box.set_text_size(&mut activity, 16)?;
    blue_box.view().set_height_wrap_content(&mut activity)?;
    blue_box.view().set_margin(&mut activity, 5)?;
    blue_box.view().set_background_color(&mut activity, 0xFF0000FFu32 as i32)?;
    blue_box.set_text_color(&mut activity, 0xFFFFFFFFu32 as i32)?; // White text
    
    // === 2. Get Text Demo ===
    println!("2. Get Text Demo");
    let text_title = activity.create_text_view("\n2. Get Text Demo", Some(root.id()))?;
    text_title.set_text_size(&mut activity, 22)?;
    text_title.view().set_height_wrap_content(&mut activity)?;
    text_title.view().set_margin(&mut activity, 10)?;
    
    let input = activity.create_edit_text("Type something here...", Some(root.id()))?;
    input.view().set_width_match_parent(&mut activity)?;
    input.view().set_height_wrap_content(&mut activity)?;
    input.view().set_margin(&mut activity, 5)?;
    
    let get_text_btn = activity.create_button("Get Text", Some(root.id()))?;
    get_text_btn.view().set_width_match_parent(&mut activity)?;
    get_text_btn.view().set_height_wrap_content(&mut activity)?;
    get_text_btn.view().set_margin(&mut activity, 5)?;
    
    let text_result = activity.create_text_view("", Some(root.id()))?;
    text_result.set_text_size(&mut activity, 16)?;
    text_result.view().set_height_wrap_content(&mut activity)?;
    text_result.view().set_margin(&mut activity, 5)?;
    text_result.set_text_color(&mut activity, 0xFF0088FFu32 as i32)?;
    
    // === 3. Grid Layout with Params Demo ===
    println!("3. Grid Layout Demo");
    let grid_title = activity.create_text_view("\n3. Grid Layout (3x3)", Some(root.id()))?;
    grid_title.set_text_size(&mut activity, 22)?;
    grid_title.view().set_height_wrap_content(&mut activity)?;
    grid_title.view().set_margin(&mut activity, 10)?;
    
    let grid = activity.create_grid_layout(3, 3, Some(root.id()))?;
    grid.view().set_width_match_parent(&mut activity)?;
    grid.view().set_height_wrap_content(&mut activity)?;
    grid.view().set_margin(&mut activity, 10)?;
    
    // Create 9 colored cells with grid layout params
    let colors: [(u32, &str); 9] = [
        (0xFFFF6B6B, "1"), (0xFF4ECDC4, "2"), (0xFF45B7D1, "3"),
        (0xFFFFA07A, "4"), (0xFF98D8C8, "5"), (0xFFF7B731, "6"),
        (0xFFEE5A6F, "7"), (0xFF778BEB, "8"), (0xFF786FA6, "9"),
    ];
    
    for (idx, &(color, text)) in colors.iter().enumerate() {
        let row = (idx / 3) as i32;
        let col = (idx % 3) as i32;
        
        let cell = activity.create_text_view(text, Some(grid.id()))?;
        cell.set_text_size(&mut activity, 24)?;
        cell.view().set_background_color(&mut activity, color as i32)?;
        cell.set_text_color(&mut activity, 0xFFFFFFFFu32 as i32)?;
        cell.view().set_width(&mut activity, 80)?;
        cell.view().set_height(&mut activity, 80)?;
        // Use the new set_grid_layout_params method
        cell.view().set_grid_layout_params(&mut activity, row, col, 1, 1, "center", "center")?;
    }
    
    // === 4. TabLayout with select_tab Demo ===
    println!("4. TabLayout Demo");
    let tab_title = activity.create_text_view("\n4. Programmatic Tab Selection", Some(root.id()))?;
    tab_title.set_text_size(&mut activity, 22)?;
    tab_title.view().set_height_wrap_content(&mut activity)?;
    tab_title.view().set_margin(&mut activity, 10)?;
    
    let tabs = activity.create_tab_layout(Some(root.id()))?;
    tabs.view().set_height_wrap_content(&mut activity)?;
    tabs.view().set_margin(&mut activity, 5)?;
    tabs.set_list(&mut activity, &["Tab 1", "Tab 2", "Tab 3"])?;
    
    let tab_btns_layout = activity.create_linear_layout_horizontal(Some(root.id()))?;
    tab_btns_layout.view().set_height_wrap_content(&mut activity)?;
    tab_btns_layout.view().set_margin(&mut activity, 5)?;
    
    let select_tab1 = activity.create_button("Select Tab 1", Some(tab_btns_layout.id()))?;
    select_tab1.view().set_height_wrap_content(&mut activity)?;
    select_tab1.view().set_margin(&mut activity, 2)?;
    
    let select_tab2 = activity.create_button("Select Tab 2", Some(tab_btns_layout.id()))?;
    select_tab2.view().set_height_wrap_content(&mut activity)?;
    select_tab2.view().set_margin(&mut activity, 2)?;
    
    let select_tab3 = activity.create_button("Select Tab 3", Some(tab_btns_layout.id()))?;
    select_tab3.view().set_height_wrap_content(&mut activity)?;
    select_tab3.view().set_margin(&mut activity, 2)?;
    
    println!("\n✓ UI created");
    println!("\n━━━━━━━━━━━━━━━━━━━━━━");
    println!("Try the features:");
    println!("  • See colored backgrounds");
    println!("  • Type text and click 'Get Text'");
    println!("  • Click buttons to select tabs");
    println!("  • View the 3x3 grid layout");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Event loop
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity closed");
                return Ok(());
            }
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == get_text_btn.id() {
                    // Use TextView::get_text() to retrieve the input
                    let text = input.get_text(&mut activity)?;
                    println!("Got text: '{}'", text);
                    text_result.set_text(&mut activity, &format!("You typed: {}", text))?;
                } else if clicked_id == select_tab1.id() {
                    println!("Selecting Tab 1");
                    tabs.select_tab(&mut activity, 0)?;
                } else if clicked_id == select_tab2.id() {
                    println!("Selecting Tab 2");
                    tabs.select_tab(&mut activity, 1)?;
                } else if clicked_id == select_tab3.id() {
                    println!("Selecting Tab 3");
                    tabs.select_tab(&mut activity, 2)?;
                }
            }
            _ => {}
        }
    }
}
