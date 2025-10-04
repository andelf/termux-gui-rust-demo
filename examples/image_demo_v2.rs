// ImageView 图像显示演示 - 使用新库 API
// 展示如何显示图片（base64编码）
// 运行: cargo run --example image_demo_v2 --release

use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== ImageView 图像显示演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("🖼️ 图片显示测试", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建说明
    let desc = activity.create_text_view("显示一个小的测试图片", Some(layout.id()))?;
    desc.view().set_margin(&mut activity, 5)?;
    desc.view().set_height_wrap_content(&mut activity)?;
    desc.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 ImageView
    let image_view = activity.create_image_view(Some(layout.id()))?;
    image_view.view().set_margin(&mut activity, 10)?;
    // ImageView 使用权重占据主要空间
    image_view.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 创建一个简单的 1x1 红色像素 PNG 图片（base64编码）
    // 这是一个最小的有效PNG文件
    let red_pixel_png_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";
    
    // 设置图片
    image_view.set_image(&mut activity, red_pixel_png_base64)?;
    
    // 创建状态显示
    let status = activity.create_text_view("✅ 图片已加载", Some(layout.id()))?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
    
    // 创建按钮布局
    let button_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;
    button_layout.view().set_margin(&mut activity, 10)?;
    button_layout.view().set_height_wrap_content(&mut activity)?;
    button_layout.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 红色图片按钮
    let red_btn = activity.create_button("🔴 红色", Some(button_layout.id()))?;
    red_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 绿色图片按钮
    let green_btn = activity.create_button("🟢 绿色", Some(button_layout.id()))?;
    green_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 蓝色图片按钮
    let blue_btn = activity.create_button("🔵 蓝色", Some(button_layout.id()))?;
    blue_btn.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 点击按钮切换不同颜色的图片");
    println!("  • 图片会自动缩放填充区域");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 预定义几个不同颜色的1x1像素PNG图片（base64）
    // 红色像素
    let red_png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";
    // 绿色像素
    let green_png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M/wHwAEBgIApD5fRAAAAABJRU5ErkJggg==";
    // 蓝色像素
    let blue_png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPj/HwADBwIAMCbHYQAAAABJRU5ErkJggg==";
    
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
                
                if clicked_id == red_btn.id() {
                    println!("🔴 切换到红色图片");
                    image_view.set_image(&mut activity, red_png)?;
                    status.set_text(&mut activity, "🔴 红色图片")?;
                    status.set_text_color(&mut activity, 0xFFF44336u32 as i32)?;
                } else if clicked_id == green_btn.id() {
                    println!("🟢 切换到绿色图片");
                    image_view.set_image(&mut activity, green_png)?;
                    status.set_text(&mut activity, "🟢 绿色图片")?;
                    status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                } else if clicked_id == blue_btn.id() {
                    println!("🔵 切换到蓝色图片");
                    image_view.set_image(&mut activity, blue_png)?;
                    status.set_text(&mut activity, "🔵 蓝色图片")?;
                    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
                }
            },
            _ => {}
        }
    }
}
