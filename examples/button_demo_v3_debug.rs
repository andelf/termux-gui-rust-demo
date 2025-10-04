// Button 演示 - 调试版本
// 找出卡住的具体位置

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Button 调试版本 ===\n");
    
    println!("[1/10] 创建 Activity...");
    let mut activity = Activity::new(true)?;
    println!("✓ Activity 创建成功 (ID = {})\n", activity.id());
    
    println!("[2/10] 创建主布局...");
    let layout = activity.create_linear_layout(None)?;
    println!("✓ 布局创建成功 (ID = {})\n", layout.id());
    
    println!("[3/10] 创建标题...");
    let title = activity.create_text_view("计数器演示 🦀", Some(layout.id()))?;
    println!("✓ 标题创建成功 (ID = {})\n", title.id());
    
    println!("[4/10] 设置标题大小...");
    title.set_text_size(&mut activity, 30)?;
    println!("✓ 标题大小设置成功\n");
    
    println!("[5/10] 设置标题边距...");
    title.view().set_margin(&mut activity, 10)?;
    println!("✓ 标题边距设置成功\n");
    
    println!("[6/10] 创建计数器文本...");
    let counter = activity.create_text_view("点击次数: 0", Some(layout.id()))?;
    println!("✓ 计数器创建成功 (ID = {})\n", counter.id());
    
    println!("[7/10] 设置计数器大小...");
    counter.set_text_size(&mut activity, 24)?;
    println!("✓ 计数器大小设置成功\n");
    
    println!("[8/10] 设置计数器边距...");
    counter.view().set_margin(&mut activity, 20)?;
    println!("✓ 计数器边距设置成功\n");
    
    println!("[9/10] 创建按钮布局（横向）...");
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    println!("✓ 按钮布局创建成功 (ID = {})\n", button_layout.id());
    
    println!("[10/10] 创建按钮...");
    let inc_button = activity.create_button("➕ 增加", Some(button_layout.id()))?;
    println!("✓ 增加按钮创建成功 (ID = {})", inc_button.id());
    
    let dec_button = activity.create_button("➖ 减少", Some(button_layout.id()))?;
    println!("✓ 减少按钮创建成功 (ID = {})", dec_button.id());
    
    let reset_button = activity.create_button("🔄 重置", Some(layout.id()))?;
    println!("✓ 重置按钮创建成功 (ID = {})\n", reset_button.id());
    
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ 界面创建完成");
    println!("  - Counter ID: {}", counter.id());
    println!("  - Button ID: {}", inc_button.id());
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("请点击按钮进行测试");
    println!("关闭窗口或按 Ctrl+C 退出");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件循环
    let mut count = 0;
    
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        
        println!("[事件] type = {}", event_type);
        
        match event_type {
            "destroy" => {
                // Activity 已被系统销毁，直接退出即可
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
    
    Ok(())
}
