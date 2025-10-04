// Space 空白间隔演示 - 使用新库 API
// 展示如何使用 Space 创建布局间隔
// 运行: cargo run --example space_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Space 空白间隔演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("📐 Space 间隔演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 示例1: 固定高度的间隔 ==========
    let section1 = activity.create_text_view("示例1: 固定高度间隔", Some(layout.id()))?;
    section1.set_text_size(&mut activity, 18)?;
    section1.view().set_margin(&mut activity, 10)?;
    section1.view().set_height_wrap_content(&mut activity)?;
    section1.view().set_linear_layout_params(&mut activity, 0, None)?;
    section1.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    let text1a = activity.create_text_view("这是第一个元素", Some(layout.id()))?;
    text1a.view().set_margin(&mut activity, 5)?;
    text1a.view().set_height_wrap_content(&mut activity)?;
    text1a.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建一个 50dp 高度的 Space
    let space1 = activity.create_space(Some(layout.id()))?;
    space1.view().set_height(&mut activity, 50)?;
    space1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let text1b = activity.create_text_view("这是第二个元素（上方有50dp间隔）", Some(layout.id()))?;
    text1b.view().set_margin(&mut activity, 5)?;
    text1b.view().set_height_wrap_content(&mut activity)?;
    text1b.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 示例2: 权重间隔（弹性空间）==========
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider1.view().set_margin(&mut activity, 10)?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section2 = activity.create_text_view("示例2: 权重间隔（填充剩余空间）", Some(layout.id()))?;
    section2.set_text_size(&mut activity, 18)?;
    section2.view().set_margin(&mut activity, 10)?;
    section2.view().set_height_wrap_content(&mut activity)?;
    section2.view().set_linear_layout_params(&mut activity, 0, None)?;
    section2.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    let text2a = activity.create_text_view("顶部内容", Some(layout.id()))?;
    text2a.view().set_margin(&mut activity, 5)?;
    text2a.view().set_height_wrap_content(&mut activity)?;
    text2a.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建一个 weight=1 的 Space，它会占据所有剩余空间
    let space2 = activity.create_space(Some(layout.id()))?;
    space2.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let text2b = activity.create_text_view("底部内容（被推到底部）", Some(layout.id()))?;
    text2b.view().set_margin(&mut activity, 5)?;
    text2b.view().set_height_wrap_content(&mut activity)?;
    text2b.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 示例3: 水平间隔 ==========
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider2.view().set_margin(&mut activity, 10)?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section3 = activity.create_text_view("示例3: 水平间隔", Some(layout.id()))?;
    section3.set_text_size(&mut activity, 18)?;
    section3.view().set_margin(&mut activity, 10)?;
    section3.view().set_height_wrap_content(&mut activity)?;
    section3.view().set_linear_layout_params(&mut activity, 0, None)?;
    section3.set_text_color(&mut activity, 0xFFFF9800u32 as i32)?;
    
    // 创建水平布局
    let h_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    h_layout.view().set_margin(&mut activity, 5)?;
    h_layout.view().set_height_wrap_content(&mut activity)?;
    h_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let btn1 = activity.create_button("按钮1", Some(h_layout.id()))?;
    btn1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 水平 Space，宽度30dp
    let h_space = activity.create_space(Some(h_layout.id()))?;
    h_space.view().set_width(&mut activity, 30)?;
    h_space.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let btn2 = activity.create_button("按钮2", Some(h_layout.id()))?;
    btn2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 弹性水平 Space（weight=1），把按钮3推到右边
    let h_space2 = activity.create_space(Some(h_layout.id()))?;
    h_space2.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    let btn3 = activity.create_button("按钮3", Some(h_layout.id()))?;
    btn3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 底部说明 ==========
    let divider3 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider3.view().set_margin(&mut activity, 10)?;
    divider3.view().set_height_wrap_content(&mut activity)?;
    divider3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let info = activity.create_text_view(
        "💡 Space 用途:\n\
        • 固定间隔：设置固定宽度/高度\n\
        • 弹性间隔：使用 weight 填充空间\n\
        • 推送元素：将元素推到边缘",
        Some(layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 观察不同类型的 Space 效果");
    println!("  • Space 本身是不可见的");
    println!("  • 使用 Space 可以精确控制布局");
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
                
                if clicked_id == btn1.id() {
                    println!("点击了按钮1");
                    info.set_text(&mut activity, "✅ 按钮1 被点击")?;
                    info.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                } else if clicked_id == btn2.id() {
                    println!("点击了按钮2");
                    info.set_text(&mut activity, "✅ 按钮2 被点击（左侧有30dp间隔）")?;
                    info.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
                } else if clicked_id == btn3.id() {
                    println!("点击了按钮3");
                    info.set_text(&mut activity, "✅ 按钮3 被点击（被弹性Space推到右边）")?;
                    info.set_text_color(&mut activity, 0xFFFF9800u32 as i32)?;
                }
            },
            _ => {}
        }
    }
}
