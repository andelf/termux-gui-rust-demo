// 调试版本：设置明确的尺寸和可见性
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::process::Command;
use serde_json::{json, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn generate_random_address() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(50).map(char::from).collect()
}

fn bind_abstract_socket(name: &str) -> std::io::Result<UnixListener> {
    unsafe {
        use std::os::unix::io::FromRawFd;
        let fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
        if fd < 0 { return Err(std::io::Error::last_os_error()); }
        
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
            return Err(std::io::Error::last_os_error());
        }
        if libc::listen(fd, 1) < 0 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        Ok(UnixListener::from_raw_fd(fd))
    }
}

fn send_message(stream: &mut UnixStream, msg: &Value) -> std::io::Result<()> {
    let json_bytes = msg.to_string().into_bytes();
    stream.write_all(&(json_bytes.len() as u32).to_be_bytes())?;
    stream.write_all(&json_bytes)?;
    stream.flush()
}

fn read_message(stream: &mut UnixStream) -> std::io::Result<Value> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let mut buf = vec![0u8; u32::from_be_bytes(len_buf) as usize];
    stream.read_exact(&mut buf)?;
    Ok(serde_json::from_slice(&buf).unwrap())
}

fn send_and_read(stream: &mut UnixStream, msg: &Value) -> std::io::Result<Value> {
    send_message(stream, msg)?;
    read_message(stream)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Spinner 调试测试 ===\n");
    
    let addr_main = generate_random_address();
    let addr_event = generate_random_address();
    let main_listener = bind_abstract_socket(&addr_main)?;
    let event_listener = bind_abstract_socket(&addr_event)?;
    
    Command::new("am")
        .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
                "--es", "mainSocket", &addr_main, "--es", "eventSocket", &addr_event])
        .output()?;
    
    let (mut main_stream, _) = main_listener.accept()?;
    let (mut event_stream, _) = event_listener.accept()?;
    
    main_stream.write_all(&[0x01])?;
    let mut ack = [0u8; 1];
    main_stream.read_exact(&mut ack)?;
    
    let response = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {"dialog": true}
    }))?;
    let aid = response[0].as_i64().unwrap();
    
    let layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true}
    }))?;
    let layout_id = layout.as_i64().unwrap();
    
    // 标题
    println!("创建标题...");
    let title = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "Spinner Debug Test", "parent": layout_id}
    }))?;
    let title_id = title.as_i64().unwrap();
    println!("标题 ID: {}", title_id);
    
    // 使用 send_message（不等待响应）
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 24}
    }))?;
    println!("✓ setTextSize 完成");
    
    println!("✓ 标题创建完成");
    
    // 创建一个 TextView 作为对比（看看是否显示）
    println!("创建测试文本...");
    let test_text = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "↓ Spinner 应该在这下面 ↓", "parent": layout_id}
    }))?;
    let test_text_id = test_text.as_i64().unwrap();
    println!("测试文本 ID: {}", test_text_id);
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": test_text_id, "margin": 10}
    }))?;
    println!("✓ setMargin 完成");
    
    println!("✓ 测试文本创建完成");
    
    // 创建 Spinner
    println!("创建 Spinner...");
    match send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    })) {
        Ok(spinner) => {
            let spinner_id = spinner.as_i64().unwrap();
            println!("✓ Spinner 创建成功: id={}", spinner_id);
            
            // 设置列表（使用 send_message）
            send_message(&mut main_stream, &json!({
                "method": "setList",
                "params": {"aid": aid, "id": spinner_id, "list": vec!["Apple", "Banana", "Cherry"]}
            }))?;
            println!("✓ setList 完成");
            
            // 设置宽度（使用 send_message）
            send_message(&mut main_stream, &json!({
                "method": "setWidth",
                "params": {"aid": aid, "id": spinner_id, "width": -1}
            }))?;
            println!("✓ setWidth 完成");
            
            // 设置边距
            send_message(&mut main_stream, &json!({
                "method": "setMargin",
                "params": {"aid": aid, "id": spinner_id, "margin": 10}
            }))?;
            println!("✓ setMargin 完成");
        },
        Err(e) => {
            println!("✗ Spinner 创建失败: {}", e);
            println!("可能该 Termux:GUI 版本不支持 Spinner");
        }
    }
    
    // 再添加一个 TextView 在下面作为对比
    println!("创建最后的测试文本...");
    let test_text2 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "↑ Spinner 应该在这上面 ↑", "parent": layout_id}
    }))?;
    println!("最后文本 ID: {}", test_text2.as_i64().unwrap());
    
    println!("\n✅ 所有组件创建完成！");
    println!("如果 Spinner 仍然不可见，说明可能是 Termux:GUI 的问题\n");
    
    loop {
        match read_message(&mut event_stream) {
            Ok(event) => {
                println!("事件: {:?}", event);
                if event["type"] == "destroy" {
                    break;
                }
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
