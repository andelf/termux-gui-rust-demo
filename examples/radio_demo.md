# RadioGroup + RadioButton 单选组件

## 简介

RadioButton（单选按钮）必须配合 RadioGroup（单选组）使用。在同一个 RadioGroup 中，**只能有一个 RadioButton 被选中**。

## 特性

- ✅ 单选约束（组内互斥）
- ✅ 多个独立的 RadioGroup
- ✅ 支持初始选中状态
- ✅ 发送 `selected` 事件
- ✅ 自动管理选中状态
- ✅ 文本标签支持

## API

### 创建 RadioGroup

```rust
{
    "method": "createRadioGroup",
    "params": {
        "aid": <activity_id>,
        "parent": <parent_id>      // 可选
    }
}
```

**返回**: RadioGroup 的 ID

### 创建 RadioButton

```rust
{
    "method": "createRadioButton",
    "params": {
        "aid": <activity_id>,
        "text": "选项文本",
        "parent": <radio_group_id>,  // 必须指定 RadioGroup
        "checked": true/false        // 初始状态
    }
}
```

**返回**: RadioButton 的 ID

### 事件处理

当用户选择 RadioButton 时，**RadioGroup** 会发送 `selected` 事件：

```json
{
    "type": "selected",
    "value": {
        "aid": 0,
        "id": 123,          // RadioGroup 的 ID
        "selected": 456     // 被选中的 RadioButton 的 ID
    }
}
```

**注意**: 事件来自 **RadioGroup**，不是 RadioButton！

## 运行示例

```bash
cargo run --example radio_demo --release
```

## 示例说明

该示例模拟电商订单提交页面，包含3个独立的单选组：

### 配送方式组
- 📮 标准配送 (免费, 3-5天)
- 🚚 快速配送 (¥15, 1-2天)
- ⚡ 当日达 (¥30, 当天送达)

### 支付方式组
- 💚 微信支付
- 💙 支付宝
- 💰 货到付款

### 发票类型组
- ❌ 不需要发票
- 📧 电子发票
- 📄 纸质发票 (+¥5)

功能：
1. 每组只能选择一个选项
2. 选择自动更新状态显示
3. 动态计算总价（配送费 + 发票费）
4. "确认订单"按钮提交

## 代码示例

```rust
// 1. 创建 RadioGroup
let radio_group = send_and_read(&mut main_stream, &json!({
    "method": "createRadioGroup",
    "params": {"aid": aid, "parent": layout_id}
}))?.as_i64().unwrap();

// 2. 在组内创建 RadioButton
let radio1 = send_and_read(&mut main_stream, &json!({
    "method": "createRadioButton",
    "params": {
        "aid": aid,
        "text": "选项 1",
        "parent": radio_group,  // 指定父容器为 RadioGroup
        "checked": true         // 默认选中
    }
}))?.as_i64().unwrap();

let radio2 = send_and_read(&mut main_stream, &json!({
    "method": "createRadioButton",
    "params": {
        "aid": aid,
        "text": "选项 2",
        "parent": radio_group,
        "checked": false
    }
}))?.as_i64().unwrap();

// 3. 处理 selected 事件
"selected" => {
    let group_id = event_value["id"].as_i64().unwrap_or(-1);
    let selected_id = event_value["selected"].as_i64().unwrap_or(-1);
    
    if group_id == radio_group {
        if selected_id == radio1 {
            println!("选择了选项 1");
        } else if selected_id == radio2 {
            println!("选择了选项 2");
        }
    }
}
```

## 使用场景

- 单选题（问卷调查）
- 支付方式选择
- 配送方式选择
- 性别选择
- 等级选择
- 任何"二选一"或"多选一"场景

## RadioButton vs Checkbox

| 特性 | RadioButton | Checkbox |
|------|-------------|----------|
| 选择数量 | 单选（组内互斥） | 多选（独立） |
| 分组 | 必须有 RadioGroup | 不需要分组 |
| 事件类型 | `selected`（来自组） | `click`（来自自身） |
| 典型场景 | 单一选择 | 多项选择 |
| 能否取消 | 不能（必须选一个） | 可以 |

## 高级用法

### 多个独立的 RadioGroup

```rust
// 第一组 - 性别
let gender_group = send_and_read(&mut main_stream, &json!({
    "method": "createRadioGroup",
    "params": {"aid": aid, "parent": layout_id}
}))?.as_i64().unwrap();

let male = send_and_read(&mut main_stream, &json!({
    "method": "createRadioButton",
    "params": {"aid": aid, "text": "男", "parent": gender_group, "checked": true}
}))?.as_i64().unwrap();

let female = send_and_read(&mut main_stream, &json!({
    "method": "createRadioButton",
    "params": {"aid": aid, "text": "女", "parent": gender_group, "checked": false}
}))?.as_i64().unwrap();

// 第二组 - 年龄段
let age_group = send_and_read(&mut main_stream, &json!({
    "method": "createRadioGroup",
    "params": {"aid": aid, "parent": layout_id}
}))?.as_i64().unwrap();

// 这些 RadioButton 与上面的组独立
// 可以同时选择"男"和"18-25岁"
```

### 动态价格计算

```rust
let prices = [0, 15, 30];  // 对应3个选项的价格
let mut selected_index = 0;

"selected" => {
    let selected_id = event_value["selected"].as_i64().unwrap_or(-1);
    
    // 根据 ID 找到索引
    if selected_id == radio_ids[0] {
        selected_index = 0;
    } else if selected_id == radio_ids[1] {
        selected_index = 1;
    } else if selected_id == radio_ids[2] {
        selected_index = 2;
    }
    
    let price = prices[selected_index];
    println!("总价: ¥{}", price);
}
```

### 选项关联

```rust
// 选择"货到付款"时禁用某些配送方式
if payment_index == 2 {  // 货到付款
    // 不支持当日达
    if delivery_index == 2 {
        delivery_index = 0;  // 重置为标准配送
        // 提示用户
    }
}
```

## 注意事项

1. **RadioButton 必须放在 RadioGroup 内**
   - 不能直接放在其他容器中

2. **事件来自 RadioGroup**
   - 监听 `selected` 事件时检查 `group_id`
   - `selected` 字段是被选中的 RadioButton ID

3. **初始状态**
   - 建议设置一个默认选中
   - 如果都不选，用户必须手动选择

4. **多个 RadioGroup 是独立的**
   - 每个组维护自己的选中状态
   - 可以在不同组中同时选择

## 调试技巧

```rust
"selected" => {
    let group_id = event_value["id"].as_i64().unwrap_or(-1);
    let selected_id = event_value["selected"].as_i64().unwrap_or(-1);
    
    println!("RadioGroup {} 选中了 RadioButton {}", group_id, selected_id);
    
    // 完整打印事件
    println!("Event: {}", serde_json::to_string_pretty(&event)?);
}
```

---

**组件类型**: ViewGroup (RadioGroup), CompoundButton (RadioButton)  
**事件类型**: selected  
**最后更新**: 2025
