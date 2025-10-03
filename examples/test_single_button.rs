// 测试单个按钮 - 最小化测试
use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== 单个按钮测试 ===\n");
    
    let mut activity = Activity::new(true)?;
    println!("✓ Activity 创建\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    println!("✓ 布局创建 (ID = {})\n", layout.id());
    
    // 只创建一个按钮
    let button = activity.create_button("点我！", Some(layout.id()))?;
    println!("✓ 按钮创建 (ID = {})\n", button.id());
    
    println!("请查看屏幕是否显示按钮");
    println!("等待 10 秒...\n");
    
    thread::sleep(Duration::from_secs(10));
    
    activity.finish()?;
    println!("✓ 结束");
    
    Ok(())
}
