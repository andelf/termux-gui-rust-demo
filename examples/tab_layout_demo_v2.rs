//! TabLayout 标签页演示 (新库版本)
//! 
//! 展示如何使用 TabLayout 创建标签页界面
//! 点击标签页切换内容显示

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== TabLayout 标签页演示 (新库版本) ===\n");

    // 创建连接和 Activity
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立");

    // 创建根布局（垂直）
    let root = activity.create_linear_layout(None)?;
    
    // 创建 TabLayout（在顶部）
    let tabs = activity.create_tab_layout(Some(root.id()))?;
    tabs.view().set_linear_layout_params(&mut activity, 0, None)?;
    tabs.view().set_height(&mut activity, -2)?; // WRAP_CONTENT
    
    // 设置标签列表
    tabs.set_list(&mut activity, &["首页", "消息", "我的"])?;
    
    // 创建内容区域
    let content_area = activity.create_linear_layout(Some(root.id()))?;
    content_area.view().set_linear_layout_params(&mut activity, 1, None)?; // weight=1 占据剩余空间
    
    // 创建三个页面的内容（初始全部隐藏）
    // 页面1 - 首页
    let page1 = activity.create_linear_layout(Some(content_area.id()))?;
    
    let title1 = activity.create_text_view("📱 首页", Some(page1.id()))?;
    title1.set_text_size(&mut activity, 28)?;
    
    let content1 = activity.create_text_view("\n欢迎使用 TabLayout！\n\n这是首页内容。\n\n☝️ 点击顶部标签切换页面", Some(page1.id()))?;
    content1.set_text_size(&mut activity, 18)?;
    
    // 页面2 - 消息
    let page2 = activity.create_linear_layout(Some(content_area.id()))?;
    activity.send(&serde_json::json!({
        "method": "setVisibility",
        "params": {
            "aid": activity.id(),
            "id": page2.id(),
            "vis": 8  // GONE initially
        }
    }))?;
    
    let title2 = activity.create_text_view("💬 消息中心", Some(page2.id()))?;
    title2.set_text_size(&mut activity, 28)?;
    
    let content2 = activity.create_text_view("\n这是第二页\n\n你有 3 条新消息\n\n• 系统通知\n• 好友消息\n• 更新提醒", Some(page2.id()))?;
    content2.set_text_size(&mut activity, 18)?;
    
    // 页面3 - 我的
    let page3 = activity.create_linear_layout(Some(content_area.id()))?;
    activity.send(&serde_json::json!({
        "method": "setVisibility",
        "params": {
            "aid": activity.id(),
            "id": page3.id(),
            "vis": 8  // GONE initially
        }
    }))?;
    
    let title3 = activity.create_text_view("👤 个人中心", Some(page3.id()))?;
    title3.set_text_size(&mut activity, 28)?;
    
    let content3 = activity.create_text_view("\n这是第三页\n\n个人信息\n\n• 账号设置\n• 隐私设置\n• 关于我们", Some(page3.id()))?;
    content3.set_text_size(&mut activity, 18)?;
    
    println!("\n✓ 界面创建完成");
    println!("\n━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 点击顶部标签切换页面");
    println!("  • 观察页面内容变化");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 当前选中的标签页
    let mut current_tab = 0;
    let pages = [page1.id(), page2.id(), page3.id()];
    
    // 事件循环
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                return Ok(());
            }
            "itemselected" => {
                // TabLayout 被点击
                if let Some(selected) = event["value"]["selected"].as_i64() {
                    if event["value"]["id"].as_i64() == Some(tabs.id()) {
                        let new_tab = selected as usize;
                        if new_tab != current_tab && new_tab < 3 {
                            println!("切换到标签 {}: {}", new_tab, ["首页", "消息", "我的"][new_tab]);
                            
                            // 隐藏当前页面
                            activity.send(&serde_json::json!({
                                "method": "setVisibility",
                                "params": {
                                    "aid": activity.id(),
                                    "id": pages[current_tab],
                                    "vis": 8  // GONE
                                }
                            }))?;
                            
                            // 显示新页面
                            activity.send(&serde_json::json!({
                                "method": "setVisibility",
                                "params": {
                                    "aid": activity.id(),
                                    "id": pages[new_tab],
                                    "vis": 0  // VISIBLE
                                }
                            }))?;
                            
                            current_tab = new_tab;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
