// Spinner 下拉列表演示 - 使用新库 API
// 展示如何创建和使用下拉选择列表，包括级联选择
// 运行: cargo run --example spinner_demo_v2 --release

use termux_gui::{Activity, TextView, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== Spinner 下拉列表演示 (新库版本) ===\n");
    
    // 创建 Activity（全屏模式）
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立\n");
    
    // 创建 NestedScrollView 作为根布局（支持滚动）
    let scroll = activity.create_nested_scroll_view(None)?;
    
    // 创建主布局（放在 ScrollView 内）
    let layout = activity.create_linear_layout(Some(scroll.id()))?;
    layout.view().set_margin(&mut activity, 20)?;
    
    // 标题
    let title = activity.create_text_view("📱 手机订购向导", Some(layout.id()))?;
    title.set_text_size(&mut activity, 24)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 品牌选择 ==========
    let brand_label = activity.create_text_view("选择品牌:", Some(layout.id()))?;
    brand_label.set_text_size(&mut activity, 18)?;
    brand_label.view().set_margin(&mut activity, 10)?;
    brand_label.view().set_height_wrap_content(&mut activity)?;
    brand_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let brands = vec!["请选择", "Apple", "Samsung", "Huawei", "Xiaomi", "OPPO", "Vivo"];
    let brand_spinner = activity.create_spinner(Some(layout.id()))?;
    brand_spinner.view().set_width_match_parent(&mut activity)?;
    brand_spinner.view().set_height_wrap_content(&mut activity)?;
    brand_spinner.view().set_linear_layout_params(&mut activity, 0, None)?;
    brand_spinner.set_list(&mut activity, &brands)?;
    
    // ========== 型号选择 ==========
    let model_label = activity.create_text_view("选择型号:", Some(layout.id()))?;
    model_label.set_text_size(&mut activity, 18)?;
    model_label.view().set_margin(&mut activity, 10)?;
    model_label.view().set_height_wrap_content(&mut activity)?;
    model_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let model_spinner = activity.create_spinner(Some(layout.id()))?;
    model_spinner.view().set_width_match_parent(&mut activity)?;
    model_spinner.view().set_height_wrap_content(&mut activity)?;
    model_spinner.view().set_linear_layout_params(&mut activity, 0, None)?;
    model_spinner.set_list(&mut activity, &["请先选择品牌"])?;
    
    // ========== 容量选择 ==========
    let storage_label = activity.create_text_view("选择容量:", Some(layout.id()))?;
    storage_label.set_text_size(&mut activity, 18)?;
    storage_label.view().set_margin(&mut activity, 10)?;
    storage_label.view().set_height_wrap_content(&mut activity)?;
    storage_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let storages = vec!["请选择", "64GB", "128GB", "256GB", "512GB", "1TB"];
    let storage_spinner = activity.create_spinner(Some(layout.id()))?;
    storage_spinner.view().set_width_match_parent(&mut activity)?;
    storage_spinner.view().set_height_wrap_content(&mut activity)?;
    storage_spinner.view().set_linear_layout_params(&mut activity, 0, None)?;
    storage_spinner.set_list(&mut activity, &storages)?;
    
    // ========== 颜色选择 ==========
    let color_label = activity.create_text_view("选择颜色:", Some(layout.id()))?;
    color_label.set_text_size(&mut activity, 18)?;
    color_label.view().set_margin(&mut activity, 10)?;
    color_label.view().set_height_wrap_content(&mut activity)?;
    color_label.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    let colors = vec!["请选择", "黑色", "白色", "金色", "银色", "蓝色", "紫色", "绿色"];
    let color_spinner = activity.create_spinner(Some(layout.id()))?;
    color_spinner.view().set_width_match_parent(&mut activity)?;
    color_spinner.view().set_height_wrap_content(&mut activity)?;
    color_spinner.view().set_linear_layout_params(&mut activity, 0, None)?;
    color_spinner.set_list(&mut activity, &colors)?;
    
    // ========== 结果显示 ==========
    let result = activity.create_text_view("请完成选择", Some(layout.id()))?;
    result.set_text_size(&mut activity, 16)?;
    result.set_text_color(&mut activity, 0xFF666666u32 as i32)?;
    result.view().set_margin(&mut activity, 10)?;
    result.view().set_height_wrap_content(&mut activity)?;
    result.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // ========== 提交按钮 ==========
    let submit_btn = activity.create_button("🛒 确认订购", Some(layout.id()))?;
    submit_btn.view().set_margin(&mut activity, 10)?;
    submit_btn.view().set_height_wrap_content(&mut activity)?;
    submit_btn.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示: 从下拉列表中选择");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 选择状态
    let mut brand_selection = String::new();
    let mut model_selection = String::new();
    let mut storage_selection = String::new();
    let mut color_selection = String::new();
    
    // 更新结果显示的辅助函数
    let update_result = |activity: &mut Activity, 
                         result: &TextView,
                         brand: &str, 
                         model: &str, 
                         storage: &str, 
                         color: &str| -> Result<()> {
        let all_selected = !brand.is_empty() && !model.is_empty() 
                          && !storage.is_empty() && !color.is_empty();
        
        let result_text = if all_selected {
            format!("✅ 已选择:\n品牌: {}\n型号: {}\n容量: {}\n颜色: {}",
                    brand, model, storage, color)
        } else {
            let mut missing = Vec::new();
            if brand.is_empty() { missing.push("品牌"); }
            if model.is_empty() { missing.push("型号"); }
            if storage.is_empty() { missing.push("容量"); }
            if color.is_empty() { missing.push("颜色"); }
            format!("⚠️ 待选择: {}", missing.join(", "))
        };
        
        result.set_text(activity, &result_text)?;
        
        let text_color = if all_selected { 
            0xFF4CAF50u32 as i32  // 绿色
        } else { 
            0xFF666666u32 as i32  // 灰色
        };
        result.set_text_color(activity, text_color)?;
        
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
                println!("\n✓ Activity 已关闭");
                return Ok(());
            },
            "itemselected" => {
                let view_id = event_value["id"].as_i64().unwrap_or(-1);
                // IMPORTANT: "selected" field contains the selected TEXT, not index!
                let selected_text = event_value["selected"].as_str().unwrap_or("");
                
                if view_id == brand_spinner.id() {
                    // 品牌选择 - 使用字符串匹配而不是索引
                    if selected_text != "请选择" && !selected_text.is_empty() {
                        brand_selection = selected_text.to_string();
                        println!("📱 品牌: {}", selected_text);
                        
                        // 根据品牌更新型号列表
                        let models: Vec<&str> = match selected_text {
                            "Apple" => vec!["请选择", "iPhone 15 Pro Max", "iPhone 15 Pro", "iPhone 15", "iPhone 14"],
                            "Samsung" => vec!["请选择", "Galaxy S24 Ultra", "Galaxy S24+", "Galaxy S24", "Galaxy Z Fold5"],
                            "Huawei" => vec!["请选择", "Mate 60 Pro", "Mate 60", "P60 Pro", "P60"],
                            "Xiaomi" => vec!["请选择", "14 Ultra", "14 Pro", "14", "13T Pro"],
                            "OPPO" => vec!["请选择", "Find X7 Ultra", "Find X7", "Reno 11 Pro", "Reno 11"],
                            "Vivo" => vec!["请选择", "X100 Pro", "X100", "S18 Pro", "S18"],
                            _ => vec!["请选择"],
                        };
                        
                        // 更新型号 Spinner
                        model_spinner.set_list(&mut activity, &models)?;
                        
                        // 重置型号选择
                        model_selection.clear();
                    } else {
                        brand_selection.clear();
                        model_spinner.set_list(&mut activity, &["请先选择品牌"])?;
                        model_selection.clear();
                    }
                } else if view_id == model_spinner.id() {
                    // 型号选择 - 使用字符串匹配
                    if selected_text != "请选择" && selected_text != "请先选择品牌" && !selected_text.is_empty() {
                        model_selection = selected_text.to_string();
                        println!("📱 型号: {}", selected_text);
                    } else {
                        model_selection.clear();
                    }
                } else if view_id == storage_spinner.id() {
                    // 容量选择 - 使用字符串匹配
                    if selected_text != "请选择" && !selected_text.is_empty() {
                        storage_selection = selected_text.to_string();
                        println!("💾 容量: {}", selected_text);
                    } else {
                        storage_selection.clear();
                    }
                } else if view_id == color_spinner.id() {
                    // 颜色选择 - 使用字符串匹配
                    if selected_text != "请选择" && !selected_text.is_empty() {
                        color_selection = selected_text.to_string();
                        println!("🎨 颜色: {}", selected_text);
                    } else {
                        color_selection.clear();
                    }
                }
                
                // 更新结果显示
                update_result(&mut activity, &result,
                             &brand_selection, &model_selection, 
                             &storage_selection, &color_selection)?;
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == submit_btn.id() {
                    let all_selected = !brand_selection.is_empty() 
                                     && !model_selection.is_empty()
                                     && !storage_selection.is_empty() 
                                     && !color_selection.is_empty();
                    
                    if all_selected {
                        let order_info = format!(
                            "🎉 订购成功！\n\n品牌: {}\n型号: {}\n容量: {}\n颜色: {}\n\n感谢您的订购！",
                            brand_selection, model_selection, storage_selection, color_selection
                        );
                        
                        println!("\n{}", order_info);
                        
                        result.set_text(&mut activity, &order_info)?;
                        result.set_text_color(&mut activity, 0xFF2196F3u32 as i32)?;
                    } else {
                        println!("⚠️  请完成所有选择！");
                    }
                }
            },
            _ => {}
        }
    }
}
