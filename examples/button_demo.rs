// Button 交互式演示 - 完整示例
// 展示如何创建按钮、布局和处理点击事件
// 运行: cargo run --example button_demo

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
    println!("=== Button 交互演示 ===\n");
    
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
    
    main_stream.write_all(&[0x01])?;
    main_stream.read_exact(&mut [0u8; 1])?;
    
    let aid = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {"dialog": true, "canceloutside": false}
    }))?[0].as_i64().unwrap();
    
    println!("✓ 连接建立\n");
    
    let layout_id = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true}
    }))?.as_i64().unwrap();
    
    let title_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "计数器演示 🦀", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 30}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 10}
    }))?;
    
    let counter_id = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "点击次数: 0", "parent": layout_id}
    }))?.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": counter_id, "size": 24}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": counter_id, "margin": 20}
    }))?;
    
    let button_layout_id = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": false, "parent": layout_id}
    }))?.as_i64().unwrap();
    
    let inc_button = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "➕ 增加", "parent": button_layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    let dec_button = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "➖ 减少", "parent": button_layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    let reset_button = send_and_read(&mut main_stream, &json!({
        "method": "createButton",
        "params": {"aid": aid, "text": "🔄 重置", "parent": layout_id, "allcaps": false}
    }))?.as_i64().unwrap();
    
    println!("✓ 界面创建完成\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("提示: 点击按钮进行交互");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let mut count = 0;
    
    loop {
        let event = read_message(&mut event_stream)?;
        let event_type = event["type"].as_str().unwrap_or("");
        
        match event_type {
            "destroy" => {
                // 只要收到 destroy 就退出，不检查 finishing
                // 这样可以避免界面关闭但程序还在等待的问题
                println!("\n✓ Activity 已关闭");
                break;
            },
            "click" => {
                let id = event["value"]["id"].as_i64().unwrap_or(-1);
                if id == inc_button {
                    count += 1;
                    println!("➕ count = {}", count);
                } else if id == dec_button {
                    count -= 1;
                    println!("➖ count = {}", count);
                } else if id == reset_button {
                    count = 0;
                    println!("🔄 count = {}", count);
                }
                
                send_message(&mut main_stream, &json!({
                    "method": "setText",
                    "params": {"aid": aid, "id": counter_id, "text": format!("点击次数: {}", count)}
                }))?;
                
                let color = if count > 0 { 0xFF4CAF50u32 as i32 } else if count < 0 { 0xFFF44336u32 as i32 } else { 0xFF2196F3u32 as i32 };
                send_message(&mut main_stream, &json!({
                    "method": "setTextColor",
                    "params": {"aid": aid, "id": counter_id, "color": color}
                }))?;
            },
            _ => {}
        }
    }
    
    println!("=== 程序结束 ===");
    Ok(())
}
