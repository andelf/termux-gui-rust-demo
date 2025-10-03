use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
use serde_json::{json, Value};

fn generate_random_address() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("termux_gui_{}", timestamp)
}

fn send_message(stream: &mut UnixStream, msg: &Value) -> std::io::Result<()> {
    let json_str = msg.to_string();
    let len = json_str.len() as u32;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(json_str.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn read_message(stream: &mut UnixStream) -> std::io::Result<Value> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;
    let len = u32::from_be_bytes(len_bytes) as usize;
    
    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer)?;
    
    let json_str = String::from_utf8_lossy(&buffer);
    Ok(serde_json::from_str(&json_str).unwrap_or(Value::Null))
}

fn send_and_read(stream: &mut UnixStream, msg: &Value) -> std::io::Result<Value> {
    send_message(stream, msg)?;
    read_message(stream)
}

fn main() -> std::io::Result<()> {
    println!("=== 🔍 Spinner 级联调试版本 ===\n");
    
    let addr_main = generate_random_address();
    let addr_event = generate_random_address();
    
    println!("📡 监听地址:");
    println!("  Main:  {}", addr_main);
    println!("  Event: {}", addr_event);
    
    let main_listener = UnixListener::bind(format!("\0{}", addr_main))?;
    let event_listener = UnixListener::bind(format!("\0{}", addr_event))?;
    
    println!("\n📢 发送广播...");
    let output = Command::new("termux-am")
        .args(&[
            "broadcast",
            "-n", "com.termux.gui/.GUIReceiver",
            "--es", "mainSocket", &addr_main,
            "--es", "eventSocket", &addr_event,
        ])
        .output()?;
    
    if !output.status.success() {
        eprintln!("❌ 广播失败: {:?}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    
    println!("✓ 等待连接...");
    let (mut main_stream, _) = main_listener.accept()?;
    let (mut event_stream, _) = event_listener.accept()?;
    println!("✓ 连接已建立\n");
    
    // 创建 Activity
    println!("🏗️ 创建界面...");
    let response = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {}
    }))?;
    let aid = response.as_i64().unwrap();
    println!("✓ Activity ID: {}", aid);
    
    // 创建 NestedScrollView
    let scroll = send_and_read(&mut main_stream, &json!({
        "method": "createNestedScrollView",
        "params": {"aid": aid}
    }))?;
    let scroll_id = scroll.as_i64().unwrap();
    println!("✓ ScrollView ID: {}", scroll_id);
    
    // 创建 LinearLayout
    let layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "parent": scroll_id}
    }))?;
    let layout_id = layout.as_i64().unwrap();
    println!("✓ Layout ID: {}", layout_id);
    
    send_message(&mut main_stream, &json!({
        "method": "setLinearLayoutParams",
        "params": {"aid": aid, "id": layout_id, "weight": 0}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": layout_id, "margin": 20}
    }))?;
    
    // 标题
    let title = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "📱 品牌型号级联测试", "parent": layout_id}
    }))?;
    let title_id = title.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 24}
    }))?;
    
    // 品牌标签
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
    let brands = vec!["请选择", "Apple", "Samsung", "Xiaomi"];
    let brand_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let brand_spinner_id = brand_spinner.as_i64().unwrap();
    println!("✓ 品牌 Spinner ID: {}", brand_spinner_id);
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": brand_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": brand_spinner_id, "list": brands}
    }))?;
    
    // 型号标签
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
    
    // 型号 Spinner
    let model_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let model_spinner_id = model_spinner.as_i64().unwrap();
    println!("✓ 型号 Spinner ID: {}", model_spinner_id);
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": model_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": model_spinner_id, "list": vec!["请先选择品牌"]}
    }))?;
    
    // 结果显示
    let result = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "等待选择...", "parent": layout_id}
    }))?;
    let result_id = result.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": result_id, "size": 16}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": result_id, "margin": 10, "top": 30}
    }))?;
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ 界面创建完成");
    println!("请在界面上选择品牌");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 事件循环
    let main_clone = Arc::new(Mutex::new(main_stream));
    let selections = Arc::new(Mutex::new(["".to_string(), "".to_string()]));
    
    thread::spawn({
        let main_clone = Arc::clone(&main_clone);
        let selections = Arc::clone(&selections);
        
        move || {
            loop {
                let event = match read_message(&mut event_stream) {
                    Ok(e) => e,
                    Err(_) => break,
                };
                
                let event_type = event["type"].as_str().unwrap_or("");
                
                if event_type == "destroy" {
                    println!("\n👋 程序退出");
                    std::process::exit(0);
                }
                
                if event_type == "itemselected" {
                    let view_id = event["value"]["id"].as_i64().unwrap_or(-1);
                    let selected = event["value"]["selected"].as_str().unwrap_or("");
                    let index = event["value"]["index"].as_i64().unwrap_or(0) as usize;
                    
                    println!("\n🔔 收到 itemselected 事件:");
                    println!("  view_id: {}", view_id);
                    println!("  selected: {}", selected);
                    println!("  index: {}", index);
                    println!("  brand_spinner_id: {}", brand_spinner_id);
                    println!("  model_spinner_id: {}", model_spinner_id);
                    
                    let mut sels = selections.lock().unwrap();
                    let mut main = main_clone.lock().unwrap();
                    
                    if view_id == brand_spinner_id {
                        println!("✓ 识别为品牌 Spinner");
                        
                        if index > 0 {
                            let brand = brands[index];
                            sels[0] = brand.to_string();
                            println!("📱 选择品牌: {}", brand);
                            
                            // 根据品牌更新型号列表
                            let models: Vec<&str> = match brand {
                                "Apple" => vec!["请选择", "iPhone 15 Pro Max", "iPhone 15 Pro", "iPhone 15"],
                                "Samsung" => vec!["请选择", "Galaxy S24 Ultra", "Galaxy S24+", "Galaxy S24"],
                                "Xiaomi" => vec!["请选择", "14 Ultra", "14 Pro", "14"],
                                _ => vec!["请选择"],
                            };
                            
                            println!("🔄 更新型号列表: {:?}", models);
                            
                            // 更新型号 Spinner
                            let update_msg = json!({
                                "method": "setList",
                                "params": {"aid": aid, "id": model_spinner_id, "list": models}
                            });
                            println!("📤 发送 setList 消息: {}", update_msg);
                            
                            if let Err(e) = send_message(&mut *main, &update_msg) {
                                println!("❌ setList 失败: {:?}", e);
                            } else {
                                println!("✓ setList 成功发送");
                            }
                            
                            // 重置型号选择
                            sels[1] = "".to_string();
                            
                            // 更新结果显示
                            let _ = send_message(&mut *main, &json!({
                                "method": "setText",
                                "params": {"aid": aid, "id": result_id, "text": format!("品牌: {}\n请选择型号", brand)}
                            }));
                        } else {
                            println!("⚠️  选择了第一项（请选择）");
                            sels[0] = "".to_string();
                        }
                    } else if view_id == model_spinner_id {
                        println!("✓ 识别为型号 Spinner");
                        
                        if index > 0 {
                            sels[1] = selected.to_string();
                            println!("📱 选择型号: {}", selected);
                            
                            // 更新结果显示
                            let _ = send_message(&mut *main, &json!({
                                "method": "setText",
                                "params": {"aid": aid, "id": result_id, "text": format!("✅ 品牌: {}\n✅ 型号: {}", sels[0], sels[1])}
                            }));
                        } else {
                            println!("⚠️  选择了第一项");
                            sels[1] = "".to_string();
                        }
                    } else {
                        println!("⚠️  未知的 Spinner ID: {}", view_id);
                    }
                }
            }
        }
    });
    
    // 主线程等待
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
