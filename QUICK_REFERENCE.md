# 🚀 快速参考手册

## 当前项目状态

✅ **库重构已完成** - termux-gui v0.2.0  
📦 **4 个 commits** 实现了完整的库结构  
📝 **代码减少 50%+** - 从 210 行到 100 行  
🎯 **下一步**: 在手机上测试新库

---

## 常用命令

### 编译

```bash
# 编译库
cargo build --lib --release

# 编译所有示例
cargo build --examples --release

# 编译特定示例
cargo build --example button_demo_v2 --release
```

### 运行示例

```bash
# 运行原版示例（已验证可用）
./target/release/examples/button_demo
./target/release/examples/checkbox_demo
./target/release/examples/input_demo
./target/release/examples/radio_demo
./target/release/examples/switch_demo

# 运行新库示例（待测试）
./target/release/examples/button_demo_v2
./target/release/examples/test_lib_minimal
```

### Git 操作

```bash
# 查看状态
git --no-pager status

# 查看日志
git --no-pager log --oneline -10

# 查看特定文件的变化
git --no-pager diff HEAD~1 src/lib.rs

# 创建新分支
git checkout -b feature/new-feature
```

### 文档

```bash
# 生成并查看文档
cargo doc --open

# 查看库的公共 API
cargo doc --lib --no-deps
```

---

## 项目结构速查

```
termux-gui-rust-demo/
├── src/
│   ├── lib.rs           ← 库入口
│   ├── error.rs         ← 错误类型
│   ├── connection.rs    ← Socket 通信
│   ├── activity.rs      ← Activity 管理
│   ├── view.rs          ← View 基类
│   └── components/      ← UI 组件
│       ├── text_view.rs
│       ├── button.rs
│       ├── edit_text.rs
│       ├── checkbox.rs
│       ├── switch.rs
│       ├── radio.rs
│       ├── spinner.rs
│       └── layout.rs
├── examples/
│   ├── button_demo.rs      ← 原版（已验证）
│   ├── button_demo_v2.rs   ← 新库版本（待测试）
│   └── ...
└── target/release/examples/
    └── ...                  ← 编译后的可执行文件
```

---

## 重要文档

| 文档 | 用途 |
|------|------|
| `STATUS_SUMMARY.md` | 📊 完整的项目状态总结 |
| `REFACTORING_PROGRESS.md` | 🔄 重构进度跟踪 |
| `REFACTORING_PLAN.md` | 📋 详细的重构计划 |
| `README.md` | 📖 项目说明 |
| `QUICK_REFERENCE.md` | ⚡ 本文档 |

---

## 新库 API 速查

### 创建 Activity

```rust
use termux_gui::{Activity, Result};

fn main() -> Result<()> {
    // 对话框模式
    let mut activity = Activity::new(true)?;
    
    // 全屏模式
    let mut activity = Activity::new(false)?;
    
    Ok(())
}
```

### 创建组件

```rust
// 创建布局
let layout = activity.create_linear_layout(None)?;
let h_layout = activity.create_linear_layout_horizontal(Some(layout.id()))?;

// 创建文本
let text = activity.create_text_view("Hello", Some(layout.id()))?;
text.set_text_size(&mut activity, 24)?;
text.set_text_color(&mut activity, 0xFF2196F3)?;

// 创建按钮
let button = activity.create_button("Click", Some(layout.id()))?;

// 创建输入框
let input = activity.create_edit_text("", Some(layout.id()))?;
input.set_hint(&mut activity, "Enter text")?;

// 创建复选框
let checkbox = activity.create_checkbox("Option", Some(layout.id()))?;

// 创建开关
let switch = activity.create_switch("Enable", Some(layout.id()))?;

// 创建单选按钮
let radio_group = activity.create_radio_group(Some(layout.id()))?;
let radio1 = activity.create_radio_button("Option 1", Some(radio_group.id()))?;
let radio2 = activity.create_radio_button("Option 2", Some(radio_group.id()))?;

// 创建下拉列表
let spinner = activity.create_spinner(Some(layout.id()))?;
spinner.set_list(&mut activity, &["Item 1", "Item 2"])?;
```

### 事件循环

```rust
use termux_gui::connection::read_message;

loop {
    let event = read_message(activity.event_stream())?;
    let event_type = event["type"].as_str().unwrap_or("");
    
    match event_type {
        "destroy" => break,
        "click" => {
            let id = event["value"]["id"].as_i64().unwrap_or(-1);
            if id == button.id() {
                println!("Button clicked!");
            }
        },
        _ => {}
    }
}

activity.finish()?;
```

---

## 故障排查

### 编译错误

```bash
# 清理并重新编译
cargo clean
cargo build --release
```

### 运行时问题

```bash
# 检查 Termux:GUI 是否安装
pm list packages | grep termux.gui

# 检查权限
termux-setup-storage

# 查看详细错误
RUST_BACKTRACE=1 ./target/release/examples/button_demo_v2
```

### 示例不显示

1. 确保手机屏幕解锁
2. 确保 Termux:GUI 应用已安装并授权
3. 尝试运行原版示例验证环境

---

## 下一步行动

### 🔥 优先级 1: 测试新库

```bash
# 1. 在手机上运行
./target/release/examples/button_demo_v2

# 2. 如果成功，继续创建更多示例
# 3. 如果失败，查看 STATUS_SUMMARY.md 获取调试建议
```

### ⭐ 优先级 2: 创建更多示例

参考 `examples/checkbox_demo.rs` 创建 `examples/checkbox_demo_v2.rs`

### 📚 优先级 3: 改进文档

为所有公共 API 添加文档注释

---

## 有用的链接

- Python 实现: `/data/data/com.termux/files/home/Documents/termux-gui-python-bindings`
- Java 源码: `/data/data/com.termux/files/home/termux-gui` (需要先 clone)
- Termux:GUI 项目: https://github.com/termux/termux-gui

---

**提示**: 随时查看 `STATUS_SUMMARY.md` 获取完整的项目状态！
