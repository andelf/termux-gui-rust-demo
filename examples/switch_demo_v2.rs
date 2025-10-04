// Switch 开关组件演示 - 使用新库 API
// 展示如何创建和使用 Switch（滑动开关）
// 运行: cargo run --example switch_demo_v2 --release

use termux_gui::{Activity, TextView, Result};
use termux_gui::connection::read_message;

// 更新状态显示的辅助函数
fn update_status(
    activity: &mut Activity,
    status: &TextView,
    light: bool,
    ac: bool,
    hum: bool,
    curt: bool,
    music: bool
) -> Result<()> {
    let mut active = Vec::new();
    if light { active.push("客厅灯"); }
    if ac { active.push("空调"); }
    if hum { active.push("加湿器"); }
    if curt { active.push("窗帘"); }
    if music { active.push("音乐"); }
    
    let text = if active.is_empty() {
        "已开启: 无".to_string()
    } else {
        format!("已开启: {}", active.join(", "))
    };
    
    let count = active.len();
    let color = if count == 0 {
        0xFF9E9E9Eu32 as i32  // 灰色
    } else if count >= 4 {
        0xFFF44336u32 as i32  // 红色（能耗高）
    } else if count >= 2 {
        0xFFFF9800u32 as i32  // 橙色
    } else {
        0xFF4CAF50u32 as i32  // 绿色
    };
    
    status.set_text(activity, &text)?;
    status.set_text_color(activity, color)?;
    
    Ok(())
}

fn main() -> Result<()> {
    println!("=== Switch 开关演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("智能家居控制 🏠", Some(layout.id()))?;
    title.set_text_size(&mut activity, 28)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("滑动开关控制设备", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 Switch 1 - 客厅灯（初始开启）
    let switch1 = activity.create_switch_checked("💡 客厅灯", Some(layout.id()), true)?;
    switch1.view().set_margin(&mut activity, 8)?;
    switch1.view().set_height_wrap_content(&mut activity)?;
    switch1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 Switch 2 - 空调（初始关闭）
    let switch2 = activity.create_switch_checked("❄️ 空调", Some(layout.id()), false)?;
    switch2.view().set_margin(&mut activity, 8)?;
    switch2.view().set_height_wrap_content(&mut activity)?;
    switch2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 Switch 3 - 加湿器（初始关闭）
    let switch3 = activity.create_switch_checked("💧 加湿器", Some(layout.id()), false)?;
    switch3.view().set_margin(&mut activity, 8)?;
    switch3.view().set_height_wrap_content(&mut activity)?;
    switch3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 Switch 4 - 窗帘（初始开启）
    let switch4 = activity.create_switch_checked("🪟 电动窗帘", Some(layout.id()), true)?;
    switch4.view().set_margin(&mut activity, 8)?;
    switch4.view().set_height_wrap_content(&mut activity)?;
    switch4.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 Switch 5 - 音乐（初始关闭）
    let switch5 = activity.create_switch_checked("🎵 背景音乐", Some(layout.id()), false)?;
    switch5.view().set_margin(&mut activity, 8)?;
    switch5.view().set_height_wrap_content(&mut activity)?;
    switch5.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建分隔线
    let divider = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider.view().set_margin(&mut activity, 10)?;
    divider.view().set_height_wrap_content(&mut activity)?;
    divider.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建状态显示
    let status = activity.create_text_view("已开启: 客厅灯, 窗帘", Some(layout.id()))?;
    status.set_text_size(&mut activity, 16)?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 创建按钮布局（水平）
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout.view().set_margin(&mut activity, 10)?;
    button_layout.view().set_height_wrap_content(&mut activity)?;
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建全部开启按钮
    let all_on_btn = activity.create_button("🔆 全部开启", Some(button_layout.id()))?;
    all_on_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建全部关闭按钮
    let all_off_btn = activity.create_button("🌙 全部关闭", Some(button_layout.id()))?;
    all_off_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 滑动开关切换设备状态");
    println!("  • 观察状态实时更新");
    println!("  • 使用 '全部开启/关闭' 按钮");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 状态跟踪
    let mut light_on = true;
    let mut ac_on = false;
    let mut humidifier_on = false;
    let mut curtain_on = true;
    let mut music_on = false;
    
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
                
                if clicked_id == switch1.id() {
                    light_on = is_checked;
                    println!("💡 客厅灯: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == switch2.id() {
                    ac_on = is_checked;
                    println!("❄️ 空调: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == switch3.id() {
                    humidifier_on = is_checked;
                    println!("💧 加湿器: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == switch4.id() {
                    curtain_on = is_checked;
                    println!("🪟 窗帘: {}", if is_checked { "打开" } else { "关闭" });
                } else if clicked_id == switch5.id() {
                    music_on = is_checked;
                    println!("🎵 音乐: {}", if is_checked { "播放" } else { "停止" });
                } else if clicked_id == all_on_btn.id() {
                    println!("\n🔆 全部开启");
                    light_on = true;
                    ac_on = true;
                    humidifier_on = true;
                    curtain_on = true;
                    music_on = true;
                    
                    switch1.set_checked(&mut activity, true)?;
                    switch2.set_checked(&mut activity, true)?;
                    switch3.set_checked(&mut activity, true)?;
                    switch4.set_checked(&mut activity, true)?;
                    switch5.set_checked(&mut activity, true)?;
                } else if clicked_id == all_off_btn.id() {
                    println!("\n🌙 全部关闭");
                    light_on = false;
                    ac_on = false;
                    humidifier_on = false;
                    curtain_on = false;
                    music_on = false;
                    
                    switch1.set_checked(&mut activity, false)?;
                    switch2.set_checked(&mut activity, false)?;
                    switch3.set_checked(&mut activity, false)?;
                    switch4.set_checked(&mut activity, false)?;
                    switch5.set_checked(&mut activity, false)?;
                }
                
                update_status(&mut activity, &status, 
                            light_on, ac_on, humidifier_on, curtain_on, music_on)?;
            },
            _ => {}
        }
    }
}
