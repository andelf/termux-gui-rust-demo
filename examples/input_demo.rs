// 输入框和按钮交互演示
// 展示 EditText 输入、Button 点击、TextView 显示
// 运行: cargo run --example input_demo --release

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
    println!("=== 输入框交互演示 ===\n");
    
    // 建立连接
    let addr_main = generate_random_address();
    let addr_event = generate_random_address();
    let main_listener = bind_abstract_socket(&addr_main)?;
    let event_listener = bind_abstract_socket(&addr_event)?;
    
    // 尝试使用 termux-am，失败则使用 am
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
    
    // 协议握手
    main_stream.write_all(&[0x01])?;
    main_stream.read_exact(&mut [0u8; 1])?;
    
    println!("✓ 连接建立\n");
    
    // 创建对话框模式的 Activity
    let aid = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {"dialog": true, "canceloutside": false}
    }))?[0].as_i64().unwrap();
    
    // 创建垂直布局
    let layout_id = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true}
    }))?.as_i64().unwrap();
    
    // 创建标题
    let title_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "文本输入演示 📝", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 28}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 10}
    }))?;
    
    // 创建说明文本
    let desc_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "在下方输入你的名字:", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": desc_id, "margin": 5}
    }))?;
    
    // 创建输入框 1 - 姓名
    let name_input_id = send_and_read(&mut main_stream, &json!({
        "method": "createEditText",
        "params": {
            "aid": aid,
            "text": "",
            "parent": layout_id,
            "singleline": true,
            "line": true,
            "blockinput": false,
            "type": "text"
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": name_input_id, "margin": 5}
    }))?;
    
    // 创建说明文本2
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "输入一个数字:", "parent": layout_id}
    }))?;
    
    // 创建输入框 2 - 数字
    let number_input_id = send_and_read(&mut main_stream, &json!({
        "method": "createEditText",
        "params": {
            "aid": aid,
            "text": "0",
            "parent": layout_id,
            "singleline": true,
            "line": true,
            "blockinput": false,
            "type": "number"
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": number_input_id, "margin": 5}
    }))?;
    
    // 创建多行文本框
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "输入多行消息:", "parent": layout_id}
    }))?;
    
    let message_input_id = send_and_read(&mut main_stream, &json!({
        "method": "createEditText",
        "params": {
            "aid": aid,
            "text": "",
            "parent": layout_id,
            "singleline": false,
            "line": true,
            "blockinput": false,
            "type": "textMultiLine"
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": message_input_id, "margin": 5}
    }))?;
    
    // 创建按钮布局（水平）
    let button_layout_id = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": false, "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": button_layout_id, "margin": 10}
    }))?;
    
    // 创建提交按钮
    let submit_button_id = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "✅ 提交", "parent": button_layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    // 创建清空按钮
    let clear_button_id = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "🗑️ 清空", "parent": button_layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    // 创建测试按钮
    let test_button_id = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "🧪 测试", "parent": button_layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    // 创建分隔线
    let divider_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "━━━━━━━━━━━━━━━━━━━━", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": divider_id, "margin": 10}
    }))?;
    
    // 创建结果显示区域
    let result_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "结果将显示在这里...", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": result_id, "size": 16}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": result_id, "margin": 10}
    }))?;
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 在输入框中输入内容");
    println!("  • 点击 '提交' 查看输入的内容");
    println!("  • 点击 '清空' 清除所有输入");
    println!("  • 点击 '测试' 填充测试数据");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件处理循环
    loop {
        let event = read_message(&mut event_stream)?;
        let event_type = event["type"].as_str().unwrap_or("");
        let event_value = &event["value"];
        
        match event_type {
            "destroy" => {
                // 只要收到 destroy 就退出，不检查 finishing
                // 这样可以避免界面关闭但程序还在等待的问题
                println!("\n✓ Activity 已关闭");
                break;
            },
            "click" => {
                let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
                
                if clicked_id == submit_button_id {
                    println!("📨 提交按钮被点击");
                    
                    // 获取姓名输入框的文本
                    let name_response = send_and_read(&mut main_stream, &json!({
                        "method": "getText",
                        "params": {"aid": aid, "id": name_input_id}
                    }))?;
                    let name = name_response.as_str().unwrap_or("");
                    
                    // 获取数字输入框的文本
                    let number_response = send_and_read(&mut main_stream, &json!({
                        "method": "getText",
                        "params": {"aid": aid, "id": number_input_id}
                    }))?;
                    let number_str = number_response.as_str().unwrap_or("0");
                    let number: i32 = number_str.parse().unwrap_or(0);
                    
                    // 获取多行消息
                    let message_response = send_and_read(&mut main_stream, &json!({
                        "method": "getText",
                        "params": {"aid": aid, "id": message_input_id}
                    }))?;
                    let message = message_response.as_str().unwrap_or("");
                    
                    // 构建结果文本
                    let result_text = if name.is_empty() {
                        "⚠️ 请输入姓名！".to_string()
                    } else {
                        let doubled = number * 2;
                        let msg_part = if message.is_empty() {
                            String::new()
                        } else {
                            format!("\n\n消息:\n{}", message)
                        };
                        
                        format!(
                            "✅ 提交成功！\n\n\
                            姓名: {}\n\
                            数字: {} (x2 = {}){}",
                            name, number, doubled, msg_part
                        )
                    };
                    
                    // 更新结果显示
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": result_id, "text": result_text}
                    }))?;
                    
                    // 根据结果改变颜色
                    let color = if name.is_empty() {
                        0xFFF44336u32 as i32  // 红色（错误）
                    } else {
                        0xFF4CAF50u32 as i32  // 绿色（成功）
                    };
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setTextColor",
                        "params": {"aid": aid, "id": result_id, "color": color}
                    }))?;
                    
                    println!("   姓名: {}", name);
                    println!("   数字: {}", number);
                    println!("   消息: {}", if message.is_empty() { "(空)" } else { message });
                    
                } else if clicked_id == clear_button_id {
                    println!("🗑️ 清空按钮被点击");
                    
                    // 清空所有输入框
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": name_input_id, "text": ""}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": number_input_id, "text": "0"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": message_input_id, "text": ""}
                    }))?;
                    
                    // 清空结果显示
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": result_id, "text": "已清空所有输入"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setTextColor",
                        "params": {"aid": aid, "id": result_id, "color": 0xFF9E9E9Eu32 as i32}
                    }))?;
                    
                } else if clicked_id == test_button_id {
                    println!("🧪 测试按钮被点击");
                    
                    // 填充测试数据
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": name_input_id, "text": "张三"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": number_input_id, "text": "42"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": message_input_id, "text": "这是一条测试消息。\n使用Rust编写的Termux:GUI应用！"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": result_id, "text": "✅ 已填充测试数据\n点击 '提交' 按钮查看结果"}
                    }))?;
                    
                    send_message(&mut main_stream, &json!({
                        "method": "setTextColor",
                        "params": {"aid": aid, "id": result_id, "color": 0xFF2196F3u32 as i32}
                    }))?;
                }
            },
            "text" => {
                // EditText 文本变化事件（可选）
                let view_id = event_value["id"].as_i64().unwrap_or(-1);
                let text = event_value["text"].as_str().unwrap_or("");
                
                if view_id == name_input_id {
                    println!("📝 姓名输入框内容变化: {}", text);
                } else if view_id == number_input_id {
                    println!("🔢 数字输入框内容变化: {}", text);
                }
            },
            _ => {}
        }
    }
    
    println!("=== 程序结束 ===");
    Ok(())
}
