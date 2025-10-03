// RadioGroup + RadioButton 单选组演示
// 展示如何创建和使用单选按钮组
// 运行: cargo run --example radio_demo --release

use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write, Error};
use std::process::Command;
use serde_json::{json, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn generate_random_address() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(50).map(char::from).collect()
}

fn bind_abstract_socket(name: &str) -> Result<UnixListener, Error> {
    unsafe {
        use std::os::unix::io::FromRawFd;
        let fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
        if fd < 0 { return Err(Error::last_os_error()); }
        
        let mut addr_bytes = vec![0u8];
        addr_bytes.extend_from_slice(name.as_bytes());
        
        let mut addr: libc::sockaddr_un = std::mem::zeroed();
        addr.sun_family = libc::AF_UNIX as u16;
        for (i, &byte) in addr_bytes.iter().enumerate() {
            addr.sun_path[i] = byte as _;
        }
        
        let addr_len = (std::mem::size_of::<libc::sa_family_t>() + addr_bytes.len()) as libc::socklen_t;
        if libc::bind(fd, &addr as *const _ as *const libc::sockaddr, addr_len) < 0 {
            libc::close(fd);
            return Err(Error::last_os_error());
        }
        if libc::listen(fd, 1) < 0 {
            libc::close(fd);
            return Err(Error::last_os_error());
        }
        Ok(UnixListener::from_raw_fd(fd))
    }
}

fn send_message(stream: &mut UnixStream, msg: &Value) -> Result<(), Error> {
    let json_bytes = msg.to_string().into_bytes();
    stream.write_all(&(json_bytes.len() as u32).to_be_bytes())?;
    stream.write_all(&json_bytes)?;
    stream.flush()
}

fn read_message(stream: &mut UnixStream) -> Result<Value, Error> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let mut buf = vec![0u8; u32::from_be_bytes(len_buf) as usize];
    stream.read_exact(&mut buf)?;
    Ok(serde_json::from_slice(&buf).unwrap())
}

