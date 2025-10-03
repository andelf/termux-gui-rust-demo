use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write, Error};
use std::process::Command;
use std::thread;
use std::time::Duration;
use serde_json::{json, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

/// 生成随机的socket地址（50个字符）
fn generate_random_address() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect()
}

/// 创建抽象命名空间的 Unix Socket
/// 抽象命名空间地址以 null 字节开头
fn bind_abstract_socket(name: &str) -> Result<UnixListener, Error> {
    // 创建抽象命名空间地址: \0 + name
    let mut addr_bytes = vec![0u8]; // 以 null 字节开头
    addr_bytes.extend_from_slice(name.as_bytes());
    
    // 使用底层系统调用绑定
    unsafe {
        use std::os::unix::io::FromRawFd;
        use std::mem;
        
        // 创建 socket
        let fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
        if fd < 0 {
            return Err(Error::last_os_error());
        }
        
        // 准备 sockaddr_un 结构
        let mut addr: libc::sockaddr_un = mem::zeroed();
        addr.sun_family = libc::AF_UNIX as u16;
        
        // 复制地址到 sun_path (包括开头的 \0)
        let copy_len = addr_bytes.len().min(addr.sun_path.len());
        for (i, &byte) in addr_bytes.iter().take(copy_len).enumerate() {
            addr.sun_path[i] = byte as _;  // 自动转换为正确的类型
        }
        
        // 绑定 socket
        let addr_len = (mem::size_of::<libc::sa_family_t>() + addr_bytes.len()) as libc::socklen_t;
        let bind_result = libc::bind(
            fd,
            &addr as *const _ as *const libc::sockaddr,
            addr_len,
        );
        
        if bind_result < 0 {
            libc::close(fd);
            return Err(Error::last_os_error());
        }
        
        // 开始监听
        if libc::listen(fd, 1) < 0 {
            libc::close(fd);
            return Err(Error::last_os_error());
        }
        
        Ok(UnixListener::from_raw_fd(fd))
    }
}

/// 发送消息到socket（4字节长度 + JSON内容）
fn send_message(stream: &mut UnixStream, msg: &Value) -> Result<(), Error> {
    let json_str = msg.to_string();
    let json_bytes = json_str.as_bytes();
    let len = (json_bytes.len() as u32).to_be_bytes();
    
    stream.write_all(&len)?;
    stream.write_all(json_bytes)?;
    stream.flush()?;
    
    println!("发送消息: {}", json_str);
    Ok(())
}

/// 从socket读取消息（4字节长度 + JSON内容）
fn read_message(stream: &mut UnixStream) -> Result<Value, Error> {
    // 读取4字节的长度
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;
    
    // 读取消息体
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    
    let json_str = String::from_utf8(buf).expect("Invalid UTF-8");
    let value: Value = serde_json::from_str(&json_str).expect("Invalid JSON");
    
    println!("接收消息: {}", json_str);
    Ok(value)
}

