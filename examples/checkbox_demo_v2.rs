// Checkbox 复选框演示 - 使用新库 API
// 展示如何创建和使用 Checkbox，处理复选状态变化
// 运行: cargo run --example checkbox_demo_v2 --release

use termux_gui::{Activity, TextView, Result};
use termux_gui::connection::read_message;

// 更新状态显示的辅助函数
fn update_status_text(
    activity: &mut Activity,
    status: &TextView,
    wifi: bool,
    bt: bool,
    loc: bool,
    notif: bool
) -> Result<()> {
    let mut selected = Vec::new();
    if wifi { selected.push("WiFi"); }
    if bt { selected.push("蓝牙"); }
    if loc { selected.push("定位"); }
    if notif { selected.push("通知"); }
    
    let text = if selected.is_empty() {
        "当前选中: 无".to_string()
    } else {
        format!("当前选中: {}", selected.join(", "))
    };
    
    status.set_text(activity, &text)?;
    Ok(())
}

fn main() -> Result<()> {
    println!("=== Checkbox 复选框演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("选择你喜欢的功能 ✅", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    
    // 创建 Checkbox 1 - WiFi (初始未选中)
    let checkbox1 = activity.create_checkbox_checked("📶 WiFi", Some(layout.id()), false)?;
    checkbox1.view().set_margin(&mut activity, 5)?;
    checkbox1.view().set_height_wrap_content(&mut activity)?;
    
    // 创建 Checkbox 2 - 蓝牙 (初始选中)
    let checkbox2 = activity.create_checkbox_checked("📡 蓝牙", Some(layout.id()), true)?;
    checkbox2.view().set_margin(&mut activity, 5)?;
    checkbox2.view().set_height_wrap_content(&mut activity)?;
    
    // 创建 Checkbox 3 - 定位 (初始未选中)
    let checkbox3 = activity.create_checkbox_checked("📍 定位服务", Some(layout.id()), false)?;
    checkbox3.view().set_margin(&mut activity, 5)?;
    checkbox3.view().set_height_wrap_content(&mut activity)?;
    
    // 创建 Checkbox 4 - 通知 (初始选中)
    let checkbox4 = activity.create_checkbox_checked("🔔 通知", Some(layout.id()), true)?;
    checkbox4.view().set_margin(&mut activity, 5)?;
    checkbox4.view().set_height_wrap_content(&mut activity)?;
    
    // 创建分隔线
    let separator = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    separator.view().set_height_wrap_content(&mut activity)?;
    
    // 创建状态显示
    let status = activity.create_text_view("当前选中: 蓝牙, 通知", Some(layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建按钮
    let apply_button = activity.create_button("✅ 应用设置", Some(layout.id()))?;
    apply_button.view().set_margin(&mut activity, 10)?;
    apply_button.view().set_height_wrap_content(&mut activity)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 点击复选框切换状态");
    println!("  • 观察状态实时更新");
    println!("  • 点击 '应用设置' 查看最终选择");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 状态跟踪
    let mut wifi_checked = false;
    let mut bluetooth_checked = true;
    let mut location_checked = false;
    let mut notification_checked = true;
    
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
                let is_checked = event_value["set"].as_bool().unwrap_or(false);
                
                if clicked_id == checkbox1.id() {
                    wifi_checked = is_checked;
                    println!("📶 WiFi: {}", if is_checked { "开启" } else { "关闭" });
                    update_status_text(&mut activity, &status, 
                                     wifi_checked, bluetooth_checked, 
                                     location_checked, notification_checked)?;
                    
                } else if clicked_id == checkbox2.id() {
                    bluetooth_checked = is_checked;
                    println!("📡 蓝牙: {}", if is_checked { "开启" } else { "关闭" });
                    update_status_text(&mut activity, &status, 
                                     wifi_checked, bluetooth_checked, 
                                     location_checked, notification_checked)?;
                    
                } else if clicked_id == checkbox3.id() {
                    location_checked = is_checked;
                    println!("📍 定位: {}", if is_checked { "开启" } else { "关闭" });
                    update_status_text(&mut activity, &status, 
                                     wifi_checked, bluetooth_checked, 
                                     location_checked, notification_checked)?;
                    
                } else if clicked_id == checkbox4.id() {
                    notification_checked = is_checked;
                    println!("🔔 通知: {}", if is_checked { "开启" } else { "关闭" });
                    update_status_text(&mut activity, &status, 
                                     wifi_checked, bluetooth_checked, 
                                     location_checked, notification_checked)?;
                    
                } else if clicked_id == apply_button.id() {
                    println!("\n✅ 应用设置:");
                    println!("  WiFi: {}", if wifi_checked { "✓" } else { "✗" });
                    println!("  蓝牙: {}", if bluetooth_checked { "✓" } else { "✗" });
                    println!("  定位: {}", if location_checked { "✓" } else { "✗" });
                    println!("  通知: {}", if notification_checked { "✓" } else { "✗" });
                    
                    // 显示确认消息
                    status.set_text(&mut activity, "✅ 设置已应用！")?;
                    status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                }
            },
            _ => {}
        }
    }
}
