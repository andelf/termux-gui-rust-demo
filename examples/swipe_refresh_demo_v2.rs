// SwipeRefreshLayout 下拉刷新演示 - 使用新库 API
// 展示如何使用 SwipeRefreshLayout 实现下拉刷新
// 运行: cargo run --example swipe_refresh_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("=== SwipeRefreshLayout 下拉刷新演示 (新库版本) ===\n");
    
    // 创建 Activity（全屏模式）
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立\n");
    
    // 创建 SwipeRefreshLayout 作为根视图
    let swipe_refresh = activity.create_swipe_refresh_layout(None)?;
    
    // 在 SwipeRefreshLayout 内创建 LinearLayout（只能有一个子视图）
    let layout = activity.create_linear_layout(Some(swipe_refresh.id()))?;
    // ⚠️ 重要：不要在SwipeRefreshLayout的直接子View上设置margin，会导致termux-gui崩溃！
    // 可以在LinearLayout的子View上设置margin
    
    // 标题
    let title = activity.create_text_view("🔄 下拉刷新演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 说明
    let desc = activity.create_text_view("向下拉动页面触发刷新", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 分隔线
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider1.view().set_margin(&mut activity, 10)?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 刷新计数器
    let refresh_count = activity.create_text_view("刷新次数: 0", Some(layout.id()))?;
    refresh_count.set_text_size(&mut activity, 24)?;
    refresh_count.view().set_margin(&mut activity, 10)?;
    refresh_count.view().set_height_wrap_content(&mut activity)?;
    refresh_count.view().set_linear_layout_params(&mut activity, 0, None)?;
    refresh_count.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 最后刷新时间
    let last_time = activity.create_text_view("最后刷新: 尚未刷新", Some(layout.id()))?;
    last_time.view().set_margin(&mut activity, 10)?;
    last_time.view().set_height_wrap_content(&mut activity)?;
    last_time.view().set_linear_layout_params(&mut activity, 0, None)?;
    last_time.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 分隔线
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider2.view().set_margin(&mut activity, 10)?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 内容区域标题
    let content_title = activity.create_text_view("📋 内容列表", Some(layout.id()))?;
    content_title.set_text_size(&mut activity, 18)?;
    content_title.view().set_margin(&mut activity, 10)?;
    content_title.view().set_height_wrap_content(&mut activity)?;
    content_title.view().set_linear_layout_params(&mut activity, 0, None)?;
    content_title.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 内容项
    let item1 = activity.create_text_view("📄 项目 1", Some(layout.id()))?;
    item1.view().set_margin(&mut activity, 8)?;
    item1.view().set_height_wrap_content(&mut activity)?;
    item1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let item2 = activity.create_text_view("📄 项目 2", Some(layout.id()))?;
    item2.view().set_margin(&mut activity, 8)?;
    item2.view().set_height_wrap_content(&mut activity)?;
    item2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let item3 = activity.create_text_view("📄 项目 3", Some(layout.id()))?;
    item3.view().set_margin(&mut activity, 8)?;
    item3.view().set_height_wrap_content(&mut activity)?;
    item3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 分隔线
    let divider3 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider3.view().set_margin(&mut activity, 10)?;
    divider3.view().set_height_wrap_content(&mut activity)?;
    divider3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 提示信息
    let info = activity.create_text_view(
        "💡 向下拉动页面刷新\n或点击下方按钮手动刷新",
        Some(layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF999999u32 as i32)?;
    
    // 手动刷新按钮
    let manual_btn = activity.create_button("🔄 手动刷新", Some(layout.id()))?;
    manual_btn.view().set_margin(&mut activity, 10)?;
    manual_btn.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 向下拉动页面触发刷新");
    println!("  • 或点击按钮手动刷新");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let mut refresh_counter = 0;
    
    // 事件循环
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                return Ok(());
            },
            "refresh" => {
                println!("🔄 下拉刷新触发！");
                
                refresh_counter += 1;
                refresh_count.set_text(&mut activity, &format!("刷新次数: {}", refresh_counter))?;
                
                // 更新时间
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                last_time.set_text(&mut activity, &format!("最后刷新: {} 秒", now % 10000))?;
                
                // 更新内容项
                item1.set_text(&mut activity, &format!("📄 项目 1 (刷新 #{})", refresh_counter))?;
                item2.set_text(&mut activity, &format!("📄 项目 2 (刷新 #{})", refresh_counter))?;
                item3.set_text(&mut activity, &format!("📄 项目 3 (刷新 #{})", refresh_counter))?;
                
                println!("⏳ 模拟加载中...");
                thread::sleep(Duration::from_secs(2));
                
                swipe_refresh.set_refreshing(&mut activity, false)?;
                println!("✅ 刷新完成！\n");
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == manual_btn.id() {
                    println!("🔄 手动刷新触发！");
                    
                    swipe_refresh.set_refreshing(&mut activity, true)?;
                    
                    refresh_counter += 1;
                    refresh_count.set_text(&mut activity, &format!("刷新次数: {}", refresh_counter))?;
                    
                    item1.set_text(&mut activity, &format!("📄 项目 1 (刷新 #{})", refresh_counter))?;
                    item2.set_text(&mut activity, &format!("📄 项目 2 (刷新 #{})", refresh_counter))?;
                    item3.set_text(&mut activity, &format!("📄 项目 3 (刷新 #{})", refresh_counter))?;
                    
                    println!("⏳ 模拟加载中...");
                    thread::sleep(Duration::from_secs(2));
                    
                    swipe_refresh.set_refreshing(&mut activity, false)?;
                    println!("✅ 刷新完成！\n");
                }
            },
            _ => {}
        }
    }
}
