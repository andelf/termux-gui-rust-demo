// 输入框交互演示 - 使用新库 API
// 展示 EditText 输入、Button 点击、TextView 显示
// 运行: cargo run --example input_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== 输入框交互演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("文本输入演示 📝", Some(layout.id()))?;
    title.set_text_size(&mut activity, 28)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明文本
    let desc1 = activity.create_text_view("在下方输入你的名字:", Some(layout.id()))?;
    desc1.view().set_margin(&mut activity, 5)?;
    desc1.view().set_height_wrap_content(&mut activity)?;
    desc1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建输入框 1 - 姓名（单行文本）
    let name_input = activity.create_edit_text("", Some(layout.id()))?;
    name_input.set_hint(&mut activity, "请输入姓名")?;
    name_input.view().set_margin(&mut activity, 5)?;
    name_input.view().set_height_wrap_content(&mut activity)?;
    name_input.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明文本2
    let desc2 = activity.create_text_view("输入一个数字:", Some(layout.id()))?;
    desc2.view().set_margin(&mut activity, 5)?;
    desc2.view().set_height_wrap_content(&mut activity)?;
    desc2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建输入框 2 - 数字
    let number_input = activity.create_edit_text("0", Some(layout.id()))?;
    number_input.set_hint(&mut activity, "请输入数字")?;
    number_input.view().set_margin(&mut activity, 5)?;
    number_input.view().set_height_wrap_content(&mut activity)?;
    number_input.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明文本3
    let desc3 = activity.create_text_view("输入多行消息:", Some(layout.id()))?;
    desc3.view().set_margin(&mut activity, 5)?;
    desc3.view().set_height_wrap_content(&mut activity)?;
    desc3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建输入框 3 - 多行文本
    let message_input = activity.create_edit_text_multiline("", Some(layout.id()))?;
    message_input.set_hint(&mut activity, "请输入消息（支持多行）")?;
    message_input.view().set_margin(&mut activity, 5)?;
    // 多行文本需要更多空间
    message_input.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建按钮布局（水平）
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout.view().set_margin(&mut activity, 10)?;
    button_layout.view().set_height_wrap_content(&mut activity)?;
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建提交按钮
    let submit_button = activity.create_button("✅ 提交", Some(button_layout.id()))?;
    submit_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建清空按钮
    let clear_button = activity.create_button("🗑️ 清空", Some(button_layout.id()))?;
    clear_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建测试按钮
    let test_button = activity.create_button("🧪 测试", Some(button_layout.id()))?;
    test_button.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建分隔线
    let divider = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider.view().set_margin(&mut activity, 10)?;
    divider.view().set_height_wrap_content(&mut activity)?;
    divider.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建结果显示区域
    let result = activity.create_text_view("结果将显示在这里...", Some(layout.id()))?;
    result.set_text_size(&mut activity, 16)?;
    result.view().set_margin(&mut activity, 10)?;
    result.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 在输入框中输入内容");
    println!("  • 点击 '提交' 查看输入的内容");
    println!("  • 点击 '清空' 清除所有输入");
    println!("  • 点击 '测试' 填充测试数据");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件循环
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                // Activity 已被系统销毁，直接退出即可
                // 不要调用 activity.finish()，因为 Activity 已经不存在了
                println!("\n✓ Activity 已关闭");
                return Ok(());
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == submit_button.id() {
                    println!("📨 提交按钮被点击");
                    
                    // 获取姓名输入框的文本
                    let name = name_input.get_text(&mut activity)?;
                    
                    // 获取数字输入框的文本
                    let number_str = number_input.get_text(&mut activity)?;
                    let number: i32 = number_str.parse().unwrap_or(0);
                    
                    // 获取多行消息
                    let message = message_input.get_text(&mut activity)?;
                    
                    // 构建结果文本
                    let result_text = if name.is_empty() {
                        "⚠️ 请输入姓名！".to_string()
                    } else {
                        let doubled = number * 2;
                        let msg_part = if message.is_empty() {
                            String::new()
                        } else {
                            format!("\n\n消息:\n{}", message)
                        };
                        
                        format!(
                            "✅ 提交成功！\n\n\
                            姓名: {}\n\
                            数字: {} (x2 = {}){}",
                            name, number, doubled, msg_part
                        )
                    };
                    
                    // 更新结果显示
                    result.set_text(&mut activity, &result_text)?;
                    
                    // 根据结果改变颜色
                    let color = if name.is_empty() {
                        0xFFF44336u32 as i32  // 红色（错误）
                    } else {
                        0xFF4CAF50u32 as i32  // 绿色（成功）
                    };
                    result.set_text_color(&mut activity, color)?;
                    
                    println!("   姓名: {}", name);
                    println!("   数字: {}", number);
                    println!("   消息: {}", if message.is_empty() { "(空)" } else { &message });
                    
                } else if clicked_id == clear_button.id() {
                    println!("🗑️ 清空按钮被点击");
                    
                    // 清空所有输入框
                    name_input.set_text(&mut activity, "")?;
                    number_input.set_text(&mut activity, "0")?;
                    message_input.set_text(&mut activity, "")?;
                    
                    // 清空结果显示
                    result.set_text(&mut activity, "已清空所有输入")?;
                    result.set_text_color(&mut activity, 0xFF9E9E9Eu32 as i32)?;
                    
                } else if clicked_id == test_button.id() {
                    println!("🧪 测试按钮被点击");
                    
                    // 填充测试数据
                    name_input.set_text(&mut activity, "张三")?;
                    number_input.set_text(&mut activity, "42")?;
                    message_input.set_text(&mut activity, "这是一条测试消息。\n使用Rust编写的Termux:GUI应用！")?;
                    
                    result.set_text(&mut activity, "✅ 已填充测试数据\n点击 '提交' 按钮查看结果")?;
                    result.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
                }
            },
            "text" => {
                // EditText 文本变化事件（可选）
                let view_id = event_value["id"].as_i64().unwrap_or(-1);
                let text = event_value["text"].as_str().unwrap_or("");
                
                if view_id == name_input.id() {
                    println!("📝 姓名输入框内容变化: {}", text);
                } else if view_id == number_input.id() {
                    println!("🔢 数字输入框内容变化: {}", text);
                }
            },
            _ => {}
        }
    }
}
