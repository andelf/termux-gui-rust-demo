// Button 交互式演示 - 详细追踪版
// 用于诊断卡住问题
// 运行: cargo run --example button_demo_v4_trace --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Button 演示 (追踪版) ===\n");
    
    // 创建 Activity
    println!("[TRACE] Creating activity...");
    let mut activity = Activity::new(true)?;
    println!("[TRACE] Activity created! AID = {}\n", activity.id());
    
    // 创建主布局
    println!("[TRACE] Creating main layout...");
    let layout = activity.create_linear_layout(None)?;
    println!("[TRACE] Layout created! ID = {}", layout.id());
    
    // 创建标题
    println!("[TRACE] Creating title text view...");
    let title = activity.create_text_view("计数器演示 🦀", Some(layout.id()))?;
    println!("[TRACE] Title created! ID = {}", title.id());
    
    println!("[TRACE] Setting title size...");
    title.set_text_size(&mut activity, 30)?;
    println!("[TRACE] Title size set");
    
    println!("[TRACE] Setting title height to WRAP_CONTENT...");
    title.view().set_height_wrap_content(&mut activity)?;
    println!("[TRACE] Title height set");
    
    println!("[TRACE] Setting title linear layout params...");
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    println!("[TRACE] Title params set");
    
    // 创建计数显示
    println!("[TRACE] Creating counter text view...");
    let counter = activity.create_text_view("点击次数: 0", Some(layout.id()))?;
    println!("[TRACE] Counter created! ID = {}", counter.id());
    
    println!("[TRACE] Setting counter size...");
    counter.set_text_size(&mut activity, 24)?;
    println!("[TRACE] Counter size set");
    
    println!("[TRACE] Setting counter height...");
    counter.view().set_height_wrap_content(&mut activity)?;
    println!("[TRACE] Counter height set");
    
    println!("[TRACE] Setting counter layout params...");
    counter.view().set_linear_layout_params(&mut activity, 1, None)?;
    println!("[TRACE] Counter params set");
    
    // 创建按钮布局
    println!("[TRACE] Creating button layout...");
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    println!("[TRACE] Button layout created! ID = {}", button_layout.id());
    
    println!("[TRACE] Setting button layout height...");
    button_layout.view().set_height_wrap_content(&mut activity)?;
    println!("[TRACE] Button layout height set");
    
    println!("[TRACE] Setting button layout params...");
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    println!("[TRACE] Button layout params set");
    
    // 创建增加按钮
    println!("[TRACE] Creating increment button...");
    let inc_button = activity.create_button("➕", Some(button_layout.id()))?;
    println!("[TRACE] Inc button created! ID = {}", inc_button.id());
    
    println!("[TRACE] Setting inc button params...");
    inc_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    println!("[TRACE] Inc button params set");
    
    // 创建减少按钮
    println!("[TRACE] Creating decrement button...");
    let dec_button = activity.create_button("➖", Some(button_layout.id()))?;
    println!("[TRACE] Dec button created! ID = {}", dec_button.id());
    
    println!("[TRACE] Setting dec button params...");
    dec_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    println!("[TRACE] Dec button params set");
    
    // 创建重置按钮
    println!("[TRACE] Creating reset button...");
    let reset_button = activity.create_button("🔄 重置", Some(layout.id()))?;
    println!("[TRACE] Reset button created! ID = {}", reset_button.id());
    
    println!("[TRACE] Setting reset button height...");
    reset_button.view().set_height_wrap_content(&mut activity)?;
    println!("[TRACE] Reset button height set");
    
    println!("[TRACE] Setting reset button params...");
    reset_button.view().set_linear_layout_params(&mut activity, 0, None)?;
    println!("[TRACE] Reset button params set");
    
    println!("\n✓ 界面创建完成!");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("等待事件...");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件循环
    let mut count = 0;
    
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        println!("[事件] type = {}", event_type);
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                println!("✓ 程序结束");
                return Ok(());
            },
            "click" => {
                let id = event["value"]["id"].as_i64().unwrap_or(-1);
                
                if id == inc_button.id() {
                    count += 1;
                    println!("➕ count = {}", count);
                } else if id == dec_button.id() {
                    count -= 1;
                    println!("➖ count = {}", count);
                } else if id == reset_button.id() {
                    count = 0;
                    println!("🔄 count = {}", count);
                }
                
                counter.set_text(&mut activity, &format!("点击次数: {}", count))?;
                
                let color = if count > 0 {
                    0xFF4CAF50u32 as i32
                } else if count < 0 {
                    0xFFF44336u32 as i32
                } else {
                    0xFF2196F3u32 as i32
                };
                counter.set_text_color(&mut activity, color)?;
            },
            _ => {}
        }
    }
}
