// Checkbox 组件演示
// 展示如何创建和使用 Checkbox（复选框）
// 运行: cargo run --example checkbox_demo --release

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
    println!("=== Checkbox 复选框演示 ===\n");
    
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
        "params": {"aid": aid, "text": "选择你喜欢的功能 ✅", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 26}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 10}
    }))?;
    
    // 创建 Checkbox 1 - WiFi
    let checkbox1_id = send_and_read(&mut main_stream, &json!({
        "method": "createCheckbox",
        "params": {
            "aid": aid,
            "text": "📶 WiFi",
            "checked": false,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": checkbox1_id, "margin": 5}
    }))?;
    
    // 创建 Checkbox 2 - 蓝牙
    let checkbox2_id = send_and_read(&mut main_stream, &json!({
        "method": "createCheckbox",
        "params": {
            "aid": aid,
            "text": "📡 蓝牙",
            "checked": true,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": checkbox2_id, "margin": 5}
    }))?;
    
    // 创建 Checkbox 3 - 定位
    let checkbox3_id = send_and_read(&mut main_stream, &json!({
        "method": "createCheckbox",
        "params": {
            "aid": aid,
            "text": "📍 定位服务",
            "checked": false,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": checkbox3_id, "margin": 5}
    }))?;
    
    // 创建 Checkbox 4 - 通知
    let checkbox4_id = send_and_read(&mut main_stream, &json!({
        "method": "createCheckbox",
        "params": {
            "aid": aid,
            "text": "🔔 通知",
            "checked": true,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": checkbox4_id, "margin": 5}
    }))?;
    
    // 创建分隔线
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "━━━━━━━━━━━━━━━━━━━━", "parent": layout_id}
    }))?;
    
    // 创建状态显示
    let status_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "当前选中: 蓝牙, 通知", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": status_id, "margin": 10}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setTextColor",
        "params": {"aid": aid, "id": status_id, "color": 0xFF2196F3u32 as i32}
    }))?;
    
    // 创建按钮
    let apply_button_id = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "✅ 应用设置", "parent": layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": apply_button_id, "margin": 10}
    }))?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 点击复选框切换状态");
    println!("  • 观察状态实时更新");
    println!("  • 点击 '应用设置' 查看最终选择");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 状态跟踪
    let mut wifi_checked = false;
    let mut bluetooth_checked = true;
    let mut location_checked = false;
    let mut notification_checked = true;
    
    // 更新状态显示的函数
    let update_status = |stream: &mut UnixStream, aid: i64, status_id: i64, 
                         wifi: bool, bt: bool, loc: bool, notif: bool| -> Result<(), Box<dyn std::error::Error>> {
        let mut selected = Vec::new();
        if wifi { selected.push("WiFi"); }
        if bt { selected.push("蓝牙"); }
        if loc { selected.push("定位"); }
        if notif { selected.push("通知"); }
        
        let text = if selected.is_empty() {
            "当前选中: 无".to_string()
        } else {
            format!("当前选中: {}", selected.join(", "))
        };
        
        send_message(stream, &json!({
            "method": "setText",
            "params": {"aid": aid, "id": status_id, "text": text}
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
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                let is_checked = event_value["set"].as_bool().unwrap_or(false);
                
                if clicked_id == checkbox1_id {
                    wifi_checked = is_checked;
                    println!("📶 WiFi: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == checkbox2_id {
                    bluetooth_checked = is_checked;
                    println!("📡 蓝牙: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == checkbox3_id {
                    location_checked = is_checked;
                    println!("📍 定位: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == checkbox4_id {
                    notification_checked = is_checked;
                    println!("🔔 通知: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == apply_button_id {
                    println!("\n✅ 应用设置:");
                    println!("  WiFi: {}", if wifi_checked { "✓" } else { "✗" });
                    println!("  蓝牙: {}", if bluetooth_checked { "✓" } else { "✗" });
                    println!("  定位: {}", if location_checked { "✓" } else { "✗" });
                    println!("  通知: {}", if notification_checked { "✓" } else { "✗" });
                    
                    // 显示确认消息
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": status_id, "text": "✅ 设置已应用！"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setTextColor",
                        "params": {"aid": aid, "id": status_id, "color": 0xFF4CAF50u32 as i32}
                    }))?;
                    
                    continue;
                }
                
                // 更新状态显示
                update_status(&mut main_stream, aid, status_id, 
                            wifi_checked, bluetooth_checked, location_checked, notification_checked)?;
            },
            _ => {}
        }
    }
    
    println!("=== 程序结束 ===");
    Ok(())
}
