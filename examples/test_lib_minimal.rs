// 最小测试 - 测试库的基本功能
use termux_gui::{Activity, Result};

fn main() -> Result<()> {
    println!("Step 1: Creating activity...");
    let mut activity = Activity::new(true)?;
    println!("Step 2: Activity created! AID = {}", activity.id());
    
    println!("Step 3: Creating layout...");
    let layout = activity.create_linear_layout(None)?;
    println!("Step 4: Layout created! ID = {}", layout.id());
    
    println!("Step 5: Creating text view...");
    let text = activity.create_text_view("Hello", Some(layout.id()))?;
    println!("Step 6: Text view created! ID = {}", text.id());
    
    println!("Step 7: Setting text size...");
    text.set_text_size(&mut activity, 24)?;
    println!("Step 8: Text size set!");
    
    println!("All steps completed successfully!");
    
    // Don't start event loop, just finish
    println!("Finishing activity...");
    activity.finish()?;
    println!("Done!");
    
    Ok(())
}