/// 发送消息并读取响应
fn send_and_read(stream: &mut UnixStream, msg: &Value) -> Result<Value, Error> {
    send_message(stream, msg)?;
    read_message(stream)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Termux:GUI Rust Demo ===\n");
    
    // 1. 生成随机socket地址
    let addr_main = generate_random_address();
    let addr_event = generate_random_address();
    
    println!("生成Socket地址:");
    println!("  Main Socket: {}", &addr_main[..20]);
    println!("  Event Socket: {}", &addr_event[..20]);
    println!();
    
    // 2. 创建Unix Domain Socket（使用抽象命名空间）
    println!("创建抽象命名空间Socket...");
    let main_listener = bind_abstract_socket(&addr_main)?;
    let event_listener = bind_abstract_socket(&addr_event)?;
    
    println!("Socket已绑定，等待连接...");
    
    // 3. 通过Android Broadcast通知Termux:GUI插件
    let output = Command::new("termux-am")
        .args(&[
            "broadcast",
            "-n", "com.termux.gui/.GUIReceiver",
            "--es", "mainSocket", &addr_main,
            "--es", "eventSocket", &addr_event,
        ])
        .output();
    
    match output {
        Ok(out) => {
            if !out.status.success() {
                eprintln!("警告: termux-am 可能失败，尝试使用 am 命令...");
                Command::new("am")
                    .args(&[
                        "broadcast",
                        "-n", "com.termux.gui/.GUIReceiver",
                        "--es", "mainSocket", &addr_main,
                        "--es", "eventSocket", &addr_event,
                    ])
                    .output()?;
            }
        },
        Err(_) => {
            println!("termux-am 不可用，尝试 am 命令...");
            Command::new("am")
                .args(&[
                    "broadcast",
                    "-n", "com.termux.gui/.GUIReceiver",
                    "--es", "mainSocket", &addr_main,
                    "--es", "eventSocket", &addr_event,
                ])
                .output()?;
        }
    }
    
    println!("广播已发送，等待插件连接...\n");
    
    // 4. 接受连接
    let (mut main_stream, _) = main_listener.accept()?;
    println!("✓ Main Socket 已连接");
    
    let (mut event_stream, _) = event_listener.accept()?;
    println!("✓ Event Socket 已连接\n");
    
    // 5. 协议版本握手
    println!("执行协议握手...");
    main_stream.write_all(&[0x01])?;  // 发送协议版本
    
    let mut version_buf = [0u8; 1];
    main_stream.read_exact(&mut version_buf)?;
    
    if version_buf[0] != 0 {
        return Err("协议版本不匹配".into());
    }
    println!("✓ 协议握手成功\n");
    
    // 6. 创建Activity
    println!("创建Activity...");
    let activity_msg = json!({
        "method": "newActivity",
        "params": {
            "canceloutside": true,
            "intercept": false
        }
    });
    
    let response = send_and_read(&mut main_stream, &activity_msg)?;
    let aid = response[0].as_i64().ok_or("无法获取Activity ID")?;
    
    // tid 可能是字符串或数字
    let tid_value = &response[1];
    let tid_str = if let Some(s) = tid_value.as_str() {
        s.to_string()
    } else if let Some(n) = tid_value.as_i64() {
        n.to_string()
    } else {
        return Err("无法获取Task ID".into());
    };
    
    println!("✓ Activity创建成功: ID={}, Task={}\n", aid, tid_str);
    
    // 7. 创建TextView显示Hello World
    println!("创建TextView...");
    let textview_msg = json!({
        "method": "createTextView",
        "params": {
            "aid": aid,
            "text": "Hello World from Rust! 🦀"
        }
    });
    
    let response = send_and_read(&mut main_stream, &textview_msg)?;
    let view_id = response.as_i64().ok_or("无法获取View ID")?;
    
    println!("✓ TextView创建成功: ID={}\n", view_id);
    
    // 8. 在后台线程监听事件
    let event_thread = thread::spawn(move || {
        println!("事件监听线程已启动...\n");
        loop {
            match read_message(&mut event_stream) {
                Ok(event) => {
                    println!("📨 收到事件: {}", event);
                    
                    // 检查是否是destroy事件
                    if event["type"] == "destroy" {
                        println!("\n收到destroy事件，准备退出...");
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("读取事件错误: {}", e);
                    break;
                }
            }
        }
    });
    
    // 9. 显示5秒后更新文本
    println!("程序将显示Hello World 5秒...");
    thread::sleep(Duration::from_secs(5));
    
    println!("\n更新TextView文本...");
    let update_msg = json!({
        "method": "setText",
        "params": {
            "aid": aid,
            "id": view_id,
            "text": "Goodbye World! 👋\n\n使用Rust实现的Termux:GUI通信"
        }
    });
    
    send_message(&mut main_stream, &update_msg)?;
    println!("✓ 文本已更新\n");
    
    // 10. 选择运行模式
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("选择运行模式:");
    println!("1. 自动模式: 5秒后自动关闭");
    println!("2. 等待模式: 等待用户关闭Activity (按返回键或点击外部)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 检查是否有环境变量设置
    let auto_mode = std::env::var("AUTO_MODE").is_ok();
    
    if auto_mode {
        println!("使用自动模式 (环境变量 AUTO_MODE 已设置)");
        println!("再显示5秒后自动关闭...");
        thread::sleep(Duration::from_secs(5));
        
        // 关闭Activity
        println!("\n关闭Activity...");
        let finish_msg = json!({
            "method": "finishActivity",
            "params": {
                "aid": aid
            }
        });
        
        send_message(&mut main_stream, &finish_msg)?;
        println!("✓ Activity已关闭");
    } else {
        println!("使用等待模式");
        println!("窗口将保持打开，直到你:");
        println!("  - 按下返回键");
        println!("  - 点击窗口外部区域");
        println!("  - 从多任务界面关闭\n");
        
        println!("等待 Activity 被关闭...");
        println!("(提示: 下次运行时设置 AUTO_MODE=1 可自动关闭)\n");
    }
    
    // 等待事件线程结束
    let _ = event_thread.join();
    
    println!("\n=== 程序结束 ===");
    Ok(())
}
