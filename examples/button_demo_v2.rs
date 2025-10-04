// Button 交互式演示 - 使用新库 API
// 展示如何创建按钮、布局和处理点击事件
// 运行: cargo run --example button_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Button 交互演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("计数器演示 🦀", Some(layout.id()))?;
    title.set_text_size(&mut activity, 30)?;
    title.view().set_margin(&mut activity, 10)?;
    // 设置标题高度为WRAP_CONTENT，避免占用太多空间
    title.view().set_height_wrap_content(&mut activity)?;
    // 设置布局权重为0，不占用额外空间
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建计数显示
    let counter = activity.create_text_view("点击次数: 0", Some(layout.id()))?;
    counter.set_text_size(&mut activity, 24)?;
    counter.view().set_margin(&mut activity, 20)?;
    // 设置计数器高度为WRAP_CONTENT
    counter.view().set_height_wrap_content(&mut activity)?;
    // 给计数器更高的权重，让它获得更多空间
    counter.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建按钮布局（横向）
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    // 按钮布局也使用WRAP_CONTENT
    button_layout.view().set_height_wrap_content(&mut activity)?;
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建按钮
    let inc_button = activity.create_button("➕ 增加", Some(button_layout.id()))?;
    inc_button.view().set_linear_layout_params(&mut activity, 1, None)?;  // 均分空间
    
    let dec_button = activity.create_button("➖ 减少", Some(button_layout.id()))?;
    dec_button.view().set_linear_layout_params(&mut activity, 1, None)?;  // 均分空间
    
    let reset_button = activity.create_button("🔄 重置", Some(layout.id()))?;
    reset_button.view().set_height_wrap_content(&mut activity)?;
    reset_button.view().set_linear_layout_params(&mut activity, 0, None)?;
    
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
                // Activity 已被系统销毁，直接退出即可
                // 不要调用 activity.finish()，因为 Activity 已经不存在了
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
