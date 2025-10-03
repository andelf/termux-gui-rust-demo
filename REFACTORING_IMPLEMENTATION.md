# 🔧 Termux GUI Rust 库重构实施计划

**创建日期**: 2025  
**目标**: 将示例项目重构为面向对象的 Rust 库  
**风格**: 面向对象、自定义错误、生命周期管理  
**策略**: 渐进式重构，保持现有功能

---

## 📋 目录

1. [重构目标](#重构目标)
2. [库设计](#库设计)
3. [实施步骤](#实施步骤)
4. [时间规划](#时间规划)
5. [验证清单](#验证清单)

---

## 🎯 重构目标

### 核心目标

1. **✅ 创建 termux-gui 库** - 面向对象的 API 设计
2. **✅ 自定义错误类型** - 清晰的错误处理
3. **✅ 生命周期管理** - 使用 Rust 的借用检查器
4. **✅ 消除代码重复** - 将 480-600 行样板代码提取到库中
5. **✅ 渐进式迁移** - 逐步替换示例，保持功能

### 成功标准

- 所有示例继续正常工作
- 示例代码量减少 50% 以上
- 提供清晰的面向对象 API
- 完整的文档和示例
- 通过所有测试

---

## 🏗️ 库设计

### 1. 模块结构

```
src/
├── lib.rs                    # 库入口
├── error.rs                  # 自定义错误类型
├── connection.rs             # 底层连接管理
├── activity.rs               # Activity 封装
├── view.rs                   # View 基类
├── event.rs                  # 事件系统
├── components/               # UI 组件
│   ├── mod.rs
│   ├── text_view.rs
│   ├── button.rs
│   ├── edit_text.rs
│   ├── checkbox.rs
│   ├── switch.rs
│   ├── radio.rs
│   ├── spinner.rs
│   └── layout.rs
└── prelude.rs               # 便捷导入
```

### 2. 错误类型设计

```rust
// src/error.rs

use std::fmt;
use std::io;
use serde_json;

/// Termux GUI 库的结果类型
pub type Result<T> = std::result::Result<T, GuiError>;

/// Termux GUI 库的错误类型
#[derive(Debug)]
pub enum GuiError {
    /// IO 错误
    Io(io::Error),
    
    /// JSON 序列化/反序列化错误
    Json(serde_json::Error),
    
    /// 连接错误
    ConnectionFailed(String),
    
    /// 无效的响应
    InvalidResponse(String),
    
    /// View 不存在
    ViewNotFound(i64),
    
    /// Activity 不存在
    ActivityNotFound(i64),
    
    /// 协议错误
    ProtocolError(String),
    
    /// 超时
    Timeout,
    
    /// 其他错误
    Other(String),
}

impl fmt::Display for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuiError::Io(err) => write!(f, "IO 错误: {}", err),
            GuiError::Json(err) => write!(f, "JSON 错误: {}", err),
            GuiError::ConnectionFailed(msg) => write!(f, "连接失败: {}", msg),
            GuiError::InvalidResponse(msg) => write!(f, "无效响应: {}", msg),
            GuiError::ViewNotFound(id) => write!(f, "View 不存在: {}", id),
            GuiError::ActivityNotFound(id) => write!(f, "Activity 不存在: {}", id),
            GuiError::ProtocolError(msg) => write!(f, "协议错误: {}", msg),
            GuiError::Timeout => write!(f, "操作超时"),
            GuiError::Other(msg) => write!(f, "错误: {}", msg),
        }
    }
}

impl std::error::Error for GuiError {}

impl From<io::Error> for GuiError {
    fn from(err: io::Error) -> Self {
        GuiError::Io(err)
    }
}

impl From<serde_json::Error> for GuiError {
    fn from(err: serde_json::Error) -> Self {
        GuiError::Json(err)
    }
}
```

### 3. 连接管理设计

```rust
// src/connection.rs

use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use serde_json::Value;
use crate::error::{Result, GuiError};

/// 生成随机 socket 地址
pub fn generate_random_address() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    
    (0..50)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// 绑定抽象命名空间 socket
pub fn bind_abstract_socket(name: &str) -> Result<UnixListener> {
    use std::os::unix::net::UnixListener as StdListener;
    use std::mem;
    use libc::{sockaddr_un, socket, bind, listen, AF_UNIX, SOCK_STREAM};
    
    unsafe {
        let fd = socket(AF_UNIX, SOCK_STREAM, 0);
        if fd < 0 {
            return Err(GuiError::ConnectionFailed("socket() 失败".to_string()));
        }
        
        let mut addr: sockaddr_un = mem::zeroed();
        addr.sun_family = AF_UNIX as u16;
        
        let name_bytes = name.as_bytes();
        if name_bytes.len() >= addr.sun_path.len() - 1 {
            return Err(GuiError::ConnectionFailed("socket 地址过长".to_string()));
        }
        
        addr.sun_path[1..name_bytes.len() + 1]
            .copy_from_slice(std::slice::from_raw_parts(name_bytes.as_ptr() as *const i8, name_bytes.len()));
        
        let addr_len = mem::size_of_val(&addr.sun_family) + 1 + name_bytes.len();
        
        if bind(fd, &addr as *const _ as *const _, addr_len as u32) < 0 {
            return Err(GuiError::ConnectionFailed("bind() 失败".to_string()));
        }
        
        if listen(fd, 1) < 0 {
            return Err(GuiError::ConnectionFailed("listen() 失败".to_string()));
        }
        
        Ok(StdListener::from_raw_fd(fd))
    }
}

/// 发送 JSON 消息
pub fn send_message(stream: &mut UnixStream, msg: &Value) -> Result<()> {
    let json_str = msg.to_string();
    let json_bytes = json_str.as_bytes();
    let len = (json_bytes.len() as u32).to_be_bytes();
    
    stream.write_all(&len)?;
    stream.write_all(json_bytes)?;
    
    Ok(())
}

/// 读取 JSON 消息
pub fn read_message(stream: &mut UnixStream) -> Result<Value> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;
    
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    
    Ok(serde_json::from_slice(&buf)?)
}

/// 发送消息并读取响应
pub fn send_and_read(stream: &mut UnixStream, msg: &Value) -> Result<Value> {
    send_message(stream, msg)?;
    read_message(stream)
}

/// 表示一个到 Termux:GUI 的连接
pub struct Connection {
    main_stream: UnixStream,
    event_stream: UnixStream,
}

impl Connection {
    /// 创建新的连接
    pub fn new() -> Result<Self> {
        let addr_main = generate_random_address();
        let addr_event = generate_random_address();
        
        let main_listener = bind_abstract_socket(&addr_main)?;
        let event_listener = bind_abstract_socket(&addr_event)?;
        
        // 发送 broadcast
        use std::process::Command;
        Command::new("termux-am")
            .args(&[
                "broadcast",
                "-n", "com.termux.gui/.GUIReceiver",
                "--es", "mainSocket", &addr_main,
                "--es", "eventSocket", &addr_event,
            ])
            .output()
            .map_err(|e| GuiError::ConnectionFailed(format!("broadcast 失败: {}", e)))?;
        
        // 接受连接
        let (mut main_stream, _) = main_listener.accept()?;
        let (mut event_stream, _) = event_listener.accept()?;
        
        // 协议握手
        main_stream.write_all(&[0x01])?;
        let mut ack = [0u8; 1];
        main_stream.read_exact(&mut ack)?;
        
        if ack[0] != 0x00 {
            return Err(GuiError::ProtocolError("握手失败".to_string()));
        }
        
        Ok(Connection {
            main_stream,
            event_stream,
        })
    }
    
    /// 发送消息（不等待响应）
    pub fn send(&mut self, msg: &Value) -> Result<()> {
        send_message(&mut self.main_stream, msg)
    }
    
    /// 发送消息并读取响应
    pub fn send_read(&mut self, msg: &Value) -> Result<Value> {
        send_and_read(&mut self.main_stream, msg)
    }
    
    /// 读取事件
    pub fn read_event(&mut self) -> Result<Value> {
        read_message(&mut self.event_stream)
    }
    
    /// 获取主流的可变引用（用于高级操作）
    pub fn main_stream_mut(&mut self) -> &mut UnixStream {
        &mut self.main_stream
    }
    
    /// 获取事件流的可变引用（用于高级操作）
    pub fn event_stream_mut(&mut self) -> &mut UnixStream {
        &mut self.event_stream
    }
}
```

### 4. Activity 设计

```rust
// src/activity.rs

use serde_json::json;
use crate::connection::Connection;
use crate::error::{Result, GuiError};
use crate::event::Event;
use crate::components::*;

/// 表示一个 GUI 窗口
pub struct Activity {
    conn: Connection,
    aid: i64,
    dialog: bool,
}

impl Activity {
    /// 创建新的 Activity
    pub fn new(dialog: bool) -> Result<Self> {
        let mut conn = Connection::new()?;
        
        let response = conn.send_read(&json!({
            "method": "newActivity",
            "params": {
                "dialog": dialog,
                "canceloutside": true,
                "intercept": false
            }
        }))?;
        
        let aid = response[0]
            .as_i64()
            .ok_or_else(|| GuiError::InvalidResponse("无法获取 Activity ID".to_string()))?;
        
        Ok(Activity { conn, aid, dialog })
    }
    
    /// 获取 Activity ID
    pub fn id(&self) -> i64 {
        self.aid
    }
    
    /// 是否为对话框模式
    pub fn is_dialog(&self) -> bool {
        self.dialog
    }
    
    /// 完成 Activity
    pub fn finish(&mut self) -> Result<()> {
        self.conn.send(&json!({
            "method": "finishActivity",
            "params": {"aid": self.aid}
        }))
    }
    
    /// 读取下一个事件
    pub fn read_event(&mut self) -> Result<Event> {
        let event = self.conn.read_event()?;
        Event::from_json(event)
    }
    
    /// 事件循环（接受闭包处理事件）
    pub fn event_loop<F>(&mut self, mut handler: F) -> Result<()>
    where
        F: FnMut(&mut Self, Event) -> Result<bool>,  // 返回 true 继续，false 退出
    {
        loop {
            let event = self.read_event()?;
            
            // 检查销毁事件
            if let Event::Destroy { .. } = event {
                return Ok(());
            }
            
            // 调用用户处理器
            if !handler(self, event)? {
                break;
            }
        }
        
        Ok(())
    }
    
    /// 创建 LinearLayout
    pub fn create_linear_layout(&mut self, parent: Option<i64>) -> Result<LinearLayout> {
        LinearLayout::new(self, parent)
    }
    
    /// 创建 NestedScrollView
    pub fn create_nested_scroll_view(&mut self, parent: Option<i64>) -> Result<NestedScrollView> {
        NestedScrollView::new(self, parent)
    }
    
    /// 创建 TextView
    pub fn create_text_view(&mut self, text: &str, parent: Option<i64>) -> Result<TextView> {
        TextView::new(self, text, parent)
    }
    
    /// 创建 Button
    pub fn create_button(&mut self, text: &str, parent: Option<i64>) -> Result<Button> {
        Button::new(self, text, parent)
    }
    
    /// 创建 EditText
    pub fn create_edit_text(&mut self, hint: &str, parent: Option<i64>) -> Result<EditText> {
        EditText::new(self, hint, parent)
    }
    
    /// 创建 Checkbox
    pub fn create_checkbox(&mut self, text: &str, parent: Option<i64>) -> Result<Checkbox> {
        Checkbox::new(self, text, parent)
    }
    
    /// 创建 Switch
    pub fn create_switch(&mut self, text: &str, parent: Option<i64>) -> Result<Switch> {
        Switch::new(self, text, parent)
    }
    
    /// 创建 RadioGroup
    pub fn create_radio_group(&mut self, parent: Option<i64>) -> Result<RadioGroup> {
        RadioGroup::new(self, parent)
    }
    
    /// 创建 Spinner
    pub fn create_spinner(&mut self, items: Vec<String>, parent: Option<i64>) -> Result<Spinner> {
        Spinner::new(self, items, parent)
    }
    
    // 内部方法：发送消息
    pub(crate) fn send(&mut self, msg: &serde_json::Value) -> Result<()> {
        self.conn.send(msg)
    }
    
    // 内部方法：发送并读取
    pub(crate) fn send_read(&mut self, msg: &serde_json::Value) -> Result<serde_json::Value> {
        self.conn.send_read(msg)
    }
}
```

### 5. View 基类设计

```rust
// src/view.rs

use serde_json::json;
use crate::activity::Activity;
use crate::error::Result;

/// View 基类，所有 UI 组件的基础
pub struct View {
    aid: i64,
    id: i64,
}

impl View {
    /// 创建新 View（内部使用）
    pub(crate) fn new(aid: i64, id: i64) -> Self {
        View { aid, id }
    }
    
    /// 获取 View ID
    pub fn id(&self) -> i64 {
        self.id
    }
    
    /// 获取 Activity ID
    pub fn activity_id(&self) -> i64 {
        self.aid
    }
    
    /// 设置宽度
    pub fn set_width(&self, activity: &mut Activity, width: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setWidth",
            "params": {"aid": self.aid, "id": self.id, "width": width}
        }))
    }
    
    /// 设置高度
    pub fn set_height(&self, activity: &mut Activity, height: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setHeight",
            "params": {"aid": self.aid, "id": self.id, "height": height}
        }))
    }
    
    /// 设置边距
    pub fn set_margin(&self, activity: &mut Activity, left: i32, top: i32, right: i32, bottom: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setMargin",
            "params": {
                "aid": self.aid,
                "id": self.id,
                "left": left,
                "top": top,
                "right": right,
                "bottom": bottom
            }
        }))
    }
}
```

### 6. 事件系统设计

```rust
// src/event.rs

use serde_json::Value;
use crate::error::{Result, GuiError};

/// GUI 事件
#[derive(Debug, Clone)]
pub enum Event {
    /// Activity 创建
    Create { aid: i64 },
    
    /// Activity 启动
    Start { aid: i64 },
    
    /// Activity 恢复
    Resume { aid: i64 },
    
    /// Activity 暂停
    Pause { aid: i64 },
    
    /// Activity 停止
    Stop { aid: i64 },
    
    /// Activity 销毁
    Destroy { aid: i64, finishing: bool },
    
    /// 点击事件
    Click { aid: i64, id: i64, set: Option<bool> },
    
    /// 长按事件
    LongClick { aid: i64, id: i64 },
    
    /// 焦点变化
    FocusChange { aid: i64, id: i64, focus: bool },
    
    /// 触摸事件
    Touch { aid: i64, id: i64, x: f64, y: f64, action: String },
    
    /// 文本变化
    Text { aid: i64, id: i64, text: String },
    
    /// RadioButton 选择
    Selected { aid: i64, id: i64 },
    
    /// Spinner 项选择
    ItemSelected { aid: i64, id: i64, index: i64 },
    
    /// 未知事件
    Unknown { raw: Value },
}

impl Event {
    /// 从 JSON 创建事件
    pub fn from_json(value: Value) -> Result<Self> {
        let event_type = value["type"]
            .as_str()
            .ok_or_else(|| GuiError::InvalidResponse("缺少事件类型".to_string()))?;
        
        let event = match event_type {
            "create" => {
                let aid = value["value"]["aid"]
                    .as_i64()
                    .ok_or_else(|| GuiError::InvalidResponse("缺少 aid".to_string()))?;
                Event::Create { aid }
            }
            
            "start" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                Event::Start { aid }
            }
            
            "resume" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                Event::Resume { aid }
            }
            
            "pause" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                Event::Pause { aid }
            }
            
            "stop" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                Event::Stop { aid }
            }
            
            "destroy" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let finishing = value["value"]["finishing"].as_bool().unwrap_or(false);
                Event::Destroy { aid, finishing }
            }
            
            "click" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                let set = value["value"]["set"].as_bool();
                Event::Click { aid, id, set }
            }
            
            "longClick" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                Event::LongClick { aid, id }
            }
            
            "focusChange" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                let focus = value["value"]["focus"].as_bool().unwrap_or(false);
                Event::FocusChange { aid, id, focus }
            }
            
            "touch" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                let x = value["value"]["x"].as_f64().unwrap_or(0.0);
                let y = value["value"]["y"].as_f64().unwrap_or(0.0);
                let action = value["value"]["action"].as_str().unwrap_or("").to_string();
                Event::Touch { aid, id, x, y, action }
            }
            
            "text" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                let text = value["value"]["text"].as_str().unwrap_or("").to_string();
                Event::Text { aid, id, text }
            }
            
            "selected" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                Event::Selected { aid, id }
            }
            
            "itemselected" => {
                let aid = value["value"]["aid"].as_i64().unwrap_or(0);
                let id = value["value"]["id"].as_i64().unwrap_or(0);
                let index = value["value"]["selected"].as_i64().unwrap_or(0);
                Event::ItemSelected { aid, id, index }
            }
            
            _ => Event::Unknown { raw: value },
        };
        
        Ok(event)
    }
}
```

### 7. 组件示例 (TextView)

```rust
// src/components/text_view.rs

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::{Result, GuiError};

/// TextView 组件
pub struct TextView {
    view: View,
}

impl TextView {
    /// 创建新的 TextView
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "text": text
        });
        
        if let Some(pid) = parent {
            params["parent"] = json!(pid);
        }
        
        let response = activity.send_read(&json!({
            "method": "createTextView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| GuiError::InvalidResponse("无法获取 View ID".to_string()))?;
        
        Ok(TextView {
            view: View::new(activity.id(), id),
        })
    }
    
    /// 获取 View ID
    pub fn id(&self) -> i64 {
        self.view.id()
    }
    
    /// 获取基础 View
    pub fn view(&self) -> &View {
        &self.view
    }
    
    /// 设置文本
    pub fn set_text(&self, activity: &mut Activity, text: &str) -> Result<()> {
        activity.send(&json!({
            "method": "setText",
            "params": {
                "aid": self.view.activity_id(),
                "id": self.view.id(),
                "text": text
            }
        }))
    }
    
    /// 设置文本大小
    pub fn set_text_size(&self, activity: &mut Activity, size: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setTextSize",
            "params": {
                "aid": self.view.activity_id(),
                "id": self.view.id(),
                "size": size
            }
        }))
    }
    
    /// 设置文本颜色 (ARGB 格式，如 0xFF0000FF 为蓝色)
    pub fn set_text_color(&self, activity: &mut Activity, color: i64) -> Result<()> {
        activity.send(&json!({
            "method": "setTextColor",
            "params": {
                "aid": self.view.activity_id(),
                "id": self.view.id(),
                "color": color
            }
        }))
    }
}
```

---

## 🔄 实施步骤

### 阶段 1: 创建基础库（2-3小时）

**目标**: 创建库结构和核心模块，不修改任何示例

#### 任务清单

1. **创建目录结构**
   ```bash
   mkdir -p src/components
   touch src/lib.rs
   touch src/error.rs
   touch src/connection.rs
   touch src/activity.rs
   touch src/view.rs
   touch src/event.rs
   touch src/prelude.rs
   touch src/components/mod.rs
   ```

2. **实现错误类型** (`src/error.rs`)
   - [ ] 定义 `GuiError` 枚举
   - [ ] 实现 `Display` trait
   - [ ] 实现 `Error` trait
   - [ ] 实现 `From<io::Error>` 和 `From<serde_json::Error>`
   - [ ] 定义 `Result<T>` 类型别名

3. **实现连接管理** (`src/connection.rs`)
   - [ ] `generate_random_address()` 函数
   - [ ] `bind_abstract_socket()` 函数
   - [ ] `send_message()` 函数
   - [ ] `read_message()` 函数
   - [ ] `send_and_read()` 函数
   - [ ] `Connection` 结构体
   - [ ] `Connection::new()` 方法

4. **实现事件系统** (`src/event.rs`)
   - [ ] `Event` 枚举定义所有事件类型
   - [ ] `Event::from_json()` 方法

5. **实现 View 基类** (`src/view.rs`)
   - [ ] `View` 结构体
   - [ ] 基础方法（id, set_width, set_height, set_margin）

6. **实现 Activity** (`src/activity.rs`)
   - [ ] `Activity` 结构体
   - [ ] `Activity::new()` 方法
   - [ ] `Activity::finish()` 方法
   - [ ] `Activity::read_event()` 方法
   - [ ] `Activity::event_loop()` 方法
   - [ ] 组件创建方法（先留空，后续实现）

7. **配置 Cargo.toml**
   ```toml
   [lib]
   name = "termux_gui"
   path = "src/lib.rs"
   ```

8. **编译测试**
   ```bash
   cargo build --lib
   cargo doc --lib --open
   ```

**验证标准**:
- [x] `cargo build --lib` 成功
- [x] `cargo build --examples` 仍然成功（示例未修改）
- [x] 无编译警告
- [x] 文档生成成功

---

### 阶段 2: 实现组件封装（4-6小时）

**目标**: 实现所有 UI 组件的封装

#### 任务清单

为每个组件创建文件并实现：

1. **LinearLayout** (`src/components/layout.rs`)
   - [ ] `LinearLayout` 结构体
   - [ ] `LinearLayout::new()` 方法
   - [ ] `set_linear_layout_params()` 方法

2. **NestedScrollView** (`src/components/layout.rs`)
   - [ ] `NestedScrollView` 结构体
   - [ ] `NestedScrollView::new()` 方法

3. **TextView** (`src/components/text_view.rs`)
   - [ ] 参考上面的设计实现

4. **Button** (`src/components/button.rs`)
   - [ ] 继承 TextView 的功能
   - [ ] 添加按钮特有方法

5. **EditText** (`src/components/edit_text.rs`)
   - [ ] `EditText` 结构体
   - [ ] 输入类型设置
   - [ ] `get_text()` 方法
   - [ ] `set_text()` 方法

6. **Checkbox** (`src/components/checkbox.rs`)
   - [ ] `Checkbox` 结构体
   - [ ] `is_checked()` 方法
   - [ ] `set_checked()` 方法

7. **Switch** (`src/components/switch.rs`)
   - [ ] 类似 Checkbox

8. **RadioGroup + RadioButton** (`src/components/radio.rs`)
   - [ ] `RadioGroup` 结构体
   - [ ] `RadioButton` 结构体
   - [ ] `get_checked()` 方法

9. **Spinner** (`src/components/spinner.rs`)
   - [ ] `Spinner` 结构体
   - [ ] `set_list()` 方法
   - [ ] `set_selection()` 方法
   - [ ] **修复级联更新问题**

10. **更新 components/mod.rs**
    ```rust
    mod text_view;
    mod button;
    mod edit_text;
    mod checkbox;
    mod switch;
    mod radio;
    mod spinner;
    mod layout;
    
    pub use text_view::TextView;
    pub use button::Button;
    pub use edit_text::EditText;
    pub use checkbox::Checkbox;
    pub use switch::Switch;
    pub use radio::{RadioGroup, RadioButton};
    pub use spinner::Spinner;
    pub use layout::{LinearLayout, NestedScrollView};
    ```

**验证标准**:
- [x] 所有组件编译通过
- [x] 文档生成正确
- [x] API 设计一致

---

### 阶段 3: 创建新示例验证（2-3小时）

**目标**: 用新库重写一个简单示例，验证 API 可用性

#### 选择 button_demo 进行验证

创建 `examples/button_demo_new.rs`:

```rust
use termux_gui::{Activity, Event};
use termux_gui::error::Result;

fn main() -> Result<()> {
    // 创建 Activity
    let mut activity = Activity::new(true)?;
    
    // 创建布局
    let layout = activity.create_linear_layout(None)?;
    
    // 创建标题
    let mut title = activity.create_text_view("按钮计数器", Some(layout.id()))?;
    title.set_text_size(&mut activity, 24)?;
    
    // 创建计数显示
    let counter = activity.create_text_view("点击次数: 0", Some(layout.id()))?;
    counter.set_text_size(&mut activity, 20)?;
    
    // 创建按钮
    let button = activity.create_button("点击我！", Some(layout.id()))?;
    
    // 计数器
    let mut count = 0;
    
    // 事件循环
    activity.event_loop(|activity, event| {
        match event {
            Event::Click { id, .. } if id == button.id() => {
                count += 1;
                counter.set_text(activity, &format!("点击次数: {}", count))?;
            }
            _ => {}
        }
        Ok(true)  // 继续循环
    })?;
    
    Ok(())
}
```

**对比分析**:
- 原始版本: ~200 行
- 新版本: ~40 行
- 减少: 80%

**验证标准**:
- [x] 编译成功
- [x] 运行正常
- [x] 功能一致
- [x] 代码更简洁

---

### 阶段 4: 迁移所有示例（4-6小时）

**目标**: 将所有示例迁移到新库

#### 迁移顺序

1. **button_demo.rs** (最简单)
2. **checkbox_demo.rs**
3. **switch_demo.rs**
4. **input_demo.rs**
5. **radio_demo.rs**
6. **spinner_demo.rs** (最复杂，同时修复级联问题)
7. **test_events.rs**

#### 迁移步骤（每个示例）

1. 备份原文件: `cp button_demo.rs button_demo.rs.bak`
2. 重写示例使用新 API
3. 编译测试: `cargo build --example button_demo --release`
4. 运行测试: `./target/release/examples/button_demo`
5. 功能验证
6. 删除备份（如果一切正常）

**验证标准**:
- [x] 所有示例编译通过
- [x] 所有示例功能正常
- [x] 代码量显著减少
- [x] Spinner 级联更新问题已修复

---

### 阶段 5: 清理和优化（1-2小时）

**目标**: 清理不需要的文件，优化代码

#### 任务清单

1. **删除测试文件**
   ```bash
   rm examples/test_minimal_spinner.rs
   rm examples/test_spinner_cascade.rs
   rm examples/test_spinner_debug.rs
   rm examples/test_spinner_fullscreen.rs
   rm examples/test_spinner_simple.rs
   ```

2. **删除备份文件**
   ```bash
   rm examples/*.bak
   ```

3. **更新文档**
   - [ ] 更新 README.md
   - [ ] 更新快速入门.md
   - [ ] 更新使用说明.md
   - [ ] 创建 API 文档

4. **代码审查**
   - [ ] 检查所有 `unwrap()` 调用
   - [ ] 添加必要的注释
   - [ ] 统一代码风格

**验证标准**:
- [x] 无未使用的文件
- [x] 文档与代码一致
- [x] 代码风格统一

---

### 阶段 6: 文档和发布（2-3小时）

**目标**: 完善文档，准备发布

#### 任务清单

1. **API 文档**
   - [ ] 为所有公共 API 添加文档注释
   - [ ] 添加示例代码到文档
   - [ ] 生成并检查文档: `cargo doc --open`

2. **README 更新**
   - [ ] 添加库介绍
   - [ ] 添加快速开始示例
   - [ ] 添加 API 概览
   - [ ] 更新依赖说明

3. **CHANGELOG 更新**
   - [ ] 记录重构变更
   - [ ] 记录 API 变化
   - [ ] 记录 Bug 修复

4. **创建示例文档**
   - [ ] examples/README.md
   - [ ] 每个示例的说明

**验证标准**:
- [x] 文档完整
- [x] 示例可运行
- [x] README 清晰

---

## ⏰ 时间规划

### 总时间估计: 15-23 小时

| 阶段 | 时间 | 累计 |
|------|------|------|
| 阶段 1: 基础库 | 2-3 小时 | 2-3 小时 |
| 阶段 2: 组件封装 | 4-6 小时 | 6-9 小时 |
| 阶段 3: 验证示例 | 2-3 小时 | 8-12 小时 |
| 阶段 4: 迁移所有 | 4-6 小时 | 12-18 小时 |
| 阶段 5: 清理优化 | 1-2 小时 | 13-20 小时 |
| 阶段 6: 文档发布 | 2-3 小时 | 15-23 小时 |

### 建议分配

- **第 1 天**: 阶段 1 (基础库)
- **第 2-3 天**: 阶段 2 (组件封装)
- **第 4 天**: 阶段 3 (验证示例)
- **第 5-6 天**: 阶段 4 (迁移所有)
- **第 7 天**: 阶段 5-6 (清理和文档)

---

## ✅ 验证清单

### 阶段 1 验证

- [ ] `cargo build --lib` 成功
- [ ] `cargo build --examples` 成功（未修改示例）
- [ ] `cargo doc --lib` 成功
- [ ] 无编译警告
- [ ] 错误类型正常工作
- [ ] 连接管理正常工作

### 阶段 2 验证

- [ ] 所有组件编译通过
- [ ] TextView 组件测试通过
- [ ] Button 组件测试通过
- [ ] EditText 组件测试通过
- [ ] Checkbox 组件测试通过
- [ ] Switch 组件测试通过
- [ ] RadioButton 组件测试通过
- [ ] Spinner 组件测试通过
- [ ] 布局组件测试通过

### 阶段 3 验证

- [ ] button_demo_new 编译成功
- [ ] 界面显示正常
- [ ] 按钮点击工作
- [ ] 计数器更新正常
- [ ] 退出正常

### 阶段 4 验证

- [ ] button_demo 迁移完成
- [ ] checkbox_demo 迁移完成
- [ ] switch_demo 迁移完成
- [ ] input_demo 迁移完成
- [ ] radio_demo 迁移完成
- [ ] spinner_demo 迁移完成（级联修复）
- [ ] test_events 迁移完成
- [ ] 所有示例功能正常

### 阶段 5 验证

- [ ] 测试文件已删除
- [ ] 备份文件已删除
- [ ] 文档已更新
- [ ] 代码风格统一
- [ ] 无 clippy 警告

### 阶段 6 验证

- [ ] API 文档完整
- [ ] README 清晰
- [ ] CHANGELOG 准确
- [ ] 示例文档完整
- [ ] 所有链接有效

### 最终验证

- [ ] `cargo build --all --release` 成功
- [ ] `cargo test` 成功（如果有测试）
- [ ] `cargo clippy` 无警告
- [ ] `cargo doc --open` 文档正确
- [ ] 所有示例手动测试通过
- [ ] 性能没有退化
- [ ] 代码量减少 40% 以上

---

## 🎯 成功标准

### 定量指标

- [x] 代码重复减少 > 90% (480 行 → 0)
- [x] 示例代码减少 > 50%
- [x] 编译时间减少或持平
- [x] 运行性能不下降

### 定性指标

- [x] API 清晰易用
- [x] 文档完整准确
- [x] 代码风格统一
- [x] 易于扩展新组件
- [x] 用户体验改善

---

## 📝 注意事项

### 重要提醒

1. **渐进式重构** - 每个阶段都要完全完成并验证再继续
2. **保持备份** - Git commit 在每个阶段后
3. **不破坏功能** - 所有示例必须继续工作
4. **Spinner 问题** - 在阶段 4 迁移时专门处理
5. **生命周期** - 仔细设计，避免借用冲突

### 风险和缓解

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|----------|
| 借用检查器问题 | 高 | 高 | 使用生命周期参数，或 RefCell |
| API 设计不当 | 中 | 高 | 阶段 3 提前验证 |
| 功能退化 | 中 | 高 | 每个阶段测试 |
| Spinner 问题 | 中 | 中 | 参考 Python 实现 |
| 性能下降 | 低 | 中 | 基准测试 |

---

## 🚀 开始重构

### 准备工作

```bash
# 1. 确保在项目目录
cd ~/termux-gui-rust-demo

# 2. 确保 Git 干净
git status
git add -A
git commit -m "📸 Before refactoring snapshot"

# 3. 创建重构分支
git checkout -b refactor/library

# 4. 创建目录
mkdir -p src/components

# 5. 创建文件
touch src/lib.rs src/error.rs src/connection.rs
touch src/activity.rs src/view.rs src/event.rs src/prelude.rs
touch src/components/mod.rs
```

### 第一步

现在可以开始阶段 1，实现 `src/error.rs`！

---

**准备好了吗？让我们开始重构！** 🚀
