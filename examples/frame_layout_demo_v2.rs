// FrameLayout 帧布局演示 - 使用新库 API
// 展示如何使用 FrameLayout 创建层叠布局
// 运行: cargo run --example frame_layout_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== FrameLayout 帧布局演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("🎭 FrameLayout 演示", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("FrameLayout 按添加顺序层叠显示子视图", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 示例1: 简单层叠 ==========
    let section1 = activity.create_text_view("示例1: 文本层叠", Some(layout.id()))?;
    section1.set_text_size(&mut activity, 18)?;
    section1.view().set_margin(&mut activity, 10)?;
    section1.view().set_height_wrap_content(&mut activity)?;
    section1.view().set_linear_layout_params(&mut activity, 0, None)?;
    section1.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建一个 FrameLayout
    let frame1 = activity.create_frame_layout(Some(layout.id()))?;
    frame1.view().set_margin(&mut activity, 10)?;
    frame1.view().set_height(&mut activity, 150)?;
    frame1.view().set_width_match_parent(&mut activity)?;
    frame1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 在 FrameLayout 中添加三个层叠的 TextView
    // 第一层（底层）
    let layer1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━\n第一层（底层）", Some(frame1.id()))?;
    layer1.set_text_size(&mut activity, 20)?;
    layer1.view().set_margin(&mut activity, 5)?;
    layer1.set_text_color(&mut activity, 0xFFFF5722u32 as i32)?;
    
    // 第二层（中层）- 稍微偏移
    let layer2 = activity.create_text_view("\n\n  第二层（中层）", Some(frame1.id()))?;
    layer2.set_text_size(&mut activity, 20)?;
    layer2.view().set_margin(&mut activity, 5)?;
    layer2.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 第三层（顶层）- 更多偏移
    let layer3 = activity.create_text_view("\n\n\n\n    第三层（顶层）", Some(frame1.id()))?;
    layer3.set_text_size(&mut activity, 20)?;
    layer3.view().set_margin(&mut activity, 5)?;
    layer3.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // ========== 示例2: 按钮叠加 ==========
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider1.view().set_margin(&mut activity, 10)?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section2 = activity.create_text_view("示例2: 按钮居中叠加", Some(layout.id()))?;
    section2.set_text_size(&mut activity, 18)?;
    section2.view().set_margin(&mut activity, 10)?;
    section2.view().set_height_wrap_content(&mut activity)?;
    section2.view().set_linear_layout_params(&mut activity, 0, None)?;
    section2.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 创建第二个 FrameLayout
    let frame2 = activity.create_frame_layout(Some(layout.id()))?;
    frame2.view().set_margin(&mut activity, 10)?;
    frame2.view().set_height(&mut activity, 200)?;
    frame2.view().set_width_match_parent(&mut activity)?;
    frame2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 背景文本
    let bg_text = activity.create_text_view("这是背景文本\n\n下面有一个居中的按钮\n\n点击试试", Some(frame2.id()))?;
    bg_text.set_text_size(&mut activity, 16)?;
    bg_text.view().set_margin(&mut activity, 10)?;
    bg_text.set_text_color(&mut activity, 0xFF999999u32 as i32)?;
    
    // 居中按钮（通过在 FrameLayout 中居中）
    // 注意：FrameLayout 默认将子视图居中对齐
    let center_btn = activity.create_button("点我！", Some(frame2.id()))?;
    center_btn.view().set_margin(&mut activity, 80)?;  // 添加边距让按钮看起来居中
    
    // ========== 底部信息 ==========
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider2.view().set_margin(&mut activity, 10)?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let info = activity.create_text_view(
        "💡 FrameLayout 特点:\n\
        • 子视图按添加顺序层叠\n\
        • 后添加的视图在上层\n\
        • 适合创建叠加效果\n\
        • 可以实现简单的 Z 轴布局",
        Some(layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 状态显示
    let status = activity.create_text_view("准备就绪", Some(layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 观察三层文本的层叠效果");
    println!("  • 点击示例2中的居中按钮");
    println!("  • FrameLayout 中后添加的元素在上层");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let mut click_count = 0;
    
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
                
                if clicked_id == center_btn.id() {
                    click_count += 1;
                    println!("✅ 居中按钮被点击！（第{}次）", click_count);
                    
                    let messages = [
                        "✅ 第一次点击！很好！",
                        "👍 第二次点击！继续！",
                        "🎉 第三次点击！太棒了！",
                        "🌟 第四次点击！你很厉害！",
                        "🏆 第五次点击！大师级别！",
                    ];
                    
                    let msg = if click_count <= 5 {
                        messages[click_count - 1]
                    } else {
                        "🎊 你已经是专家了！"
                    };
                    
                    status.set_text(&mut activity, &format!("{} (点击{}次)", msg, click_count))?;
                    
                    // 根据点击次数改变颜色
                    let color = match click_count % 5 {
                        1 => 0xFF4CAF50u32 as i32,  // 绿色
                        2 => 0xFF2196F3u32 as i32,  // 蓝色
                        3 => 0xFFFF9800u32 as i32,  // 橙色
                        4 => 0xFF9C27B0u32 as i32,  // 紫色
                        _ => 0xFFF44336u32 as i32,  // 红色
                    };
                    status.set_text_color(&mut activity, color)?;
                }
            },
            _ => {}
        }
    }
}
