//! TabLayout demo using HorizontalScrollView (v2 version)
//! 
//! Demonstrates how to create a tab layout interface with page switching.
//! Uses HorizontalScrollView for smooth page transitions.
//! 
//! **Important**: Do NOT use visibility control for switching pages, as it doesn't
//! work reliably in Termux:GUI. Always use HorizontalScrollView with scroll positioning.

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== TabLayout Demo (HorizontalScrollView approach) ===\n");

    // Create Activity
    let mut activity = Activity::new(false)?;
    println!("✓ Connection established");

    // Create root layout (vertical)
    let root = activity.create_linear_layout(None)?;
    
    // Create TabLayout at the top
    let tabs = activity.create_tab_layout(Some(root.id()))?;
    tabs.view().set_linear_layout_params(&mut activity, 0, None)?;
    tabs.view().set_height_wrap_content(&mut activity)?;
    
    // Set tab list
    tabs.set_list(&mut activity, &["Home", "Messages", "Profile"])?;
    
    println!("TabLayout created");
    
    // Create HorizontalScrollView for page content (no scrollbar, snapping, fill viewport)
    let scroll_view = activity.create_horizontal_scroll_view_with_params(
        Some(root.id()), 
        true,   // fillviewport
        true,   // snapping - snap to pages
        true    // nobar - hide scrollbar
    )?;
    scroll_view.view().set_linear_layout_params(&mut activity, 1, None)?; // weight=1 to fill remaining space
    
    // Wait for dimensions to be available
    println!("Waiting for ScrollView dimensions...");
    let mut page_width = 0;
    for _ in 0..100 {
        let dims = scroll_view.view().get_dimensions(&mut activity)?;
        if dims.0 > 0 {
            page_width = dims.0;
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    if page_width == 0 {
        println!("Error: Could not get page width");
        return Ok(());
    }
    
    println!("Page width: {} px", page_width);
    
    // Create horizontal layout inside ScrollView
    let content_layout = activity.create_linear_layout_horizontal(Some(scroll_view.id()))?;
    
    // Create three pages, each with width = page_width (in pixels!)
    // Page 1 - Home
    let page1 = activity.create_linear_layout(Some(content_layout.id()))?;
    page1.view().set_width_px(&mut activity, page_width)?;  // Must use pixels, not dp!
    page1.view().set_height_match_parent(&mut activity)?;
    
    let title1 = activity.create_text_view("📱 Home", Some(page1.id()))?;
    title1.set_text_size(&mut activity, 28)?;
    title1.view().set_height_wrap_content(&mut activity)?;
    
    let content1 = activity.create_text_view(
        "\nWelcome to TabLayout!\n\nThis is the home page.\n\n☝️ Tap tabs to switch pages", 
        Some(page1.id())
    )?;
    content1.set_text_size(&mut activity, 18)?;
    content1.view().set_height_wrap_content(&mut activity)?;
    
    // Page 2 - Messages
    let page2 = activity.create_linear_layout(Some(content_layout.id()))?;
    page2.view().set_width_px(&mut activity, page_width)?;
    page2.view().set_height_match_parent(&mut activity)?;
    
    let title2 = activity.create_text_view("💬 Messages", Some(page2.id()))?;
    title2.set_text_size(&mut activity, 28)?;
    title2.view().set_height_wrap_content(&mut activity)?;
    
    let content2 = activity.create_text_view(
        "\nYou have 3 new messages\n\n• System notification\n• Friend message\n• Update reminder", 
        Some(page2.id())
    )?;
    content2.set_text_size(&mut activity, 18)?;
    content2.view().set_height_wrap_content(&mut activity)?;
    
    // Page 3 - Profile
    let page3 = activity.create_linear_layout(Some(content_layout.id()))?;
    page3.view().set_width_px(&mut activity, page_width)?;
    page3.view().set_height_match_parent(&mut activity)?;
    
    let title3 = activity.create_text_view("👤 Profile", Some(page3.id()))?;
    title3.set_text_size(&mut activity, 28)?;
    title3.view().set_height_wrap_content(&mut activity)?;
    
    let content3 = activity.create_text_view(
        "\nProfile Information\n\n• Account settings\n• Privacy settings\n• About us", 
        Some(page3.id())
    )?;
    content3.set_text_size(&mut activity, 18)?;
    content3.view().set_height_wrap_content(&mut activity)?;
    
    println!("\n✓ UI created");
    println!("\n━━━━━━━━━━━━━━━━━━━━━━");
    println!("Tips:");
    println!("  • Tap tabs to switch pages");
    println!("  • Pages scroll smoothly");
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
            "itemselected" => {
                let view_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if view_id == tabs.id() {
                    if let Some(new_tab) = event_value["selected"].as_i64() {
                        let new_tab = new_tab as i32;
                        if new_tab >= 0 && new_tab < 3 {
                            let scroll_x = page_width * new_tab;
                            println!("Switch to tab {}: {} (scroll to {}px)", 
                                    new_tab, ["Home", "Messages", "Profile"][new_tab as usize], scroll_x);
                            
                            // Scroll to the corresponding page (x = page_width * tab_index)
                            scroll_view.set_scroll_position(&mut activity, scroll_x, 0, true)?;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
