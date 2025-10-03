// Spinner 下拉列表演示
// 展示如何创建和使用下拉选择列表
// 运行: cargo run --example spinner_demo --release

use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write, Error};
use std::process::Command;
use serde_json::{json, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::sync::{Arc, Mutex};
use std::thread;

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
    println!("=== Spinner 下拉列表演示 ===\n");
    
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
        }
        Err(_) => {
            Command::new("am")
                .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
                        "--es", "mainSocket", &addr_main, "--es", "eventSocket", &addr_event])
                .output()?;
        }
        _ => {}
    }
    
    let (mut main_stream, _) = main_listener.accept()?;
    let (mut event_stream, _) = event_listener.accept()?;
    
    // 协议握手
    main_stream.write_all(&[0x01])?;
    let mut ack = [0u8; 1];
    main_stream.read_exact(&mut ack)?;
    
    // 创建 Activity (全屏模式)
    let response = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {}
    }))?;
    let aid = response[0].as_i64().unwrap();
    
    // 创建 NestedScrollView 作为根布局（支持滚动）
    let scroll = send_and_read(&mut main_stream, &json!({
        "method": "createNestedScrollView",
        "params": {"aid": aid}
    }))?;
    let scroll_id = scroll.as_i64().unwrap();
    
    // 创建主布局（放在 ScrollView 内）
    let layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true, "parent": scroll_id}
    }))?;
    let layout_id = layout.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": layout_id, "margin": 20}
    }))?;
    
    // 标题
    let title = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "📱 手机订购向导", "parent": layout_id}
    }))?;
    let title_id = title.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 24}
    }))?;
    
    // 品牌选择
    let brand_label = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择品牌:", "parent": layout_id}
    }))?;
    let brand_label_id = brand_label.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": brand_label_id, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": brand_label_id, "margin": 10, "top": 20}
    }))?;
    
    // 品牌 Spinner
    let brands = vec!["请选择", "Apple", "Samsung", "Huawei", "Xiaomi", "OPPO", "Vivo"];
    let brand_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let brand_spinner_id = brand_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": brand_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": brand_spinner_id, "list": brands}
    }))?;
    
    // 型号选择
    let model_label = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择型号:", "parent": layout_id}
    }))?;
    let model_label_id = model_label.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": model_label_id, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": model_label_id, "margin": 10, "top": 20}
    }))?;
    
    // 型号 Spinner (初始为空)
    let model_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let model_spinner_id = model_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": model_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": model_spinner_id, "list": vec!["请先选择品牌"]}
    }))?;
    
    // 容量选择
    let storage_label = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择容量:", "parent": layout_id}
    }))?;
    let storage_label_id = storage_label.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": storage_label_id, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": storage_label_id, "margin": 10, "top": 20}
    }))?;
    
    // 容量 Spinner
    let storages = vec!["请选择", "64GB", "128GB", "256GB", "512GB", "1TB"];
    let storage_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let storage_spinner_id = storage_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": storage_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": storage_spinner_id, "list": storages}
    }))?;
    
    // 颜色选择
    let color_label = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择颜色:", "parent": layout_id}
    }))?;
    let color_label_id = color_label.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": color_label_id, "size": 18}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": color_label_id, "margin": 10, "top": 20}
    }))?;
    
    // 颜色 Spinner
    let colors = vec!["请选择", "黑色", "白色", "金色", "银色", "蓝色", "紫色", "绿色"];
    let color_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let color_spinner_id = color_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": color_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": color_spinner_id, "list": colors}
    }))?;
    
    // 结果显示
    let result = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "请完成选择", "parent": layout_id}
    }))?;
    let result_id = result.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": result_id, "size": 16}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setTextColor",
        "params": {"aid": aid, "id": result_id, "color": 0xFF666666_u32}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": result_id, "margin": 10, "top": 30}
    }))?;
    
    // 提交按钮
    let submit_btn = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "🛒 确认订购", "parent": layout_id}
    }))?;
    let submit_btn_id = submit_btn.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": submit_btn_id, "margin": 10, "top": 20}
    }))?;
    
    println!("✓ 连接建立");
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示: 从下拉列表中选择");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 选择状态
    let selections = Arc::new(Mutex::new(vec![
        ("品牌".to_string(), "".to_string()),
        ("型号".to_string(), "".to_string()),
        ("容量".to_string(), "".to_string()),
        ("颜色".to_string(), "".to_string()),
    ]));
    
    let main_stream = Arc::new(Mutex::new(main_stream));
    let main_clone = Arc::clone(&main_stream);
    let selections_clone = Arc::clone(&selections);
    
    // 事件监听线程
    thread::spawn(move || {
        loop {
            match read_message(&mut event_stream) {
                Ok(event) => {
                    let event_type = event["type"].as_str().unwrap_or("");
                    
                    if event_type == "destroy" {
                        println!("\n👋 程序退出");
                        std::process::exit(0);
                    }
                    
                    if event_type == "itemselected" {
                        let view_id = event["value"]["id"].as_i64().unwrap_or(-1);
                        let index = event["value"]["index"].as_i64().unwrap_or(0) as usize;
                        
                        let mut sels = selections_clone.lock().unwrap();
                        let mut main = main_clone.lock().unwrap();
                        
                        if view_id == brand_spinner_id {
                            // 品牌选择
                            if index > 0 {
                                let brand = brands[index];
                                sels[0].1 = brand.to_string();
                                println!("📱 品牌: {}", brand);
                                
                                // 根据品牌更新型号列表
                                let models: Vec<&str> = match brand {
                                    "Apple" => vec!["请选择", "iPhone 15 Pro Max", "iPhone 15 Pro", "iPhone 15", "iPhone 14"],
                                    "Samsung" => vec!["请选择", "Galaxy S24 Ultra", "Galaxy S24+", "Galaxy S24", "Galaxy Z Fold5"],
                                    "Huawei" => vec!["请选择", "Mate 60 Pro", "Mate 60", "P60 Pro", "P60"],
                                    "Xiaomi" => vec!["请选择", "14 Ultra", "14 Pro", "14", "13T Pro"],
                                    "OPPO" => vec!["请选择", "Find X7 Ultra", "Find X7", "Reno 11 Pro", "Reno 11"],
                                    "Vivo" => vec!["请选择", "X100 Pro", "X100", "S18 Pro", "S18"],
                                    _ => vec!["请选择"],
                                };
                                
                                // 更新型号 Spinner
                                let _ = send_message(&mut *main, &json!({
                                    "method": "setList",
                                    "params": {"aid": aid, "id": model_spinner_id, "list": models}
                                }));
                                
                                // 重置型号选择
                                sels[1].1 = "".to_string();
                            } else {
                                sels[0].1 = "".to_string();
                            }
                        } else if view_id == model_spinner_id {
                            // 型号选择
                            if index > 0 {
                                let models_list = match sels[0].1.as_str() {
                                    "Apple" => vec!["", "iPhone 15 Pro Max", "iPhone 15 Pro", "iPhone 15", "iPhone 14"],
                                    "Samsung" => vec!["", "Galaxy S24 Ultra", "Galaxy S24+", "Galaxy S24", "Galaxy Z Fold5"],
                                    "Huawei" => vec!["", "Mate 60 Pro", "Mate 60", "P60 Pro", "P60"],
                                    "Xiaomi" => vec!["", "14 Ultra", "14 Pro", "14", "13T Pro"],
                                    "OPPO" => vec!["", "Find X7 Ultra", "Find X7", "Reno 11 Pro", "Reno 11"],
                                    "Vivo" => vec!["", "X100 Pro", "X100", "S18 Pro", "S18"],
                                    _ => vec![""],
                                };
                                
                                if index < models_list.len() {
                                    let model = models_list[index];
                                    sels[1].1 = model.to_string();
                                    println!("📱 型号: {}", model);
                                }
                            } else {
                                sels[1].1 = "".to_string();
                            }
                        } else if view_id == storage_spinner_id {
                            // 容量选择
                            if index > 0 {
                                let storage = storages[index];
                                sels[2].1 = storage.to_string();
                                println!("💾 容量: {}", storage);
                            } else {
                                sels[2].1 = "".to_string();
                            }
                        } else if view_id == color_spinner_id {
                            // 颜色选择
                            if index > 0 {
                                let color = colors[index];
                                sels[3].1 = color.to_string();
                                println!("🎨 颜色: {}", color);
                            } else {
                                sels[3].1 = "".to_string();
                            }
                        }
                        
                        // 更新结果显示
                        let all_selected = sels.iter().all(|(_, v)| !v.is_empty());
                        let result_text = if all_selected {
                            format!("✅ 已选择:\n品牌: {}\n型号: {}\n容量: {}\n颜色: {}",
                                sels[0].1, sels[1].1, sels[2].1, sels[3].1)
                        } else {
                            let missing: Vec<&str> = sels.iter()
                                .filter(|(_, v)| v.is_empty())
                                .map(|(k, _)| k.as_str())
                                .collect();
                            format!("⚠️ 待选择: {}", missing.join(", "))
                        };
                        
                        let _ = send_message(&mut *main, &json!({
                            "method": "setText",
                            "params": {"aid": aid, "id": result_id, "text": result_text}
                        }));
                        
                        // 更新颜色
                        let color = if all_selected { 0xFF4CAF50_u32 } else { 0xFF666666_u32 };
                        let _ = send_message(&mut *main, &json!({
                            "method": "setTextColor",
                            "params": {"aid": aid, "id": result_id, "color": color}
                        }));
                    }
                    
                    if event_type == "click" {
                        let view_id = event["value"]["id"].as_i64().unwrap_or(-1);
                        
                        if view_id == submit_btn_id {
                            let sels = selections_clone.lock().unwrap();
                            let all_selected = sels.iter().all(|(_, v)| !v.is_empty());
                            
                            if all_selected {
                                let order_info = format!(
                                    "🎉 订购成功！\n\n品牌: {}\n型号: {}\n容量: {}\n颜色: {}\n\n感谢您的订购！",
                                    sels[0].1, sels[1].1, sels[2].1, sels[3].1
                                );
                                
                                println!("\n{}", order_info);
                                
                                let mut main = main_clone.lock().unwrap();
                                let _ = send_message(&mut *main, &json!({
                                    "method": "setText",
                                    "params": {"aid": aid, "id": result_id, "text": order_info}
                                }));
                                
                                let _ = send_message(&mut *main, &json!({
                                    "method": "setTextColor",
                                    "params": {"aid": aid, "id": result_id, "color": 0xFF2196F3_u32}
                                }));
                            } else {
                                println!("⚠️  请完成所有选择！");
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("事件读取错误: {}", e);
                    break;
                }
            }
        }
    });
    
    // 主线程等待
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
