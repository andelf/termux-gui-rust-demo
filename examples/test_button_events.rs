// 按钮事件测试
use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== 按钮事件测试 ===\n");
    
    // 创建 Activity
    let mut activity = Activity::new(true)?;
    println!("✓ Activity 创建成功 (ID = {})\n", activity.id());
    
    // 创建布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建计数显示
    let counter = activity.create_text_view("计数: 0", Some(layout.id()))?;
    counter.set_text_size(&mut activity, 24)?;
    
    // 创建按钮
    let button = activity.create_button("点击我", Some(layout.id()))?;
    
    println!("✓ 界面创建完成");
    println!("  - Counter ID: {}", counter.id());
    println!("  - Button ID: {}\n", button.id());
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("请点击按钮进行测试");
    println!("关闭窗口或按 Ctrl+C 退出");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件循环
    let mut count = 0;
    
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        
        println!("[事件] type = {}", event_type);
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                println!("\n✓ 程序结束");
                return Ok(());
            },
            "click" => {
                let id = event["value"]["id"].as_i64().unwrap_or(-1);
                println!("  点击的组件 ID = {}", id);
                
                if id == button.id() {
                    count += 1;
                    println!("  ➕ 计数增加到 {}", count);
                    counter.set_text(&mut activity, &format!("计数: {}", count))?;
                }
            },
            _ => {
                // 忽略其他事件
            }
        }
    }
}
