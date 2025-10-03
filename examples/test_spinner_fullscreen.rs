// 全屏 Spinner 测试 - 更好的布局和间距
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
    println!("=== Spinner 全屏测试 ===\n");
    
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
    
    // 全屏 Activity
    let response = send_and_read(&mut main_stream, &json!({
        "method": "newActivity",
        "params": {}
    }))?;
    let aid = response[0].as_i64().unwrap();
    println!("✓ 全屏 Activity 创建");
    
    // 创建 NestedScrollView
    let scroll = send_and_read(&mut main_stream, &json!({
        "method": "createNestedScrollView",
        "params": {"aid": aid}
    }))?;
    let scroll_id = scroll.as_i64().unwrap();
    println!("✓ ScrollView 创建");
    
    // 创建主布局
    let layout = send_and_read(&mut main_stream, &json!({
        "method": "createLinearLayout",
        "params": {"aid": aid, "vertical": true, "parent": scroll_id}
    }))?;
    let layout_id = layout.as_i64().unwrap();
    
    // 设置布局边距
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": layout_id, "margin": 20}
    }))?;
    println!("✓ 布局创建");
    
    // 标题
    let title = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "📱 Spinner 演示", "parent": layout_id}
    }))?;
    let title_id = title.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": title_id, "size": 28}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": title_id, "margin": 10, "bottom": 20}
    }))?;
    println!("✓ 标题创建");
    
    // 品牌标签
    let label1 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择品牌:", "parent": layout_id}
    }))?;
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": label1.as_i64().unwrap(), "size": 18}
    }))?;
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": label1.as_i64().unwrap(), "margin": 5, "top": 10}
    }))?;
    
    // 品牌 Spinner
    let brands = vec!["请选择", "Apple", "Samsung", "Huawei", "Xiaomi", "OPPO", "Vivo"];
    let brand_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let brand_spinner_id = brand_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": brand_spinner_id, "list": brands}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": brand_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": brand_spinner_id, "margin": 10, "bottom": 15}
    }))?;
    println!("✓ 品牌 Spinner 创建");
    
    // 型号标签
    let label2 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择型号:", "parent": layout_id}
    }))?;
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": label2.as_i64().unwrap(), "size": 18}
    }))?;
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": label2.as_i64().unwrap(), "margin": 5, "top": 10}
    }))?;
    
    // 型号 Spinner
    let model_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let model_spinner_id = model_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": model_spinner_id, "list": vec!["请先选择品牌"]}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": model_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": model_spinner_id, "margin": 10, "bottom": 15}
    }))?;
    println!("✓ 型号 Spinner 创建");
    
    // 容量标签
    let label3 = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "选择容量:", "parent": layout_id}
    }))?;
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": label3.as_i64().unwrap(), "size": 18}
    }))?;
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": label3.as_i64().unwrap(), "margin": 5, "top": 10}
    }))?;
    
    // 容量 Spinner
    let storages = vec!["请选择", "64GB", "128GB", "256GB", "512GB", "1TB"];
    let storage_spinner = send_and_read(&mut main_stream, &json!({
        "method": "createSpinner",
        "params": {"aid": aid, "parent": layout_id}
    }))?;
    let storage_spinner_id = storage_spinner.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": storage_spinner_id, "list": storages}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setWidth",
        "params": {"aid": aid, "id": storage_spinner_id, "width": -1}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": storage_spinner_id, "margin": 10, "bottom": 20}
    }))?;
    println!("✓ 容量 Spinner 创建");
    
    // 结果显示
    let result = send_and_read(&mut main_stream, &json!({
        "method": "createTextView",
        "params": {"aid": aid, "text": "请选择配置...", "parent": layout_id}
    }))?;
    let result_id = result.as_i64().unwrap();
    
    send_message(&mut main_stream, &json!({
        "method": "setTextSize",
        "params": {"aid": aid, "id": result_id, "size": 16}
    }))?;
    
    send_message(&mut main_stream, &json!({
        "method": "setMargin",
        "params": {"aid": aid, "id": result_id, "margin": 15, "top": 20}
    }))?;
    
    println!("\n✅ 界面创建完成！");
    println!("全屏显示，布局清晰，间距合理\n");
    
    loop {
        match read_message(&mut event_stream) {
            Ok(event) => {
                let event_type = event["type"].as_str().unwrap_or("");
                
                if event_type == "destroy" {
                    break;
                }
                
                if event_type == "itemselected" {
                    let view_id = event["value"]["id"].as_i64().unwrap_or(-1);
                    let selected = event["value"]["selected"].as_str().unwrap_or("");
                    
                    println!("选择: {} (view_id: {})", selected, view_id);
                    
                    // 更新结果显示
                    send_message(&mut main_stream, &json!({
                        "method": "setText",
                        "params": {"aid": aid, "id": result_id, "text": format!("您选择了: {}", selected)}
                    }))?;
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
