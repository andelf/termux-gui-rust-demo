// Button 交互式演示 - 简化版本（无布局参数）
// 模仿原始demo，不调用任何set_height/set_width/set_linear_layout_params
// 运行: cargo run --example button_demo_v5_simple --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Button 交互演示 (简化版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("计数器演示 🦀", Some(layout.id()))?;
    title.set_text_size(&mut activity, 30)?;
    title.view().set_margin(&mut activity, 10)?;
    
    // 创建计数显示
    let counter = activity.create_text_view("点击次数: 0", Some(layout.id()))?;
    counter.set_text_size(&mut activity, 24)?;
    counter.view().set_margin(&mut activity, 20)?;
    
    // 创建按钮布局（横向）
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    
    // 创建按钮
    let inc_button = activity.create_button("➕ 增加", Some(button_layout.id()))?;
    let dec_button = activity.create_button("➖ 减少", Some(button_layout.id()))?;
    let reset_button = activity.create_button("🔄 重置", Some(layout.id()))?;
    
    println!("✓ 界面创建完成");
    println!("  - Counter ID: {}", counter.id());
    println!("  - Button ID: {}", inc_button.id());
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示: 点击按钮进行交互");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件循环
    let mut count = 0;
    
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                break;
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
                
                // 更新文本
                counter.set_text(&mut activity, &format!("点击次数: {}", count))?;
                
                // 根据计数改变颜色
                let color = if count > 0 {
                    0xFF4CAF50u32 as i32  // 绿色
                } else if count < 0 {
                    0xFFF44336u32 as i32  // 红色
                } else {
                    0xFF2196F3u32 as i32  // 蓝色
                };
                counter.set_text_color(&mut activity, color)?;
            },
            _ => {}
        }
    }
    
    activity.finish()?;
    println!("✓ 程序结束");
    Ok(())
}
