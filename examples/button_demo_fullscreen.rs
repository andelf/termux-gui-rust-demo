// Button 交互式演示 - 全屏版本
// 测试是否Dialog尺寸问题导致控件不可见

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Button 演示 (全屏版本) ===\n");
    
    // 创建 Activity（全屏模式 - dialog=false）
    let mut activity = Activity::new(false)?;
    println!("✓ Activity 创建 (全屏)\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("计数器演示 🦀", Some(layout.id()))?;
    title.set_text_size(&mut activity, 30)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建计数显示
    let counter = activity.create_text_view("点击次数: 0", Some(layout.id()))?;
    counter.set_text_size(&mut activity, 24)?;
    counter.view().set_height_wrap_content(&mut activity)?;
    counter.view().set_linear_layout_params(&mut activity, 1, None)?;  // 占据主要空间
    
    // 创建按钮布局（横向）
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout.view().set_height_wrap_content(&mut activity)?;
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建按钮
    let inc_button = activity.create_button("➕ 增加", Some(button_layout.id()))?;
    inc_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let dec_button = activity.create_button("➖ 减少", Some(button_layout.id()))?;
    dec_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let reset_button = activity.create_button("🔄 重置", Some(layout.id()))?;
    reset_button.view().set_height_wrap_content(&mut activity)?;
    reset_button.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    println!("✓ 界面创建完成");
    println!("  - Counter ID: {}", counter.id());
    println!("  - Button IDs: {}, {}, {}", inc_button.id(), dec_button.id(), reset_button.id());
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
}
