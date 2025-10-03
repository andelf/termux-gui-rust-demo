// Switch 开关组件演示
// 展示如何创建和使用 Switch（滑动开关）
// 运行: cargo run --example switch_demo --release

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
    println!("=== Switch 开关演示 ===\n");
    
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
        "params": {"aid": aid, "text": "智能家居控制 🏠", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 28}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 10}
    }))?;
    
    // 创建说明
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "滑动开关控制设备", "parent": layout_id}
    }))?;
    
    // 创建 Switch 1 - 客厅灯
    let switch1_id = send_and_read(&mut main_stream, &json!({
        "method": "createSwitch",
        "params": {
            "aid": aid,
            "text": "💡 客厅灯",
            "checked": true,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": switch1_id, "margin": 8}
    }))?;
    
    // 创建 Switch 2 - 空调
    let switch2_id = send_and_read(&mut main_stream, &json!({
        "method": "createSwitch",
        "params": {
            "aid": aid,
            "text": "❄️ 空调",
            "checked": false,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": switch2_id, "margin": 8}
    }))?;
    
    // 创建 Switch 3 - 加湿器
    let switch3_id = send_and_read(&mut main_stream, &json!({
        "method": "createSwitch",
        "params": {
            "aid": aid,
            "text": "💧 加湿器",
            "checked": false,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": switch3_id, "margin": 8}
    }))?;
    
    // 创建 Switch 4 - 窗帘
    let switch4_id = send_and_read(&mut main_stream, &json!({
        "method": "createSwitch",
        "params": {
            "aid": aid,
            "text": "🪟 电动窗帘",
            "checked": true,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": switch4_id, "margin": 8}
    }))?;
    
    // 创建 Switch 5 - 音乐
    let switch5_id = send_and_read(&mut main_stream, &json!({
        "method": "createSwitch",
        "params": {
            "aid": aid,
            "text": "🎵 背景音乐",
            "checked": false,
            "parent": layout_id
        }
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": switch5_id, "margin": 8}
    }))?;
    
    // 创建分隔线
    let divider_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "━━━━━━━━━━━━━━━━━━━━", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": divider_id, "margin": 10}
    }))?;
    
    // 创建状态显示
    let status_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "已开启: 客厅灯, 窗帘", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": status_id, "size": 16}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": status_id, "margin": 10}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setTextColor",
        "params": {"aid": aid, "id": status_id, "color": 0xFF4CAF50u32 as i32}
    }))?;
    
    // 创建按钮
    let button_layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": false, "parent": layout_id}
    }))?.as_i64().unwrap();
    
    let all_on_btn = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "🔆 全部开启", "parent": button_layout, "allcaps": false}
    }))?.as_i64().unwrap();
    
    let all_off_btn = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "🌙 全部关闭", "parent": button_layout, "allcaps": false}
    }))?.as_i64().unwrap();
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示:");
    println!("  • 滑动开关切换设备状态");
    println!("  • 观察状态实时更新");
    println!("  • 使用 '全部开启/关闭' 按钮");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 状态跟踪
    let mut light_on = true;
    let mut ac_on = false;
    let mut humidifier_on = false;
    let mut curtain_on = true;
    let mut music_on = false;
    
    // 更新状态显示
    let update_status = |stream: &mut UnixStream, aid: i64, status_id: i64,
                         light: bool, ac: bool, hum: bool, curt: bool, music: bool| 
                         -> Result<(), Box<dyn std::error::Error>> {
        let mut active = Vec::new();
        if light { active.push("客厅灯"); }
        if ac { active.push("空调"); }
        if hum { active.push("加湿器"); }
        if curt { active.push("窗帘"); }
        if music { active.push("音乐"); }
        
        let text = if active.is_empty() {
            "已开启: 无".to_string()
        } else {
            format!("已开启: {}", active.join(", "))
        };
        
        let count = active.len();
        let color = if count == 0 {
            0xFF9E9E9Eu32 as i32  // 灰色
        } else if count >= 4 {
            0xFFF44336u32 as i32  // 红色（能耗高）
        } else if count >= 2 {
            0xFFFF9800u32 as i32  // 橙色
        } else {
            0xFF4CAF50u32 as i32  // 绿色
        };
        
        send_message(stream, &json!({
            "method": "setText",
            "params": {"aid": aid, "id": status_id, "text": text}
        }))?;
        
        send_message(stream, &json!({
            "method": "setTextColor",
            "params": {"aid": aid, "id": status_id, "color": color}
        }))?;
        
        Ok(())
    };
    
    // 设置开关状态
    let set_switch = |stream: &mut UnixStream, aid: i64, switch_id: i64, checked: bool| 
                      -> Result<(), Box<dyn std::error::Error>> {
        send_message(stream, &json!({
            "method": "setChecked",
            "params": {"aid": aid, "id": switch_id, "checked": checked}
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
                
                if clicked_id == switch1_id {
                    light_on = is_checked;
                    println!("💡 客厅灯: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == switch2_id {
                    ac_on = is_checked;
                    println!("❄️ 空调: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == switch3_id {
                    humidifier_on = is_checked;
                    println!("💧 加湿器: {}", if is_checked { "开启" } else { "关闭" });
                } else if clicked_id == switch4_id {
                    curtain_on = is_checked;
                    println!("🪟 窗帘: {}", if is_checked { "打开" } else { "关闭" });
                } else if clicked_id == switch5_id {
                    music_on = is_checked;
                    println!("🎵 音乐: {}", if is_checked { "播放" } else { "停止" });
                } else if clicked_id == all_on_btn {
                    println!("\n🔆 全部开启");
                    light_on = true;
                    ac_on = true;
                    humidifier_on = true;
                    curtain_on = true;
                    music_on = true;
                    
                    set_switch(&mut main_stream, aid, switch1_id, true)?;
                    set_switch(&mut main_stream, aid, switch2_id, true)?;
                    set_switch(&mut main_stream, aid, switch3_id, true)?;
                    set_switch(&mut main_stream, aid, switch4_id, true)?;
                    set_switch(&mut main_stream, aid, switch5_id, true)?;
                } else if clicked_id == all_off_btn {
                    println!("\n🌙 全部关闭");
                    light_on = false;
                    ac_on = false;
                    humidifier_on = false;
                    curtain_on = false;
                    music_on = false;
                    
                    set_switch(&mut main_stream, aid, switch1_id, false)?;
                    set_switch(&mut main_stream, aid, switch2_id, false)?;
                    set_switch(&mut main_stream, aid, switch3_id, false)?;
                    set_switch(&mut main_stream, aid, switch4_id, false)?;
                    set_switch(&mut main_stream, aid, switch5_id, false)?;
                }
                
                update_status(&mut main_stream, aid, status_id, 
                            light_on, ac_on, humidifier_on, curtain_on, music_on)?;
            },
            _ => {}
        }
    }
    
    println!("=== 程序结束 ===");
    Ok(())
}
