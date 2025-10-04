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
    layout.view().set_margin(&mut activity, 15)?;
    
    // 创建标题
    let title = activity.create_text_view("🔄 下拉刷新演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("向下拉动页面可以刷新内容", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 分隔线
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider1.view().set_margin(&mut activity, 10)?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 刷新计数器
    let refresh_label = activity.create_text_view("刷新次数:", Some(layout.id()))?;
    refresh_label.set_text_size(&mut activity, 18)?;
    refresh_label.view().set_margin(&mut activity, 10)?;
    refresh_label.view().set_height_wrap_content(&mut activity)?;
    refresh_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    refresh_label.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    let refresh_count = activity.create_text_view("0 次", Some(layout.id()))?;
    refresh_count.set_text_size(&mut activity, 32)?;
    refresh_count.view().set_margin(&mut activity, 10)?;
    refresh_count.view().set_height_wrap_content(&mut activity)?;
    refresh_count.view().set_linear_layout_params(&mut activity, 0, None)?;
    refresh_count.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 分隔线
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider2.view().set_margin(&mut activity, 10)?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 最后刷新时间
    let time_label = activity.create_text_view("最后刷新:", Some(layout.id()))?;
    time_label.set_text_size(&mut activity, 16)?;
    time_label.view().set_margin(&mut activity, 10)?;
    time_label.view().set_height_wrap_content(&mut activity)?;
    time_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let last_refresh_time = activity.create_text_view("尚未刷新", Some(layout.id()))?;
    last_refresh_time.view().set_margin(&mut activity, 5)?;
    last_refresh_time.view().set_height_wrap_content(&mut activity)?;
    last_refresh_time.view().set_linear_layout_params(&mut activity, 0, None)?;
    last_refresh_time.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 分隔线
    let divider3 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider3.view().set_margin(&mut activity, 10)?;
    divider3.view().set_height_wrap_content(&mut activity)?;
    divider3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 内容列表
    let content_label = activity.create_text_view("内容列表:", Some(layout.id()))?;
    content_label.set_text_size(&mut activity, 18)?;
    content_label.view().set_margin(&mut activity, 10)?;
    content_label.view().set_height_wrap_content(&mut activity)?;
    content_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    content_label.set_text_color(&mut activity, 0xFFFF9800u32 as i32)?;
    
    // 添加一些内容项
    let mut content_items = Vec::new();
    for i in 1..=5 {
        let item = activity.create_text_view(&format!("📄 项目 {}", i), Some(layout.id()))?;
        item.set_text_size(&mut activity, 16)?;
        item.view().set_margin(&mut activity, 8)?;
        item.view().set_height_wrap_content(&mut activity)?;
        item.view().set_linear_layout_params(&mut activity, 0, None)?;
        content_items.push(item);
    }
    
    // 分隔线
    let divider4 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider4.view().set_margin(&mut activity, 10)?;
    divider4.view().set_height_wrap_content(&mut activity)?;
    divider4.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 提示信息
    let info = activity.create_text_view(
        "💡 使用方法:\n\
        • 在屏幕顶部向下拉动\n\
        • 看到刷新图标后松手\n\
        • 等待刷新动画完成\n\
        • 刷新计数会增加",
        Some(layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 手动触发按钮
    let manual_btn = activity.create_button("🔄 手动刷新", Some(layout.id()))?;
    manual_btn.view().set_margin(&mut activity, 10)?;
    manual_btn.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 向下拉动页面触发刷新");
    println!("  • 或点击按钮手动刷新");
    println!("  • 刷新会模拟2秒的加载时间");
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
                // 用户触发了下拉刷新
                println!("🔄 下拉刷新触发！");
                
                refresh_counter += 1;
                refresh_count.set_text(&mut activity, &format!("{} 次", refresh_counter))?;
                
                // 获取当前时间（简单格式）
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let time_str = format!("时间戳: {}", now);
                last_refresh_time.set_text(&mut activity, &time_str)?;
                
                // 更新内容项
                for (i, item) in content_items.iter().enumerate() {
                    item.set_text(&mut activity, &format!("📄 项目 {} (刷新 #{})", i + 1, refresh_counter))?;
                }
                
                println!("⏳ 模拟加载中...");
                
                // 模拟网络请求或数据加载（2秒）
                thread::sleep(Duration::from_secs(2));
                
                // 刷新完成，停止动画
                swipe_refresh.set_refreshing(&mut activity, false)?;
                println!("✅ 刷新完成！\n");
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == manual_btn.id() {
                    println!("🔄 手动刷新触发！");
                    
                    // 显示刷新动画
                    swipe_refresh.set_refreshing(&mut activity, true)?;
                    
                    refresh_counter += 1;
                    refresh_count.set_text(&mut activity, &format!("{} 次", refresh_counter))?;
                    
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let time_str = format!("时间戳: {}", now);
                    last_refresh_time.set_text(&mut activity, &time_str)?;
                    
                    for (i, item) in content_items.iter().enumerate() {
                        item.set_text(&mut activity, &format!("📄 项目 {} (刷新 #{})", i + 1, refresh_counter))?;
                    }
                    
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
