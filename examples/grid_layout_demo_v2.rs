// GridLayout 网格布局演示 - 使用新库 API
// 展示如何使用 GridLayout 创建网格排列
// 运行: cargo run --example grid_layout_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== GridLayout 网格布局演示 (新库版本) ===\n");
    
    // 创建 Activity（全屏模式）
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立\n");
    
    // 创建 NestedScrollView 作为根布局（支持滚动）
    let scroll = activity.create_nested_scroll_view(None)?;
    
    // 创建主布局（放在 ScrollView 内）
    let layout = activity.create_linear_layout(Some(scroll.id()))?;
    layout.view().set_margin(&mut activity, 15)?;
    
    // 创建标题
    let title = activity.create_text_view("🎯 GridLayout 演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("网格布局：行列自动排列", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 示例1: 3x3 按钮网格 ==========
    let section1 = activity.create_text_view("示例1: 3×3 按钮网格", Some(layout.id()))?;
    section1.set_text_size(&mut activity, 18)?;
    section1.view().set_margin(&mut activity, 10)?;
    section1.view().set_height_wrap_content(&mut activity)?;
    section1.view().set_linear_layout_params(&mut activity, 0, None)?;
    section1.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建 3x3 GridLayout
    let grid1 = activity.create_grid_layout(3, 3, Some(layout.id()))?;
    grid1.view().set_margin(&mut activity, 10)?;
    grid1.view().set_width_match_parent(&mut activity)?;
    grid1.view().set_height_wrap_content(&mut activity)?;
    grid1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建9个按钮（自动填充到3x3网格）
    let mut grid1_buttons = Vec::new();
    for i in 1..=9 {
        let btn = activity.create_button(&format!("{}", i), Some(grid1.id()))?;
        grid1_buttons.push(btn);
    }
    
    // ========== 示例2: 2x4 文本网格 ==========
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider1.view().set_margin(&mut activity, 10)?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section2 = activity.create_text_view("示例2: 2×4 表情网格", Some(layout.id()))?;
    section2.set_text_size(&mut activity, 18)?;
    section2.view().set_margin(&mut activity, 10)?;
    section2.view().set_height_wrap_content(&mut activity)?;
    section2.view().set_linear_layout_params(&mut activity, 0, None)?;
    section2.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 创建 2x4 GridLayout
    let grid2 = activity.create_grid_layout(2, 4, Some(layout.id()))?;
    grid2.view().set_margin(&mut activity, 10)?;
    grid2.view().set_width_match_parent(&mut activity)?;
    grid2.view().set_height_wrap_content(&mut activity)?;
    grid2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 8个表情文本（自动填充到2x4网格）
    let emojis = ["😀", "😎", "🎉", "🚀", "🌟", "❤️", "👍", "🔥"];
    for emoji in &emojis {
        let text = activity.create_text_view(emoji, Some(grid2.id()))?;
        text.set_text_size(&mut activity, 32)?;
        text.view().set_margin(&mut activity, 5)?;
    }
    
    // ========== 示例3: 4x2 混合网格 ==========
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider2.view().set_margin(&mut activity, 10)?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section3 = activity.create_text_view("示例3: 4×2 开关网格", Some(layout.id()))?;
    section3.set_text_size(&mut activity, 18)?;
    section3.view().set_margin(&mut activity, 10)?;
    section3.view().set_height_wrap_content(&mut activity)?;
    section3.view().set_linear_layout_params(&mut activity, 0, None)?;
    section3.set_text_color(&mut activity, 0xFFFF9800u32 as i32)?;
    
    // 创建 4x2 GridLayout
    let grid3 = activity.create_grid_layout(4, 2, Some(layout.id()))?;
    grid3.view().set_margin(&mut activity, 10)?;
    grid3.view().set_width_match_parent(&mut activity)?;
    grid3.view().set_height_wrap_content(&mut activity)?;
    grid3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 8个 ToggleButton（自动填充到4x2网格）
    let toggle_labels = ["WiFi", "蓝牙", "GPS", "飞行", "数据", "热点", "蓝光", "省电"];
    let mut grid3_toggles = Vec::new();
    for label in &toggle_labels {
        let toggle = activity.create_toggle_button(label, Some(grid3.id()))?;
        grid3_toggles.push(toggle);
    }
    
    // ========== 底部信息 ==========
    let divider3 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider3.view().set_margin(&mut activity, 10)?;
    divider3.view().set_height_wrap_content(&mut activity)?;
    divider3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let info = activity.create_text_view(
        "💡 GridLayout 特点:\n\
        • 指定行数和列数创建网格\n\
        • 子视图按添加顺序自动填充\n\
        • 从左到右、从上到下排列\n\
        • 适合创建规则的网格界面",
        Some(layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 状态显示
    let status = activity.create_text_view("准备就绪 - 点击任意元素试试", Some(layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 观察三个不同尺寸的网格布局");
    println!("  • 3×3 按钮网格：数字1-9");
    println!("  • 2×4 表情网格：8个表情");
    println!("  • 4×2 开关网格：8个ToggleButton");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
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
                
                // 检查是否是第一个网格的按钮
                for (i, btn) in grid1_buttons.iter().enumerate() {
                    if clicked_id == btn.id() {
                        println!("✅ 点击了数字按钮: {}", i + 1);
                        status.set_text(&mut activity, &format!("✅ 点击了数字: {}", i + 1))?;
                        status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                        break;
                    }
                }
                
                // 检查是否是第三个网格的开关
                for (i, toggle) in grid3_toggles.iter().enumerate() {
                    if clicked_id == toggle.id() {
                        let is_checked = event_value["set"].as_bool().unwrap_or(false);
                        println!("🔘 {}: {}", toggle_labels[i], if is_checked { "开启" } else { "关闭" });
                        status.set_text(&mut activity, 
                            &format!("🔘 {}: {}", toggle_labels[i], if is_checked { "开启" } else { "关闭" }))?;
                        status.set_text_color(&mut activity, 
                            if is_checked { 0xFFFF9800u32 as i32 } else { 0xFF9E9E9Eu32 as i32 })?;
                        break;
                    }
                }
            },
            _ => {}
        }
    }
}
