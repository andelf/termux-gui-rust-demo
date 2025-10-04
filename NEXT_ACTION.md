# 下一步行动计划

## 📅 更新时间: 2025-01-04

## ✅ 当前状态

### 重构进度
- ✅ 核心库创建完成 (src/lib.rs, src/activity.rs, src/connection.rs, src/view.rs)
- ✅ 所有组件封装完成 (8个组件)
- ✅ 关键Bug修复 (send() vs send_read())
- ✅ 布局参数支持 (WRAP_CONTENT, MATCH_PARENT, weight)
- ✅ 示例代码量减少50%+

### 当前问题
- ⚠️ button_demo_v2 界面显示不完整（只看到文本，没有按钮）
- ⚠️ spinner_demo 级联更新不工作（原有问题）
- ⚠️ 太多测试版本文件需要清理

---

## 🎯 立即行动 (今天)

### 1. 验证新库示例 ⬅️ **当前任务**

#### 步骤A: 运行并检查输出
```bash
cd ~/termux-gui-rust-demo

# 运行新库示例
./target/release/examples/button_demo_v2

# 或重新编译运行
cargo run --example button_demo_v2 --release
```

#### 预期行为
- ✅ 应该看到一个对话框
- ✅ 标题: "计数器演示 🦀"
- ✅ 计数器: "点击次数: 0"
- ✅ 三个按钮: "➕ 增加", "➖ 减少", "🔄 重置"

#### 如果问题依然存在
检查终端输出的调试信息：
```
[DEBUG] Activity::new() - creating connection...
[DEBUG] Generating addresses...
[DEBUG] Binding sockets...
[DEBUG] Sending broadcast...
[DEBUG] Accepting connections...
[DEBUG] Handshake...
[DEBUG] Connection established!
[DEBUG] Activity::new() - sending newActivity...
[DEBUG] send_and_read: sending...
[DEBUG] send_and_read: reading response...
[DEBUG] send_and_read: got response!
...
```

看看在哪个组件创建后停止输出。

#### 步骤B: 对比旧版本
```bash
# 运行旧版本（工作正常）
./target/release/examples/button_demo
```

如果旧版本正常，新版本不正常，说明库封装有问题。

---

### 2. 诊断和修复

#### 可能原因1: 按钮创建卡住
如果输出在 "创建按钮" 时停止，可能是：
- Button::new() 内部的 send_read() 没有收到响应
- 检查 button.rs 的实现

#### 可能原因2: 布局参数设置卡住
如果输出在布局设置时停止，可能是：
- set_linear_layout_params() 调用问题
- 尝试注释掉所有布局参数设置，只保留基础创建

#### 可能原因3: 事件循环问题
如果界面出现但没有控件，可能是：
- 组件创建了但没有正确添加到布局
- 检查 parent 参数是否正确传递

---

### 3. 临时解决方案: 创建最小示例

如果问题难以定位，创建一个最最简单的版本：

```bash
nano examples/button_demo_minimal.rs
```

```rust
use termux_gui::{Activity, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== 最小 Button 示例 ===\n");
    
    let mut activity = Activity::new(true)?;
    println!("✓ Activity 创建");
    
    let layout = activity.create_linear_layout(None)?;
    println!("✓ Layout 创建: {}", layout.id());
    
    let text = activity.create_text_view("Hello", Some(layout.id()))?;
    println!("✓ TextView 创建: {}", text.id());
    
    let button = activity.create_button("点我", Some(layout.id()))?;
    println!("✓ Button 创建: {}", button.id());
    
    println!("\n等待事件...\n");
    
    loop {
        let event = read_message(activity.event_stream())?;
        println!("[事件] {:?}", event);
        
        if event["type"] == "destroy" {
            break;
        }
        
        if event["type"] == "click" {
            let id = event["value"]["id"].as_i64().unwrap_or(-1);
            if id == button.id() {
                text.set_text(&mut activity, "已点击!")?;
            }
        }
    }
    
    Ok(())
}
```

编译并运行：
```bash
cargo build --example button_demo_minimal --release
./target/release/examples/button_demo_minimal
```

---

## 📝 今天的目标

1. ✅ 编译通过 (已完成)
2. ⏳ 在设备上验证 button_demo_v2
3. ⏳ 找出界面不完整的原因
4. ⏳ 修复问题或创建最小可用示例
5. ⏳ Git commit 记录修复

---

## 📋 本周计划

### 第1天 (今天)
- [x] 编译新库示例
- [ ] 验证功能
- [ ] 诊断和修复问题
- [ ] Git commit

### 第2天
- [ ] 清理测试文件
- [ ] 迁移 checkbox_demo_v2
- [ ] 迁移 input_demo_v2

### 第3天
- [ ] 迁移 switch_demo_v2
- [ ] 迁移 radio_demo_v2

### 第4天
- [ ] 迁移 spinner_demo_v2
- [ ] 修复 spinner 级联更新问题

### 第5天
- [ ] 删除旧示例
- [ ] 更新文档
- [ ] 完整测试所有示例

---

## 🚀 下一步命令

```bash
# 1. 运行测试
./target/release/examples/button_demo_v2

# 2. 如果有问题，创建最小示例
nano examples/button_demo_minimal.rs
# (复制上面的代码)

# 3. 编译测试
cargo build --example button_demo_minimal --release
./target/release/examples/button_demo_minimal

# 4. 记录结果
echo "测试结果: ..." >> TEST_RESULTS.txt

# 5. Commit
git add .
git commit -m "test: 验证新库示例功能"
```

---

## 💡 调试技巧

1. **增加调试输出**
   在每个组件创建后添加 `println!`

2. **对比Python实现**
   ```bash
   python3 ~/Documents/termux-gui-python-bindings/examples/button.py
   ```
   确认是否是底层协议问题

3. **检查Java日志**
   ```bash
   logcat | grep -i termux
   ```
   查看GUI插件的日志

4. **分步创建**
   一个组件一个组件地创建，找出卡住的位置

---

**开始吧！** 🦀
