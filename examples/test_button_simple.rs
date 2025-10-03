// 简单的按钮测试 - 逐步调试
use termux_gui::{Activity, Result};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== 按钮简单测试 ===\n");
    
    println!("[1] 创建 Activity...");
    let mut activity = Activity::new(true)?;
    println!("    ✓ Activity ID = {}\n", activity.id());
    
    println!("[2] 创建主布局...");
    let layout = activity.create_linear_layout(None)?;
    println!("    ✓ Layout ID = {}\n", layout.id());
    
    println!("[3] 创建标题文本...");
    let title = activity.create_text_view("按钮测试", Some(layout.id()))?;
    println!("    ✓ Title ID = {}\n", title.id());
    
    println!("[4] 设置标题大小...");
    title.set_text_size(&mut activity, 24)?;
    println!("    ✓ 大小设置完成\n");
    
    println!("[5] 创建按钮...");
    let button = activity.create_button("点击我", Some(layout.id()))?;
    println!("    ✓ Button ID = {}\n", button.id());
    
    println!("✓ 所有组件创建完成！");
    println!("等待 10 秒钟展示界面...\n");
    
    thread::sleep(Duration::from_secs(10));
    
    println!("结束 Activity...");
    activity.finish()?;
    println!("完成！");
    
    Ok(())
}
