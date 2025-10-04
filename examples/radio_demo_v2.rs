// RadioGroup + RadioButton 单选组演示 - 使用新库 API
// 展示如何创建和使用单选按钮组
// 运行: cargo run --example radio_demo_v2 --release

use termux_gui::{Activity, TextView, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== RadioGroup + RadioButton 单选演示 (新库版本) ===\n");
    
    // 创建 Activity（对话框模式）
    let mut activity = Activity::new(true)?;
    println!("✓ 连接建立\n");
    
    // 创建主布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let title = activity.create_text_view("📦 选择配送方式", Some(layout.id()))?;
    title.set_text_size(&mut activity, 26)?;
    title.view().set_margin(&mut activity, 10)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 第一个 RadioGroup - 配送方式 ==========
    let section1 = activity.create_text_view("配送方式：", Some(layout.id()))?;
    section1.set_text_size(&mut activity, 18)?;
    section1.view().set_margin(&mut activity, 8)?;
    section1.view().set_height_wrap_content(&mut activity)?;
    section1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let radio_group1 = activity.create_radio_group(Some(layout.id()))?;
    radio_group1.view().set_height_wrap_content(&mut activity)?;
    radio_group1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 1.1 - 标准配送（默认选中）
    let radio1_1 = activity.create_radio_button_checked(
        "📮 标准配送 (免费, 3-5天)", 
        Some(radio_group1.id()), 
        true
    )?;
    radio1_1.view().set_height_wrap_content(&mut activity)?;
    radio1_1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 1.2 - 快速配送
    let radio1_2 = activity.create_radio_button_checked(
        "🚚 快速配送 (¥15, 1-2天)", 
        Some(radio_group1.id()), 
        false
    )?;
    radio1_2.view().set_height_wrap_content(&mut activity)?;
    radio1_2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 1.3 - 当日达
    let radio1_3 = activity.create_radio_button_checked(
        "⚡ 当日达 (¥30, 当天送达)", 
        Some(radio_group1.id()), 
        false
    )?;
    radio1_3.view().set_height_wrap_content(&mut activity)?;
    radio1_3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 分隔线
    let divider1 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider1.view().set_height_wrap_content(&mut activity)?;
    divider1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 第二个 RadioGroup - 支付方式 ==========
    let section2 = activity.create_text_view("支付方式：", Some(layout.id()))?;
    section2.set_text_size(&mut activity, 18)?;
    section2.view().set_margin(&mut activity, 8)?;
    section2.view().set_height_wrap_content(&mut activity)?;
    section2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let radio_group2 = activity.create_radio_group(Some(layout.id()))?;
    radio_group2.view().set_height_wrap_content(&mut activity)?;
    radio_group2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 2.1 - 微信支付（默认选中）
    let radio2_1 = activity.create_radio_button_checked(
        "💚 微信支付", 
        Some(radio_group2.id()), 
        true
    )?;
    radio2_1.view().set_height_wrap_content(&mut activity)?;
    radio2_1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 2.2 - 支付宝
    let radio2_2 = activity.create_radio_button_checked(
        "💙 支付宝", 
        Some(radio_group2.id()), 
        false
    )?;
    radio2_2.view().set_height_wrap_content(&mut activity)?;
    radio2_2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 2.3 - 货到付款
    let radio2_3 = activity.create_radio_button_checked(
        "💰 货到付款", 
        Some(radio_group2.id()), 
        false
    )?;
    radio2_3.view().set_height_wrap_content(&mut activity)?;
    radio2_3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 分隔线
    let divider2 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider2.view().set_height_wrap_content(&mut activity)?;
    divider2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 第三个 RadioGroup - 发票类型 ==========
    let section3 = activity.create_text_view("发票类型：", Some(layout.id()))?;
    section3.set_text_size(&mut activity, 18)?;
    section3.view().set_margin(&mut activity, 8)?;
    section3.view().set_height_wrap_content(&mut activity)?;
    section3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let radio_group3 = activity.create_radio_group(Some(layout.id()))?;
    radio_group3.view().set_height_wrap_content(&mut activity)?;
    radio_group3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 3.1 - 不需要发票（默认选中）
    let radio3_1 = activity.create_radio_button_checked(
        "❌ 不需要发票", 
        Some(radio_group3.id()), 
        true
    )?;
    radio3_1.view().set_height_wrap_content(&mut activity)?;
    radio3_1.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 3.2 - 电子发票
    let radio3_2 = activity.create_radio_button_checked(
        "📧 电子发票", 
        Some(radio_group3.id()), 
        false
    )?;
    radio3_2.view().set_height_wrap_content(&mut activity)?;
    radio3_2.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // RadioButton 3.3 - 纸质发票
    let radio3_3 = activity.create_radio_button_checked(
        "📄 纸质发票 (+¥5)", 
        Some(radio_group3.id()), 
        false
    )?;
    radio3_3.view().set_height_wrap_content(&mut activity)?;
    radio3_3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 分隔线
    let divider3 = activity.create_text_view("━━━━━━━━━━━━━━━━━━━━", Some(layout.id()))?;
    divider3.view().set_margin(&mut activity, 8)?;
    divider3.view().set_height_wrap_content(&mut activity)?;
    divider3.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 状态显示
    let status = activity.create_text_view(
        "当前选择:\n配送: 标准配送\n支付: 微信支付\n发票: 不需要发票",
        Some(layout.id())
    )?;
    status.view().set_margin(&mut activity, 10)?;
    status.view().set_height_wrap_content(&mut activity)?;
    status.view().set_linear_layout_params(&mut activity, 0, None)?;
    status.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
    
    // 总价显示
    let price = activity.create_text_view("总计: ¥0", Some(layout.id()))?;
    price.set_text_size(&mut activity, 22)?;
    price.view().set_margin(&mut activity, 10)?;
    price.view().set_height_wrap_content(&mut activity)?;
    price.view().set_linear_layout_params(&mut activity, 0, None)?;
    price.set_text_color(&mut activity, 0xFFFF5722u32 as i32)?;
    
    // 确认按钮
    let confirm_btn = activity.create_button("✅ 确认订单", Some(layout.id()))?;
    confirm_btn.view().set_margin(&mut activity, 10)?;
    confirm_btn.view().set_height_wrap_content(&mut activity)?;
    confirm_btn.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 每组只能选择一个选项");
    println!("  • 选择会自动更新总价");
    println!("  • 点击 '确认订单' 提交");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 状态跟踪
    let delivery_options = ["标准配送", "快速配送", "当日达"];
    let delivery_prices = [0, 15, 30];
    let mut delivery_index = 0;
    
    let payment_options = ["微信支付", "支付宝", "货到付款"];
    let mut payment_index = 0;
    
    let invoice_options = ["不需要发票", "电子发票", "纸质发票"];
    let invoice_prices = [0, 0, 5];
    let mut invoice_index = 0;
    
    // 更新显示的辅助函数
    let update_display = |activity: &mut Activity, 
                          status: &TextView, 
                          price: &TextView,
                          del_idx: usize, 
                          pay_idx: usize, 
                          inv_idx: usize| -> Result<()> {
        let status_text = format!(
            "当前选择:\n配送: {}\n支付: {}\n发票: {}",
            delivery_options[del_idx],
            payment_options[pay_idx],
            invoice_options[inv_idx]
        );
        status.set_text(activity, &status_text)?;
        
        let total = delivery_prices[del_idx] + invoice_prices[inv_idx];
        let price_text = format!("总计: ¥{}", total);
        price.set_text(activity, &price_text)?;
        
        Ok(())
    };
    
    // 事件循环
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                // Activity 已被系统销毁，直接退出即可
                // 不要调用 activity.finish()，因为 Activity 已经不存在了
                println!("\n✓ Activity 已关闭");
                return Ok(());
            },
            "selected" => {
                let group_id = event_value["id"].as_i64().unwrap_or(-1);
                let selected_id = event_value["selected"].as_i64().unwrap_or(-1);
                
                if group_id == radio_group1.id() {
                    // 配送方式组
                    if selected_id == radio1_1.id() {
                        delivery_index = 0;
                        println!("📮 选择: 标准配送");
                    } else if selected_id == radio1_2.id() {
                        delivery_index = 1;
                        println!("🚚 选择: 快速配送 (+¥15)");
                    } else if selected_id == radio1_3.id() {
                        delivery_index = 2;
                        println!("⚡ 选择: 当日达 (+¥30)");
                    }
                } else if group_id == radio_group2.id() {
                    // 支付方式组
                    if selected_id == radio2_1.id() {
                        payment_index = 0;
                        println!("💚 选择: 微信支付");
                    } else if selected_id == radio2_2.id() {
                        payment_index = 1;
                        println!("💙 选择: 支付宝");
                    } else if selected_id == radio2_3.id() {
                        payment_index = 2;
                        println!("💰 选择: 货到付款");
                    }
                } else if group_id == radio_group3.id() {
                    // 发票类型组
                    if selected_id == radio3_1.id() {
                        invoice_index = 0;
                        println!("❌ 选择: 不需要发票");
                    } else if selected_id == radio3_2.id() {
                        invoice_index = 1;
                        println!("📧 选择: 电子发票");
                    } else if selected_id == radio3_3.id() {
                        invoice_index = 2;
                        println!("📄 选择: 纸质发票 (+¥5)");
                    }
                }
                
                update_display(&mut activity, &status, &price,
                             delivery_index, payment_index, invoice_index)?;
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == confirm_btn.id() {
                    println!("\n✅ 订单确认:");
                    println!("  配送方式: {} (¥{})", 
                            delivery_options[delivery_index],
                            delivery_prices[delivery_index]);
                    println!("  支付方式: {}", payment_options[payment_index]);
                    println!("  发票类型: {} (¥{})",
                            invoice_options[invoice_index],
                            invoice_prices[invoice_index]);
                    println!("  总计: ¥{}", 
                            delivery_prices[delivery_index] + invoice_prices[invoice_index]);
                    
                    // 显示确认消息
                    status.set_text(&mut activity, "✅ 订单已确认！")?;
                    status.set_text_color(&mut activity, 0xFF4CAF50u32 as i32)?;
                }
            },
            _ => {}
        }
    }
}
