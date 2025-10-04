// HorizontalScrollView 水平滚动演示 - 使用新库 API
// 展示如何使用 HorizontalScrollView 实现水平滚动
// 运行: cargo run --example horizontal_scroll_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== HorizontalScrollView 水平滚动演示 (新库版本) ===\n");
    
    // 创建 Activity（全屏模式）
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立\n");
    
    // 创建主垂直布局
    let main_layout = activity.create_linear_layout(None)?;
    main_layout.view().set_margin(&mut activity, 15)?;
    
    // 创建标题
    let title = activity.create_text_view("↔️ HorizontalScrollView 演示", Some(main_layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("左右滑动查看更多按钮", Some(main_layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 第一行: 水果按钮 ==========
    let section1 = activity.create_text_view("第一行: 水果按钮（左右滑动）", Some(main_layout.id()))?;
    section1.set_text_size(&mut activity, 18)?;
    section1.view().set_margin(&mut activity, 10)?;
    section1.view().set_height_wrap_content(&mut activity)?;
    section1.view().set_linear_layout_params(&mut activity, 0, None)?;
    section1.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建第一个 HorizontalScrollView
    let h_scroll1 = activity.create_horizontal_scroll_view(Some(main_layout.id()))?;
    h_scroll1.view().set_margin(&mut activity, 10)?;
    h_scroll1.view().set_height_wrap_content(&mut activity)?;
    h_scroll1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 在 HorizontalScrollView 内创建水平 LinearLayout
    let h_layout1 = activity.create_linear_layout_horizontal(Some(h_scroll1.id()))?;
    h_layout1.view().set_height_wrap_content(&mut activity)?;
    
    // 添加10个水果按钮
    let fruits = ["🍎 苹果", "🍊 橙子", "🍌 香蕉", "🍇 葡萄", "🍓 草莓", 
                  "🍑 桃子", "🍒 樱桃", "🍍 菠萝", "🥝 猕猴桃", "🥭 芒果"];
    let mut fruit_buttons = Vec::new();
    for fruit in &fruits {
        let btn = activity.create_button(fruit, Some(h_layout1.id()))?;
        btn.view().set_width(&mut activity, 180)?;  // 固定宽度180dp
        btn.view().set_margin(&mut activity, 5)?;
        fruit_buttons.push(btn);
    }
    
    // ========== 第二行: 数字按钮 ==========
    let section2 = activity.create_text_view("第二行: 数字按钮（左右滑动）", Some(main_layout.id()))?;
    section2.set_text_size(&mut activity, 18)?;
    section2.view().set_margin(&mut activity, 10)?;
    section2.view().set_height_wrap_content(&mut activity)?;
    section2.view().set_linear_layout_params(&mut activity, 0, None)?;
    section2.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 创建第二个 HorizontalScrollView
    let h_scroll2 = activity.create_horizontal_scroll_view(Some(main_layout.id()))?;
    h_scroll2.view().set_margin(&mut activity, 10)?;
    h_scroll2.view().set_height_wrap_content(&mut activity)?;
    h_scroll2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 在 HorizontalScrollView 内创建水平 LinearLayout
    let h_layout2 = activity.create_linear_layout_horizontal(Some(h_scroll2.id()))?;
    h_layout2.view().set_height_wrap_content(&mut activity)?;
    
    // 添加20个数字按钮
    let mut number_buttons = Vec::new();
    for i in 0..20 {
        let btn = activity.create_button(&format!("数字 {}", i * 5), Some(h_layout2.id()))?;
        btn.view().set_width(&mut activity, 150)?;  // 固定宽度150dp
        btn.view().set_margin(&mut activity, 5)?;
        number_buttons.push(btn);
    }
    
    // ========== 底部信息 ==========
    let divider = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(main_layout.id()))?;
    divider.view().set_margin(&mut activity, 10)?;
    divider.view().set_height_wrap_content(&mut activity)?;
    divider.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let info = activity.create_text_view(
        "💡 提示:\n\
        • 在每一行中左右滑动\n\
        • 点击任意按钮查看效果\n\
        • HorizontalScrollView 支持水平滚动",
        Some(main_layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 状态显示
    let status = activity.create_text_view("准备就绪 - 尝试左右滑动", Some(main_layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 第一行: 10个水果按钮");
    println!("  • 第二行: 20个数字按钮");
    println!("  • 左右滑动查看更多内容");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
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
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                // 检查水果按钮
                for (i, btn) in fruit_buttons.iter().enumerate() {
                    if clicked_id == btn.id() {
                        println!("✅ 选择: {}", fruits[i]);
                        status.set_text(&mut activity, &format!("✅ 选择: {}", fruits[i]))?;
                        status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                        break;
                    }
                }
                
                // 检查数字按钮
                for (i, btn) in number_buttons.iter().enumerate() {
                    if clicked_id == btn.id() {
                        let value = i * 5;
                        println!("🔢 选择数字: {}", value);
                        status.set_text(&mut activity, &format!("🔢 选择数字: {}", value))?;
                        status.set_text_color(&mut activity, 0xFFFF9800u32 as i32)?;
                        break;
                    }
                }
            },
            _ => {}
        }
    }
}
