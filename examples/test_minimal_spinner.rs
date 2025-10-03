// 最小化 Spinner 测试
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
    println!("=== 最小化 Spinner 测试 ===");
    
    let addr_main = generate_random_address();
    let addr_event = generate_random_address();
    println!("1. 生成地址完成");
    
    let main_listener = bind_abstract_socket(&addr_main)?;
    let event_listener = bind_abstract_socket(&addr_event)?;
    println!("2. Socket 绑定完成");
    
    Command::new("am")
        .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
                "--es", "mainSocket", &addr_main, "--es", "eventSocket", &addr_event])
        .output()?;
    println!("3. 广播发送完成");
    
    let (mut main_stream, _) = main_listener.accept()?;
    let (mut event_stream, _) = event_listener.accept()?;
    println!("4. 连接建立完成");
    
    // 握手
    main_stream.write_all(&[0x01])?;
    let mut ack = [0u8; 1];
    main_stream.read_exact(&mut ack)?;
    println!("5. 握手完成");
    
    // 创建 Activity (全屏模式)
    let response = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {}
    }))?;
    let aid = response[0].as_i64().unwrap();
    println!("6. Activity 创建: aid={}", aid);
    
    // 创建 NestedScrollView
    let scroll = send_and_read(&mut main_stream, &json!({
        "method": "createNestedScrollView",
        "params": {"aid": aid}
    }))?;
    let scroll_id = scroll.as_i64().unwrap();
    println!("7. ScrollView 创建: scroll_id={}", scroll_id);
    
    // 创建布局
    let layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true, "parent": scroll_id}
    }))?;
    let layout_id = layout.as_i64().unwrap();
    println!("8. 布局创建: layout_id={}", layout_id);
    
    // 创建标题
    let title = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "Spinner Test", "parent": layout_id}
    }))?;
    println!("8. 标题创建: {:?}", title);
    
    // 创建 Spinner (不传 list)
    println!("9. 准备创建 Spinner...");
    let spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let spinner_id = spinner.as_i64().unwrap();
    println!("10. Spinner 创建成功: spinner_id={}", spinner_id);
    
    // 设置宽度
    send_and_read(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": spinner_id, "width": -1}
    }))?;
    println!("11. 宽度设置完成");
    
    // 设置列表
    let items = vec!["Apple", "Samsung", "Huawei"];
    send_and_read(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": spinner_id, "list": items}
    }))?;
    println!("12. 列表设置完成");
    
    println!("\n✅ 界面创建成功！请检查屏幕");
    println!("等待事件...\n");
    
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
