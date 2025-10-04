// ProgressBar 进度条演示 - 使用新库 API
// 展示如何显示和更新进度条
// 运行: cargo run --example progress_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("=== ProgressBar 进度条演示 (新库版本) ===\n");
    
    // 创建 Activity（全屏模式）
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立\n");
    
    // 创建 NestedScrollView 作为根布局（支持滚动）
    let scroll = activity.create_nested_scroll_view(None)?;
    
    // 创建主布局（放在 ScrollView 内）
    let layout = activity.create_linear_layout(Some(scroll.id()))?;
    layout.view().set_margin(&mut activity, 15)?;
    
    // 创建标题
    let title = activity.create_text_view("📊 进度条演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("点击按钮控制进度", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 ProgressBar 1 - 主进度条
    let label1 = activity.create_text_view("主任务进度:", Some(layout.id()))?;
    label1.view().set_margin(&mut activity, 10)?;
    label1.view().set_height_wrap_content(&mut activity)?;
    label1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let progress1 = activity.create_progress_bar(Some(layout.id()))?;
    progress1.view().set_margin(&mut activity, 10)?;
    progress1.view().set_width_match_parent(&mut activity)?;
    progress1.view().set_height_wrap_content(&mut activity)?;
    progress1.view().set_linear_layout_params(&mut activity, 0, None)?;
    progress1.set_progress(&mut activity, 0)?;
    
    let progress1_text = activity.create_text_view("0%", Some(layout.id()))?;
    progress1_text.view().set_margin(&mut activity, 5)?;
    progress1_text.view().set_height_wrap_content(&mut activity)?;
    progress1_text.view().set_linear_layout_params(&mut activity, 0, None)?;
    progress1_text.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建 ProgressBar 2 - 次要进度条
    let label2 = activity.create_text_view("下载进度:", Some(layout.id()))?;
    label2.view().set_margin(&mut activity, 10)?;
    label2.view().set_height_wrap_content(&mut activity)?;
    label2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let progress2 = activity.create_progress_bar(Some(layout.id()))?;
    progress2.view().set_margin(&mut activity, 10)?;
    progress2.view().set_width_match_parent(&mut activity)?;
    progress2.view().set_height_wrap_content(&mut activity)?;
    progress2.view().set_linear_layout_params(&mut activity, 0, None)?;
    progress2.set_progress(&mut activity, 0)?;
    
    let progress2_text = activity.create_text_view("0%", Some(layout.id()))?;
    progress2_text.view().set_margin(&mut activity, 5)?;
    progress2_text.view().set_height_wrap_content(&mut activity)?;
    progress2_text.view().set_linear_layout_params(&mut activity, 0, None)?;
    progress2_text.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建分隔线
    let divider = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider.view().set_margin(&mut activity, 10)?;
    divider.view().set_height_wrap_content(&mut activity)?;
    divider.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建按钮布局
    let button_layout1 = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout1.view().set_margin(&mut activity, 10)?;
    button_layout1.view().set_height_wrap_content(&mut activity)?;
    button_layout1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let inc_btn = activity.create_button("➕ 增加", Some(button_layout1.id()))?;
    inc_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let dec_btn = activity.create_button("➖ 减少", Some(button_layout1.id()))?;
    dec_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let reset_btn = activity.create_button("🔄 重置", Some(button_layout1.id()))?;
    reset_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 第二行按钮
    let button_layout2 = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout2.view().set_margin(&mut activity, 10)?;
    button_layout2.view().set_height_wrap_content(&mut activity)?;
    button_layout2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let auto_btn = activity.create_button("▶️ 自动演示", Some(button_layout2.id()))?;
    auto_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let complete_btn = activity.create_button("✅ 完成", Some(button_layout2.id()))?;
    complete_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • ➕ 增加 - 进度 +10%");
    println!("  • ➖ 减少 - 进度 -10%");
    println!("  • 🔄 重置 - 进度归零");
    println!("  • ▶️ 自动演示 - 模拟加载过程");
    println!("  • ✅ 完成 - 进度设为100%");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 进度状态
    let mut progress1_value = 0;
    let mut progress2_value = 0;
    
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
                
                if clicked_id == inc_btn.id() {
                    // 增加进度
                    progress1_value = (progress1_value + 10).min(100);
                    progress1.set_progress(&mut activity, progress1_value)?;
                    progress1_text.set_text(&mut activity, &format!("{}%", progress1_value))?;
                    println!("➕ 进度: {}%", progress1_value);
                    
                } else if clicked_id == dec_btn.id() {
                    // 减少进度
                    progress1_value = (progress1_value - 10).max(0);
                    progress1.set_progress(&mut activity, progress1_value)?;
                    progress1_text.set_text(&mut activity, &format!("{}%", progress1_value))?;
                    println!("➖ 进度: {}%", progress1_value);
                    
                } else if clicked_id == reset_btn.id() {
                    // 重置
                    progress1_value = 0;
                    progress2_value = 0;
                    progress1.set_progress(&mut activity, 0)?;
                    progress2.set_progress(&mut activity, 0)?;
                    progress1_text.set_text(&mut activity, "0%")?;
                    progress2_text.set_text(&mut activity, "0%")?;
                    progress1_text.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
                    progress2_text.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
                    println!("🔄 进度已重置");
                    
                } else if clicked_id == complete_btn.id() {
                    // 完成
                    progress1_value = 100;
                    progress2_value = 100;
                    progress1.set_progress(&mut activity, 100)?;
                    progress2.set_progress(&mut activity, 100)?;
                    progress1_text.set_text(&mut activity, "100% ✓")?;
                    progress2_text.set_text(&mut activity, "100% ✓")?;
                    progress1_text.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                    progress2_text.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                    println!("✅ 任务完成！");
                    
                } else if clicked_id == auto_btn.id() {
                    // 自动演示 - 模拟加载过程
                    println!("▶️ 开始自动演示...");
                    progress1_value = 0;
                    progress2_value = 0;
                    
                    // 模拟两个进度条同时加载
                    for i in 0..=10 {
                        progress1_value = i * 10;
                        progress2_value = i * 10;
                        
                        progress1.set_progress(&mut activity, progress1_value)?;
                        progress2.set_progress(&mut activity, progress2_value)?;
                        
                        progress1_text.set_text(&mut activity, &format!("{}%", progress1_value))?;
                        progress2_text.set_text(&mut activity, &format!("{}%", progress2_value))?;
                        
                        if i == 10 {
                            progress1_text.set_text(&mut activity, "100% ✓")?;
                            progress2_text.set_text(&mut activity, "100% ✓")?;
                            progress1_text.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                            progress2_text.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                        }
                        
                        println!("   进度: {}%", progress1_value);
                        
                        // 延迟一下，让用户看到进度变化
                        thread::sleep(Duration::from_millis(300));
                    }
                    
                    println!("✅ 自动演示完成！");
                }
            },
            _ => {}
        }
    }
}
