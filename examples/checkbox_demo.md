# Checkbox 复选框组件

## 简介

Checkbox（复选框）用于多选场景，用户可以同时选择多个选项。

## 特性

- ✅ 可以设置初始选中状态
- ✅ 支持文本标签
- ✅ 点击时发送 click 事件
- ✅ 事件包含 `set` 字段表示当前状态
- ✅ 可以通过 API 动态设置状态

## API

### 创建 Checkbox

```rust
{
    "method": "createCheckbox",
    "params": {
        "aid": <activity_id>,
        "text": "标签文本",
        "checked": true/false,     // 初始状态
        "parent": <parent_id>      // 可选
    }
}
```

**返回**: Checkbox 的 ID

### 设置选中状态

```rust
{
    "method": "setChecked",
    "params": {
        "aid": <activity_id>,
        "id": <checkbox_id>,
        "checked": true/false
    }
}
```

### 事件处理

当用户点击 Checkbox 时，会收到 click 事件：

```json
{
    "type": "click",
    "value": {
        "aid": 0,
        "id": 42,
        "set": true    // 当前是否选中
    }
}
```

## 运行示例

```bash
cargo run --example checkbox_demo --release
```

## 示例说明

该示例创建了一个设置界面，包含4个复选框：
- 📶 WiFi（初始：未选中）
- 📡 蓝牙（初始：选中）
- 📍 定位服务（初始：未选中）
- 🔔 通知（初始：选中）

功能：
1. 点击任何复选框，状态实时更新
2. 底部显示当前选中的项目
3. 点击"应用设置"按钮显示最终配置

## 代码示例

```rust
// 创建 Checkbox
let checkbox_id = send_and_read(&mut main_stream, &json!({
    "method": "createCheckbox",
    "params": {
        "aid": aid,
        "text": "📶 WiFi",
        "checked": false,
        "parent": layout_id
    }
}))?.as_i64().unwrap();

// 处理 click 事件
"click" => {
    let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
    let is_checked = event_value["set"].as_bool().unwrap_or(false);
    
    if clicked_id == checkbox_id {
        println!("WiFi: {}", if is_checked { "开启" } else { "关闭" });
    }
}
```

## 使用场景

- 设置界面（多个开关选项）
- 表单（用户协议、偏好设置）
- 任务列表（完成状态）
- 权限选择
- 过滤器（多条件筛选）

## 与 Radio Button 的区别

| 特性 | Checkbox | Radio Button |
|------|----------|--------------|
| 选择数量 | 多选 | 单选 |
| 独立性 | 独立 | 分组 |
| 典型场景 | 多个设置 | 单一选择 |

---

**组件类型**: CompoundButton  
**最后更新**: 2025
