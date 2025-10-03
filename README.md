# Termux:GUI Rust Demo

这是一个使用 Rust 实现的 Termux:GUI 通信演示项目，展示了如何通过 Unix Domain Socket 与 Termux:GUI 插件进行底层通信。

## 功能特性

✅ 完整实现了 Termux:GUI 的底层通信协议  
✅ 展示双 Socket 架构（Main Socket + Event Socket）  
✅ 演示消息序列化/反序列化（4字节长度前缀 + JSON）  
✅ 实现 Activity 创建和 TextView 显示  
✅ 异步事件监听  
✅ 动态更新界面内容  

## 项目结构

```
termux-gui-rust-demo/
├── Cargo.toml          # 项目配置和依赖
├── src/
│   └── main.rs         # 主程序（完整的通信实现）
└── README.md           # 本文件
```

## 依赖项

- `serde_json`: JSON 序列化/反序列化
- `rand`: 生成随机 socket 地址

## 核心实现

### 1. Socket 连接建立

```rust
// 生成随机地址
let addr_main = generate_random_address();
let addr_event = generate_random_address();

// 绑定到抽象命名空间（\0前缀）
let main_listener = UnixListener::bind(format!("\0{}", addr_main))?;
let event_listener = UnixListener::bind(format!("\0{}", addr_event))?;

// 通过 broadcast 通知插件
Command::new("termux-am")
    .args(&["broadcast", "-n", "com.termux.gui/.GUIReceiver",
            "--es", "mainSocket", &addr_main,
            "--es", "eventSocket", &addr_event])
    .output()?;
```

### 2. 消息协议

所有消息采用统一格式：`[4字节长度(Big Endian)][JSON消息体(UTF-8)]`

```rust
fn send_message(stream: &mut UnixStream, msg: &Value) -> Result<(), Error> {
    let json_str = msg.to_string();
    let json_bytes = json_str.as_bytes();
    let len = (json_bytes.len() as u32).to_be_bytes();
    
    stream.write_all(&len)?;      // 发送长度
    stream.write_all(json_bytes)?; // 发送内容
    Ok(())
}

fn read_message(stream: &mut UnixStream) -> Result<Value, Error> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;
    
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    
    serde_json::from_slice(&buf)
}
```

### 3. 创建 Activity

```rust
let activity_msg = json!({
    "method": "newActivity",
    "params": {
        "canceloutside": true,
        "intercept": false
    }
});

let response = send_and_read(&mut main_stream, &activity_msg)?;
let aid = response[0].as_i64()?;  // Activity ID
let tid = response[1].as_str()?;  // Task ID
```

### 4. 创建 TextView

```rust
let textview_msg = json!({
    "method": "createTextView",
    "params": {
        "aid": aid,
        "text": "Hello World from Rust! 🦀"
    }
});

let response = send_and_read(&mut main_stream, &textview_msg)?;
let view_id = response.as_i64()?;
```

### 5. 事件监听

```rust
thread::spawn(move || {
    loop {
        let event = read_message(&mut event_stream)?;
        println!("收到事件: {}", event);
        
        if event["type"] == "destroy" {
            break;  // Activity被销毁，退出循环
        }
    }
});
```

## 编译和运行

### 编译项目

```bash
cd termux-gui-rust-demo
cargo build --release
```

### 运行程序

```bash
cargo run
```

或直接运行编译好的二进制文件：

```bash
./target/release/termux-gui-rust-demo
```

## 运行效果

程序会：

1. 🔌 建立与 Termux:GUI 插件的连接
2. 📱 创建一个新的 Activity
3. 📝 显示 "Hello World from Rust! 🦀"
4. ⏱️ 5秒后更新为 "Goodbye World! 👋"
5. 📨 持续监听和打印所有 GUI 事件
6. 🔚 再过5秒后自动关闭

## 输出示例

```
=== Termux:GUI Rust Demo ===

生成Socket地址:
  Main Socket: 8KjN2mZpQvXcBfLr9dYt
  Event Socket: 5HwR6nPqGzWsJxMv3uTc

Socket已绑定，等待连接...
广播已发送，等待插件连接...

✓ Main Socket 已连接
✓ Event Socket 已连接

执行协议握手...
✓ 协议握手成功

创建Activity...
发送消息: {"method":"newActivity","params":{"canceloutside":true,"intercept":false}}
接收消息: [1,"task_abc123"]
✓ Activity创建成功: ID=1, Task=task_abc123

创建TextView...
发送消息: {"method":"createTextView","params":{"aid":1,"text":"Hello World from Rust! 🦀"}}
接收消息: 42
✓ TextView创建成功: ID=42

程序将显示Hello World 5秒...
事件监听线程已启动...

📨 收到事件: {"type":"create","value":{"aid":1}}
📨 收到事件: {"type":"start","value":{"aid":1}}
📨 收到事件: {"type":"resume","value":{"aid":1}}

更新TextView文本...
发送消息: {"method":"setText","params":{"aid":1,"id":42,"text":"Goodbye World! 👋"}}
✓ 文本已更新

再显示5秒后自动关闭...

关闭Activity...
发送消息: {"method":"finishActivity","params":{"aid":1}}
✓ Activity已关闭

📨 收到事件: {"type":"destroy","value":{"aid":1,"finishing":true}}

=== 程序结束 ===
```

## 技术要点

### 通信架构

- **双 Socket 设计**: 命令通道和事件通道分离
- **异步事件处理**: 使用独立线程监听事件
- **线程安全**: Rust 的所有权系统天然保证线程安全

### 协议细节

- **抽象命名空间**: Socket 地址以 `\0` 开头，不占用文件系统
- **长度前缀协议**: 4字节 Big Endian 长度 + 消息体，避免消息边界问题
- **JSON 格式**: 使用 `serde_json` 进行结构化通信
- **协议握手**: 发送 `0x01` 版本号，接收 `0x00` 确认

### Rust 特性

- **错误处理**: 使用 `Result<T, E>` 和 `?` 操作符
- **所有权系统**: 自动管理资源，无需手动关闭 socket
- **类型安全**: 编译时检查类型，避免运行时错误
- **零成本抽象**: 性能接近 C/C++

## 与 Python 实现的对比

| 特性 | Python | Rust |
|------|--------|------|
| 代码行数 | ~50行 | ~200行 |
| 性能 | 中等 | 极高 |
| 内存安全 | 运行时检查 | 编译时保证 |
| 错误处理 | 异常 | Result<T,E> |
| 类型检查 | 动态 | 静态 |
| 依赖管理 | pip | cargo |
| 部署 | 需要解释器 | 单个二进制文件 |

## 扩展建议

你可以在此基础上扩展：

- 添加更多 View 类型（Button, EditText, ImageView 等）
- 实现复杂的布局（LinearLayout, GridLayout）
- 处理更多事件类型（click, touch, text 等）
- 添加错误重试机制
- 实现配置文件支持
- 创建可复用的库（crate）

## 相关资源

- [Termux:GUI 官方仓库](https://github.com/termux/termux-gui)
- [Python Bindings](https://github.com/tareksander/termux-gui-python-bindings)
- [Unix Domain Socket 文档](https://man7.org/linux/man-pages/man7/unix.7.html)

## 许可证

本项目仅用于学习和演示目的。

---

**作者**: 演示项目  
**版本**: 0.1.0  
**最后更新**: 2025
