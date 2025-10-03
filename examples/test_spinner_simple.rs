// 完全模仿 Python inputs2.py 的 Spinner 部分
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
    println!("=== 简单 Spinner 测试（模仿 Python）===");
    
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
    
    // 握手
    main_stream.write_all(&[0x01])?;
    let mut ack = [0u8; 1];
    main_stream.read_exact(&mut ack)?;
    
    // 创建 Activity - 对话框模式（像 Python 一样）
    let response = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {"dialog": true}
    }))?;
    let aid = response[0].as_i64().unwrap();
    println!("Activity 创建: aid={}", aid);
    
    // 创建 LinearLayout（不用 ScrollView，像 Python 一样）
    let layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true}
    }))?;
    let layout_id = layout.as_i64().unwrap();
    println!("布局创建: layout_id={}", layout_id);
    
    // 创建标题
    let title = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "Input Demo", "parent": layout_id}
    }))?;
    let title_id = title.as_i64().unwrap();
    
    // 设置标题大小（像 Python 一样）
    send_and_read(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 30}
    }))?;
    
    send_and_read(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 5}
    }))?;
    
    println!("标题创建完成");
    
    // 创建 Spinner（像 Python 一样：不传 list）
    let spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let spinner_id = spinner.as_i64().unwrap();
    println!("Spinner 创建: spinner_id={}", spinner_id);
    
    // 尝试设置 Spinner 的尺寸
    send_and_read(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": spinner_id, "width": -1}
    }))?;
    println!("Spinner 宽度设置: MATCH_PARENT");
    
    send_and_read(&mut main_stream, &json!({
        "method": "setHeight",
        "params": {"aid": aid, "id": spinner_id, "height": -2}
    }))?;
    println!("Spinner 高度设置: WRAP_CONTENT");
    
    // 设置列表（像 Python 一样）
    let strings = vec!["Option 1", "Option 2", "Option 3"];
    send_and_read(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": spinner_id, "list": strings}
    }))?;
    println!("列表设置完成");
    
    // 添加一些边距
    send_and_read(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": spinner_id, "margin": 10}
    }))?;
    println!("Spinner 边距设置完成");
    
    println!("\n✅ 界面创建成功！");
    println!("等待事件...\n");
    
    loop {
        match read_message(&mut event_stream) {
            Ok(event) => {
                let event_type = event["type"].as_str().unwrap_or("");
                println!("事件: {:?}", event);
                
                if event_type == "destroy" {
                    break;
                }
                
                if event_type == "itemselected" {
                    if let Some(selected) = event["value"]["selected"].as_str() {
                        println!("Spinner 选择: {}", selected);
                    }
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
