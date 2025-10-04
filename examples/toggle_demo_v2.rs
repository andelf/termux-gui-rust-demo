// ToggleButton 切换按钮演示 - 使用新库 API
// 展示如何使用 ToggleButton（类似Switch但样式不同）
// 运行: cargo run --example toggle_demo_v2 --release

use termux_gui::{Activity, TextView, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== ToggleButton 切换按钮演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("🔘 ToggleButton 演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("ToggleButton 是带开关状态的按钮", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 ToggleButton 1 - WiFi（关闭）
    let toggle1 = activity.create_toggle_button_checked("📶 WiFi", Some(layout.id()), false)?;
    toggle1.view().set_margin(&mut activity, 8)?;
    toggle1.view().set_height_wrap_content(&mut activity)?;
    toggle1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 ToggleButton 2 - 蓝牙（开启）
    let toggle2 = activity.create_toggle_button_checked("📡 蓝牙", Some(layout.id()), true)?;
    toggle2.view().set_margin(&mut activity, 8)?;
    toggle2.view().set_height_wrap_content(&mut activity)?;
    toggle2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 ToggleButton 3 - GPS（关闭）
    let toggle3 = activity.create_toggle_button_checked("🛰️ GPS", Some(layout.id()), false)?;
    toggle3.view().set_margin(&mut activity, 8)?;
    toggle3.view().set_height_wrap_content(&mut activity)?;
    toggle3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 ToggleButton 4 - 飞行模式（关闭）
    let toggle4 = activity.create_toggle_button_checked("✈️ 飞行模式", Some(layout.id()), false)?;
    toggle4.view().set_margin(&mut activity, 8)?;
    toggle4.view().set_height_wrap_content(&mut activity)?;
    toggle4.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建分隔线
    let divider = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider.view().set_margin(&mut activity, 10)?;
    divider.view().set_height_wrap_content(&mut activity)?;
    divider.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建状态显示
    let status = activity.create_text_view("当前开启: 蓝牙", Some(layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建按钮布局
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout.view().set_margin(&mut activity, 10)?;
    button_layout.view().set_height_wrap_content(&mut activity)?;
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let all_on_btn = activity.create_button("🔆 全部开启", Some(button_layout.id()))?;
    all_on_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let all_off_btn = activity.create_button("🌙 全部关闭", Some(button_layout.id()))?;
    all_off_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 点击 ToggleButton 切换状态");
    println!("  • ToggleButton 开启时按钮会凹陷");
    println!("  • 使用按钮控制全部开关");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 状态跟踪
    let mut wifi_on = false;
    let mut bluetooth_on = true;
    let mut gps_on = false;
    let mut airplane_on = false;
    
    // 更新状态显示的辅助函数
    let update_status = |activity: &mut Activity, 
                         status: &TextView,
                         wifi: bool, 
                         bt: bool, 
                         gps: bool, 
                         airplane: bool| -> Result<()> {
        let mut active = Vec::new();
        if wifi { active.push("WiFi"); }
        if bt { active.push("蓝牙"); }
        if gps { active.push("GPS"); }
        if airplane { active.push("飞行模式"); }
        
        let text = if active.is_empty() {
            "当前开启: 无".to_string()
        } else {
            format!("当前开启: {}", active.join(", "))
        };
        
        status.set_text(activity, &text)?;
        
        // 如果开启飞行模式，文字显示橙色警告
        let color = if airplane {
            0xFFFF9800u32 as i32  // 橙色
        } else if active.is_empty() {
            0xFF9E9E9Eu32 as i32  // 灰色
        } else {
            0xFF2196F3u32 as i32  // 蓝色
        };
        status.set_text_color(activity, color)?;
        
        Ok(())
    };
    
    // 事件循环
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                // Activity 已被系统销毁，直接退出即可
                println!("\n✓ Activity 已关闭");
                return Ok(());
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                let is_checked = event_value["set"].as_bool().unwrap_or(false);
                
                if clicked_id == toggle1.id() {
                    wifi_on = is_checked;
                    println!("📶 WiFi: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == toggle2.id() {
                    bluetooth_on = is_checked;
                    println!("📡 蓝牙: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == toggle3.id() {
                    gps_on = is_checked;
                    println!("🛰️ GPS: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == toggle4.id() {
                    airplane_on = is_checked;
                    println!("✈️ 飞行模式: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == all_on_btn.id() {
                    println!("\n🔆 全部开启");
                    wifi_on = true;
                    bluetooth_on = true;
                    gps_on = true;
                    airplane_on = true;
                    
                    toggle1.set_checked(&mut activity, true)?;
                    toggle2.set_checked(&mut activity, true)?;
                    toggle3.set_checked(&mut activity, true)?;
                    toggle4.set_checked(&mut activity, true)?;
                } else if clicked_id == all_off_btn.id() {
                    println!("\n🌙 全部关闭");
                    wifi_on = false;
                    bluetooth_on = false;
                    gps_on = false;
                    airplane_on = false;
                    
                    toggle1.set_checked(&mut activity, false)?;
                    toggle2.set_checked(&mut activity, false)?;
                    toggle3.set_checked(&mut activity, false)?;
                    toggle4.set_checked(&mut activity, false)?;
                }
                
                update_status(&mut activity, &status, wifi_on, bluetooth_on, gps_on, airplane_on)?;
            },
            _ => {}
        }
    }
}
