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
    
    // 创建标题
    let title = activity.create_text_view("↔️ HorizontalScrollView 演示", Some(main_layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("左右滑动查看更多内容", Some(main_layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 示例1: 水平按钮滚动 ==========
    let section1 = activity.create_text_view("示例1: 水平按钮列表", Some(main_layout.id()))?;
    section1.set_text_size(&mut activity, 18)?;
    section1.view().set_margin(&mut activity, 10)?;
    section1.view().set_height_wrap_content(&mut activity)?;
    section1.view().set_linear_layout_params(&mut activity, 0, None)?;
    section1.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 创建 HorizontalScrollView
    let h_scroll1 = activity.create_horizontal_scroll_view(Some(main_layout.id()))?;
    h_scroll1.view().set_margin(&mut activity, 10)?;
    h_scroll1.view().set_height_wrap_content(&mut activity)?;
    h_scroll1.view().set_width_match_parent(&mut activity)?;
    h_scroll1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 在 HorizontalScrollView 内创建水平 LinearLayout
    let h_layout1 = activity.create_linear_layout_horizontal(Some(h_scroll1.id()))?;
    h_layout1.view().set_height_wrap_content(&mut activity)?;
    h_layout1.view().set_width_wrap_content(&mut activity)?;  // 关键：宽度设为wrap_content让内容可以超出屏幕
    
    // 添加10个按钮
    let button_labels = [
        "🍎 苹果", "🍊 橙子", "🍌 香蕉", "🍇 葡萄", "🍓 草莓",
        "🍑 桃子", "🍒 樱桃", "🍍 菠萝", "🥝 猕猴桃", "🥭 芒果"
    ];
    let mut buttons = Vec::new();
    for label in &button_labels {
        let btn = activity.create_button(label, Some(h_layout1.id()))?;
        btn.view().set_width(&mut activity, 180)?;  // 设置固定宽度
        btn.view().set_margin(&mut activity, 5)?;
        buttons.push(btn);
    }
    
    // ========== 示例2: 水平图片画廊 ==========
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(main_layout.id()))?;
    divider1.view().set_margin(&mut activity, 10)?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section2 = activity.create_text_view("示例2: 水平卡片画廊", Some(main_layout.id()))?;
    section2.set_text_size(&mut activity, 18)?;
    section2.view().set_margin(&mut activity, 10)?;
    section2.view().set_height_wrap_content(&mut activity)?;
    section2.view().set_linear_layout_params(&mut activity, 0, None)?;
    section2.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 第二个 HorizontalScrollView
    let h_scroll2 = activity.create_horizontal_scroll_view(Some(main_layout.id()))?;
    h_scroll2.view().set_margin(&mut activity, 10)?;
    h_scroll2.view().set_height_wrap_content(&mut activity)?;
    h_scroll2.view().set_width_match_parent(&mut activity)?;
    h_scroll2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 水平布局容器
    let h_layout2 = activity.create_linear_layout_horizontal(Some(h_scroll2.id()))?;
    h_layout2.view().set_height_wrap_content(&mut activity)?;
    h_layout2.view().set_width_wrap_content(&mut activity)?;  // 关键：宽度设为wrap_content
    
    // 添加5个"卡片"（垂直布局模拟卡片）
    let card_emojis = ["🌟", "🎨", "🎭", "🎪", "🎯"];
    let card_titles = ["明星", "艺术", "戏剧", "马戏", "目标"];
    let card_colors = [
        0xFFFFEB3Bu32 as i32,  // 黄色
        0xFFE91E63u32 as i32,  // 粉色
        0xFF9C27B0u32 as i32,  // 紫色
        0xFF3F51B5u32 as i32,  // 靛蓝
        0xFF4CAF50u32 as i32,  // 绿色
    ];
    
    for i in 0..5 {
        // 每个卡片是一个垂直布局
        let card = activity.create_linear_layout(Some(h_layout2.id()))?;
        card.view().set_margin(&mut activity, 10)?;
        card.view().set_width(&mut activity, 150)?;
        card.view().set_height_wrap_content(&mut activity)?;
        
        // 卡片图标
        let icon = activity.create_text_view(card_emojis[i], Some(card.id()))?;
        icon.set_text_size(&mut activity, 48)?;
        icon.view().set_margin(&mut activity, 10)?;
        icon.view().set_height_wrap_content(&mut activity)?;
        icon.view().set_linear_layout_params(&mut activity, 0, None)?;
        
        // 卡片标题
        let title = activity.create_text_view(card_titles[i], Some(card.id()))?;
        title.set_text_size(&mut activity, 18)?;
        title.view().set_margin(&mut activity, 5)?;
        title.view().set_height_wrap_content(&mut activity)?;
        title.view().set_linear_layout_params(&mut activity, 0, None)?;
        title.set_text_color(&mut activity, card_colors[i])?;
    }
    
    // ========== 示例3: 数字滚动条 ==========
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(main_layout.id()))?;
    divider2.view().set_margin(&mut activity, 10)?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let section3 = activity.create_text_view("示例3: 数字选择器", Some(main_layout.id()))?;
    section3.set_text_size(&mut activity, 18)?;
    section3.view().set_margin(&mut activity, 10)?;
    section3.view().set_height_wrap_content(&mut activity)?;
    section3.view().set_linear_layout_params(&mut activity, 0, None)?;
    section3.set_text_color(&mut activity, 0xFFFF9800u32 as i32)?;
    
    // 第三个 HorizontalScrollView
    let h_scroll3 = activity.create_horizontal_scroll_view(Some(main_layout.id()))?;
    h_scroll3.view().set_margin(&mut activity, 10)?;
    h_scroll3.view().set_height_wrap_content(&mut activity)?;
    h_scroll3.view().set_width_match_parent(&mut activity)?;
    h_scroll3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 水平布局容器
    let h_layout3 = activity.create_linear_layout_horizontal(Some(h_scroll3.id()))?;
    h_layout3.view().set_height_wrap_content(&mut activity)?;
    h_layout3.view().set_width_wrap_content(&mut activity)?;  // 关键：宽度设为wrap_content
    
    // 添加20个数字
    let mut number_buttons = Vec::new();
    for i in 0..20 {
        let btn = activity.create_button(&format!("{}", i * 5), Some(h_layout3.id()))?;
        btn.view().set_width(&mut activity, 120)?;  // 设置固定宽度
        btn.view().set_margin(&mut activity, 5)?;
        number_buttons.push(btn);
    }
    
    // ========== 底部信息 ==========
    let divider3 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(main_layout.id()))?;
    divider3.view().set_margin(&mut activity, 10)?;
    divider3.view().set_height_wrap_content(&mut activity)?;
    divider3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let info = activity.create_text_view(
        "💡 HorizontalScrollView 特点:\n\
        • 支持水平方向滚动\n\
        • 内容超过宽度时可左右滑动\n\
        • 适合画廊、选项列表等\n\
        • 与 NestedScrollView 类似",
        Some(main_layout.id())
    )?;
    info.view().set_margin(&mut activity, 10)?;
    info.view().set_height_wrap_content(&mut activity)?;
    info.view().set_linear_layout_params(&mut activity, 0, None)?;
    info.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    
    // 状态显示
    let status = activity.create_text_view("准备就绪 - 左右滑动试试", Some(main_layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 左右滑动查看更多内容");
    println!("  • 示例1: 10个水果按钮");
    println!("  • 示例2: 5个彩色卡片");
    println!("  • 示例3: 20个数字按钮");
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
                
                // 检查水果按钮
                for (i, btn) in buttons.iter().enumerate() {
                    if clicked_id == btn.id() {
                        println!("✅ 选择了: {}", button_labels[i]);
                        status.set_text(&mut activity, &format!("✅ 选择了: {}", button_labels[i]))?;
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
