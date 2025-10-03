// 测试事件接收 - 打印所有收到的事件
// 用于调试事件处理问题

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
    println!("=== 事件调试工具 ===");
    println!("此工具会打印所有收到的事件");
    println!("请手动关闭界面并观察输出\n");
    
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
    
    send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "事件调试工具\n\n请关闭此界面", "parent": layout_id}
    }))?;
    
    println!("✓ 界面创建完成");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("开始监听事件...\n");
    
    let mut event_count = 0;
    loop {
        match read_message(&mut event_stream) {
            Ok(event) => {
                event_count += 1;
                println!("事件 #{}: {}", event_count, serde_json::to_string_pretty(&event)?);
                println!("━━━━━━━━━━━━━━━━━━━━━━");
                
                let event_type = event["type"].as_str().unwrap_or("");
                
                // 尝试多种退出条件
                if event_type == "destroy" {
                    let finishing = event["value"]["finishing"].as_bool();
                    println!("⚠️  收到 destroy 事件");
                    println!("    finishing 字段: {:?}", finishing);
                    
                    if finishing == Some(true) {
                        println!("✅ finishing=true, 退出程序");
                        break;
                    } else {
                        println!("⚠️  finishing={:?}, 继续等待...", finishing);
                    }
                }
            },
            Err(e) => {
                println!("❌ 读取错误: {}", e);
                break;
            }
        }
    }
    
    println!("\n=== 程序结束 ===");
    println!("总共收到 {} 个事件", event_count);
    Ok(())
}
