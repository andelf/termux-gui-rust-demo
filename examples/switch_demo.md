# Switch 开关组件

## 简介

Switch（滑动开关）是一个可视化的开关按钮，适合用于开/关状态切换。与 Checkbox 功能相似，但外观更像物理开关。

## 特性

- ✅ 滑动切换动画
- ✅ 文本标签显示在左侧
- ✅ 开关显示在右侧
- ✅ 支持初始状态设置
- ✅ 与 Checkbox 相同的事件机制
- ✅ 可通过 API 动态设置状态

## API

### 创建 Switch

```rust
{
    "method": "createSwitch",
    "params": {
        "aid": <activity_id>,
        "text": "开关标签",
        "checked": true/false,     // 初始状态
        "parent": <parent_id>      // 可选
    }
}
```

**返回**: Switch 的 ID

### 设置开关状态

```rust
{
    "method": "setChecked",
    "params": {
        "aid": <activity_id>,
        "id": <switch_id>,
        "checked": true/false
    }
}
```

### 事件处理

当用户切换 Switch 时，会收到 click 事件：

```json
{
    "type": "click",
    "value": {
        "aid": 0,
        "id": 42,
        "set": true    // 当前是否开启
    }
}
```

## 运行示例

```bash
cargo run --example switch_demo --release
```

## 示例说明

该示例模拟智能家居控制面板，包含5个开关：
- 💡 客厅灯（初始：开启）
- ❄️ 空调（初始：关闭）
- 💧 加湿器（初始：关闭）
- 🪟 电动窗帘（初始：开启）
- 🎵 背景音乐（初始：关闭）

功能：
1. 滑动开关切换设备状态
2. 实时显示已开启的设备列表
3. 根据开启数量改变状态颜色：
   - 0个：灰色
   - 1个：绿色
   - 2-3个：橙色
   - 4+个：红色（高能耗）
4. "全部开启/关闭"按钮批量控制

## 代码示例

```rust
// 创建 Switch
let switch_id = send_and_read(&mut main_stream, &json!({
    "method": "createSwitch",
    "params": {
        "aid": aid,
        "text": "💡 客厅灯",
        "checked": true,
        "parent": layout_id
    }
}))?.as_i64().unwrap();

// 处理 click 事件
"click" => {
    let clicked_id = event_value["id"].as_i64().unwrap_or(-1);
    let is_checked = event_value["set"].as_bool().unwrap_or(false);
    
    if clicked_id == switch_id {
        println!("客厅灯: {}", if is_checked { "开启" } else { "关闭" });
    }
}

// 动态设置状态
send_message(&mut main_stream, &json!({
    "method": "setChecked",
    "params": {"aid": aid, "id": switch_id, "checked": true}
}))?;
```

## 使用场景

- 智能家居控制
- 系统设置（开关功能）
- 权限管理
- 功能启用/禁用
- 偏好设置

## Switch vs Checkbox

| 特性 | Switch | Checkbox |
|------|--------|----------|
| 外观 | 滑动开关 | 复选框 ☑️ |
| 文本位置 | 左侧 | 右侧 |
| 适用场景 | 开/关状态 | 多选列表 |
| 视觉效果 | 动画滑动 | 勾选标记 |
| 典型用途 | 设置开关 | 选项勾选 |

**选择建议**:
- 用 **Switch** 表示即时生效的开关（如WiFi开关）
- 用 **Checkbox** 表示需要确认的选项（如协议勾选）

## 高级用法

### 批量控制

```rust
// 全部开启
for switch_id in &switch_ids {
    send_message(&mut main_stream, &json!({
        "method": "setChecked",
        "params": {"aid": aid, "id": switch_id, "checked": true}
    }))?;
}
```

### 状态同步

```rust
// 跟踪所有开关状态
let mut states = vec![false; 5];

// 更新状态
if clicked_id == switch_ids[0] {
    states[0] = is_checked;
    // 同步到其他相关开关
    if states[0] {
        // 灯开启时自动关闭窗帘
        send_message(&mut main_stream, &json!({
            "method": "setChecked",
            "params": {"aid": aid, "id": switch_ids[3], "checked": false}
        }))?;
    }
}
```

---

**组件类型**: CompoundButton  
**父类**: Button  
**最后更新**: 2025