fn send_and_read(stream: &mut UnixStream, msg: &Value) -> Result<Value, Error> {
    send_message(stream, msg)?;
    read_message(stream)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RadioGroup + RadioButton 单选演示 ===\n");
    
    // 建立连接
    let addr_main = generate_random_address();
    let addr_event = generate_random_address();
    let main_listener = bind_abstract_socket(&addr_main)?;
    let event_listener = bind_abstract_socket(&addr_event)?;
    
    let output = Command::new("termux-am")
        .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
                "--es", "mainSocket", &addr_main, "--es", "eventSocket", &addr_event])
        .output();
    
    match output {
        Ok(out) if !out.status.success() => {
            Command::new("am")
                .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
                        "--es", "mainSocket", &addr_main, "--es", "eventSocket", &addr_event])
                .output()?;
        },
        Err(_) => {
            Command::new("am")
                .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
                        "--es", "mainSocket", &addr_main, "--es", "eventSocket", &addr_event])
                .output()?;
        },
        _ => {}
    }
    
    let (mut main_stream, _) = main_listener.accept()?;
    let (mut event_stream, _) = event_listener.accept()?;
    
    main_stream.write_all(&[0x01])?;
    main_stream.read_exact(&mut [0u8; 1])?;
    
    println!("✓ 连接建立\n");
    
    let aid = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {"dialog": true, "canceloutside": false}
    }))?[0].as_i64().unwrap();
    
    let layout_id = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true}
    }))?.as_i64().unwrap();
    
    // 创建标题
    let title_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "📦 选择配送方式", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 26}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 10}
    }))?;
    
    // 第一个 RadioGroup - 配送方式
    let section1 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "配送方式：", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": section1, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": section1, "margin": 8}
    }))?;
    
    let radio_group1 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioGroup",
        "params": {"aid": aid, "parent": layout_id}
    }))?.as_i64().unwrap();
    
    // RadioButton 1.1 - 标准配送
    let radio1_1 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "📮 标准配送 (免费, 3-5天)",
            "parent": radio_group1,
            "checked": true
        }
    }))?.as_i64().unwrap();
    
    // RadioButton 1.2 - 快速配送
    let radio1_2 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "🚚 快速配送 (¥15, 1-2天)",
            "parent": radio_group1,
            "checked": false
        }
    }))?.as_i64().unwrap();
    
    // RadioButton 1.3 - 当日达
    let radio1_3 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "⚡ 当日达 (¥30, 当天送达)",
            "parent": radio_group1,
            "checked": false
        }
    }))?.as_i64().unwrap();
    
    // 分隔线
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "━━━━━━━━━━━━━━━━━━━━", "parent": layout_id}
    }))?;
    
    // 第二个 RadioGroup - 支付方式
    let section2 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "支付方式：", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": section2, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": section2, "margin": 8}
    }))?;
    
    let radio_group2 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioGroup",
        "params": {"aid": aid, "parent": layout_id}
    }))?.as_i64().unwrap();
    
    // RadioButton 2.1 - 微信支付
    let radio2_1 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "💚 微信支付",
            "parent": radio_group2,
            "checked": true
        }
    }))?.as_i64().unwrap();
    
    // RadioButton 2.2 - 支付宝
    let radio2_2 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "💙 支付宝",
            "parent": radio_group2,
            "checked": false
        }
    }))?.as_i64().unwrap();
    
    // RadioButton 2.3 - 货到付款
    let radio2_3 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "💰 货到付款",
            "parent": radio_group2,
            "checked": false
        }
    }))?.as_i64().unwrap();
    
    // 分隔线
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "━━━━━━━━━━━━━━━━━━━━", "parent": layout_id}
    }))?;
    
    // 第三个 RadioGroup - 发票类型
    let section3 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "发票类型：", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": section3, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": section3, "margin": 8}
    }))?;
    
    let radio_group3 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioGroup",
        "params": {"aid": aid, "parent": layout_id}
    }))?.as_i64().unwrap();
    
    // RadioButton 3.1 - 不需要
    let radio3_1 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "❌ 不需要发票",
            "parent": radio_group3,
            "checked": true
        }
    }))?.as_i64().unwrap();
    
    // RadioButton 3.2 - 电子发票
    let radio3_2 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "📧 电子发票",
            "parent": radio_group3,
            "checked": false
        }
    }))?.as_i64().unwrap();
    
    // RadioButton 3.3 - 纸质发票
    let radio3_3 = send_and_read(&mut main_stream, &json!({
        "method": "createRadioButton",
        "params": {
            "aid": aid,
            "text": "📄 纸质发票 (+¥5)",
            "parent": radio_group3,
            "checked": false
        }
    }))?.as_i64().unwrap();
    
    // 分隔线
    let divider_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "━━━━━━━━━━━━━━━━━━━━", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": divider_id, "margin": 8}
    }))?;
    
    // 状态显示
    let status_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {
            "aid": aid,
            "text": "当前选择:\n配送: 标准配送\n支付: 微信支付\n发票: 不需要发票",
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": status_id, "margin": 10}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setTextColor",
        "params": {"aid": aid, "id": status_id, "color": 0xFF2196F3u32 as i32}
    }))?;
    
    // 总价显示
    let price_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "总计: ¥0", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": price_id, "size": 22}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": price_id, "margin": 10}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setTextColor",
        "params": {"aid": aid, "id": price_id, "color": 0xFFFF5722u32 as i32}
    }))?;
    
    // 确认按钮
    let confirm_btn = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "✅ 确认订单", "parent": layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": confirm_btn, "margin": 10}
    }))?;
    
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
    
    // 更新显示
    let update_display = |stream: &mut UnixStream, aid: i64, status_id: i64, price_id: i64,
                          del_idx: usize, pay_idx: usize, inv_idx: usize|
                          -> Result<(), Box<dyn std::error::Error>> {
        let status_text = format!(
            "当前选择:\n配送: {}\n支付: {}\n发票: {}",
            delivery_options[del_idx],
            payment_options[pay_idx],
            invoice_options[inv_idx]
        );
        
        send_message(stream, &json!({
            "method": "setText",
            "params": {"aid": aid, "id": status_id, "text": status_text}
        }))?;
        
        let total = delivery_prices[del_idx] + invoice_prices[inv_idx];
        let price_text = format!("总计: ¥{}", total);
        
        send_message(stream, &json!({
            "method": "setText",
            "params": {"aid": aid, "id": price_id, "text": price_text}
        }))?;
        
        Ok(())
    };
    
    // 事件处理
    loop {
        let event = read_message(&mut event_stream)?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                break;
            },
            "selected" => {
                let group_id = event_value["id"].as_i64().unwrap_or(-1);
                let selected_id = event_value["selected"].as_i64().unwrap_or(-1);
                
                if group_id == radio_group1 {
                    // 配送方式组
                    if selected_id == radio1_1 {
                        delivery_index = 0;
                        println!("📮 选择: 标准配送");
                    } else if selected_id == radio1_2 {
                        delivery_index = 1;
                        println!("🚚 选择: 快速配送 (+¥15)");
                    } else if selected_id == radio1_3 {
                        delivery_index = 2;
                        println!("⚡ 选择: 当日达 (+¥30)");
                    }
                } else if group_id == radio_group2 {
                    // 支付方式组
                    if selected_id == radio2_1 {
                        payment_index = 0;
                        println!("💚 选择: 微信支付");
                    } else if selected_id == radio2_2 {
                        payment_index = 1;
                        println!("💙 选择: 支付宝");
                    } else if selected_id == radio2_3 {
                        payment_index = 2;
                        println!("💰 选择: 货到付款");
                    }
                } else if group_id == radio_group3 {
                    // 发票类型组
                    if selected_id == radio3_1 {
                        invoice_index = 0;
                        println!("❌ 选择: 不需要发票");
                    } else if selected_id == radio3_2 {
                        invoice_index = 1;
                        println!("📧 选择: 电子发票");
                    } else if selected_id == radio3_3 {
                        invoice_index = 2;
                        println!("📄 选择: 纸质发票 (+¥5)");
                    }
                }
                
                update_display(&mut main_stream, aid, status_id, price_id,
                             delivery_index, payment_index, invoice_index)?;
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == confirm_btn {
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
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": status_id, "text": "✅ 订单已确认！"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setTextColor",
                        "params": {"aid": aid, "id": status_id, "color": 0xFF4CAF50u32 as i32}
                    }))?;
                }
            },
            _ => {}
        }
    }
    
    println!("=== 程序结束 ===");
    Ok(())
}
