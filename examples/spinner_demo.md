# Spinner 下拉列表示例

## 概述

这个示例展示了如何使用 Spinner（下拉列表）组件，实现了一个完整的手机订购向导，包括品牌、型号、容量和颜色的选择，以及品牌和型号的联动功能。

## 运行方式

```bash
cd ~/termux-gui-rust-demo
cargo run --example spinner_demo --release
```

## 功能特性

### 1. Spinner 组件 ✅
- 下拉选择列表
- 动态选项设置 (`setList`)
- 选项选择事件 (`itemselected`)
- 多个独立 Spinner

### 2. 联动选择 ✅
- 品牌选择触发型号列表更新
- 根据不同品牌显示不同型号
- 实时响应用户选择

### 3. 状态管理 ✅
- 追踪所有选择状态
- 实时更新结果显示
- 验证选择完整性

### 4. 动态反馈 ✅
- 颜色变化（完成/待完成）
- 文字提示
- 订购确认

## 界面组成

```
┌─────────────────────────────┐
│   📱 手机订购向导            │
│                             │
│ 选择品牌:                    │
│ [Apple ▼]                   │
│                             │
│ 选择型号:                    │
│ [iPhone 15 Pro Max ▼]      │
│                             │
│ 选择容量:                    │
│ [256GB ▼]                   │
│                             │
│ 选择颜色:                    │
│ [钛黑色 ▼]                   │
│                             │
│ ✅ 已选择:                   │
│ 品牌: Apple                 │
│ 型号: iPhone 15 Pro Max     │
│ 容量: 256GB                 │
│ 颜色: 钛黑色                 │
│                             │
│     [🛒 确认订购]            │
└─────────────────────────────┘
```

## Spinner 选项

### 品牌选项（6个）
```
Apple
Samsung
Huawei
Xiaomi
OPPO
Vivo
```

### 型号选项（根据品牌动态变化）

**Apple**:
- iPhone 15 Pro Max
- iPhone 15 Pro
- iPhone 15
- iPhone 14

**Samsung**:
- Galaxy S24 Ultra
- Galaxy S24+
- Galaxy S24
- Galaxy Z Fold5

**Huawei**:
- Mate 60 Pro
- Mate 60
- P60 Pro
- P60

**Xiaomi**:
- 14 Ultra
- 14 Pro
- 14
- 13T Pro

**OPPO**:
- Find X7 Ultra
- Find X7
- Reno 11 Pro
- Reno 11

**Vivo**:
- X100 Pro
- X100
- S18 Pro
- S18

### 容量选项（5个）
```
64GB
128GB
256GB
512GB
1TB
```

### 颜色选项（7个）
```
黑色
白色
金色
银色
蓝色
紫色
绿色
```

## API 使用

### 1. 创建 Spinner

```rust
let brands = vec!["请选择", "Apple", "Samsung", "Huawei"];
let spinner = send_and_read(&mut main_stream, &json!({
    "method": "createSpinner",
    "params": {
        "aid": aid,
        "list": brands
    }
}))?;
let spinner_id = spinner.as_i64().unwrap();
```

### 2. 更新 Spinner 列表

```rust
let models = vec!["请选择", "iPhone 15 Pro Max", "iPhone 15 Pro"];
send_and_read(&mut main_stream, &json!({
    "method": "setList",
    "params": {
        "aid": aid,
        "id": spinner_id,
        "list": models
    }
}))?;
```

### 3. 处理选择事件

```rust
if event_type == "itemselected" {
    let view_id = event["value"]["id"].as_i64().unwrap_or(-1);
    let index = event["value"]["index"].as_i64().unwrap_or(0) as usize;
    
    if view_id == spinner_id && index > 0 {
        let selected_item = items[index];
        println!("选择了: {}", selected_item);
    }
}
```

## 事件说明

### itemselected 事件

```json
{
    "type": "itemselected",
    "value": {
        "aid": 1,
        "id": 42,
        "index": 2
    }
}
```

- **type**: 固定为 `"itemselected"`
- **aid**: Activity ID
- **id**: Spinner 的 View ID
- **index**: 选中项的索引（0-based）

## 实现要点

### 1. 联动选择

品牌选择会触发型号列表的更新：

```rust
if view_id == brand_spinner_id && index > 0 {
    let brand = brands[index];
    
    // 根据品牌获取型号列表
    let models = match brand {
        "Apple" => vec!["请选择", "iPhone 15 Pro Max", ...],
        "Samsung" => vec!["请选择", "Galaxy S24 Ultra", ...],
        _ => vec!["请选择"],
    };
    
    // 更新型号 Spinner
    send_and_read(&mut main_stream, &json!({
        "method": "setList",
        "params": {"aid": aid, "id": model_spinner_id, "list": models}
    }))?;
}
```

### 2. 状态追踪

使用 `Arc<Mutex<Vec>>` 追踪所有选择：

```rust
let selections = Arc::new(Mutex::new(vec![
    ("品牌".to_string(), "".to_string()),
    ("型号".to_string(), "".to_string()),
    ("容量".to_string(), "".to_string()),
    ("颜色".to_string(), "".to_string()),
]));
```

### 3. 实时反馈

根据选择状态动态更新结果显示：

```rust
let all_selected = sels.iter().all(|(_, v)| !v.is_empty());

let result_text = if all_selected {
    format!("✅ 已选择:\n品牌: {}\n型号: {}\n容量: {}\n颜色: {}",
        sels[0].1, sels[1].1, sels[2].1, sels[3].1)
} else {
    let missing: Vec<&str> = sels.iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| k.as_str())
        .collect();
    format!("⚠️ 待选择: {}", missing.join(", "))
};
```

### 4. 颜色反馈

```rust
let color = if all_selected { 
    0xFF4CAF50_u32  // 绿色：已完成
} else { 
    0xFF666666_u32  // 灰色：未完成
};

send_and_read(&mut main_stream, &json!({
    "method": "setTextColor",
    "params": {"aid": aid, "id": result_id, "color": color}
}))?;
```

## 运行效果

### 初始状态
```
=== Spinner 下拉列表演示 ===

✓ 连接建立
✓ 界面创建完成

━━━━━━━━━━━━━━━━━━━━━━
提示: 从下拉列表中选择
━━━━━━━━━━━━━━━━━━━━━━

结果显示: ⚠️ 待选择: 品牌, 型号, 容量, 颜色
```

### 选择品牌后
```
📱 品牌: Apple

结果显示: ⚠️ 待选择: 型号, 容量, 颜色
（型号列表已更新为 Apple 的型号）
```

### 完成所有选择
```
📱 品牌: Apple
📱 型号: iPhone 15 Pro Max
💾 容量: 256GB
🎨 颜色: 钛黑色

结果显示: ✅ 已选择
品牌: Apple
型号: iPhone 15 Pro Max
容量: 256GB
颜色: 钛黑色
（文字变为绿色）
```

### 点击确认订购
```
🎉 订购成功！

品牌: Apple
型号: iPhone 15 Pro Max
容量: 256GB
颜色: 钛黑色

感谢您的订购！
```

## 使用场景

### 1. 电商应用
- 商品规格选择
- 收货地址选择
- 配送方式选择

### 2. 表单应用
- 省市区三级联动
- 年月日选择器
- 分类筛选器

### 3. 设置界面
- 语言选择
- 主题选择
- 字体大小选择

### 4. 数据录入
- 职业选择
- 学历选择
- 行业分类

## 技术特点

### 1. 多 Spinner 管理
- 4 个独立的 Spinner 组件
- 每个 Spinner 独立追踪状态
- 通过 ID 区分不同 Spinner

### 2. 动态列表更新
- `setList` 方法动态修改选项
- 品牌改变时自动更新型号
- 实时响应用户操作

### 3. 验证和提示
- 检查所有选项是否已选择
- 显示缺失的选项
- 提交前验证完整性

### 4. 用户体验
- 清晰的视觉反馈（颜色变化）
- 实时的文字提示
- 友好的错误提示

## 代码统计

```
总行数:        ~550 行
函数数量:      8 个
Spinner 数量:  4 个
选项总数:      30+ 个
品牌数据:      6 个品牌 × 4-5 个型号
```

## 与其他示例对比

| 特性 | Checkbox | Switch | Radio | Spinner |
|------|----------|--------|-------|---------|
| 组件类型 | CompoundButton | CompoundButton | RadioGroup | View |
| 选择方式 | 多选 | 开关 | 单选 | 下拉单选 |
| 事件类型 | click | click | selected | itemselected |
| 联动功能 | ❌ | ❌ | ❌ | ✅ |
| 动态列表 | ❌ | ❌ | ❌ | ✅ |
| 占用空间 | 中 | 中 | 大 | 小 |
| 适用场景 | 设置 | 控制 | 表单 | 表单 |

## 扩展建议

### 1. 三级联动
```rust
// 省 -> 市 -> 区
let provinces = vec!["广东省", "浙江省", "江苏省"];
let cities_gd = vec!["广州市", "深圳市", "东莞市"];
let districts_gz = vec!["天河区", "越秀区", "海珠区"];
```

### 2. 搜索功能
```rust
// 结合 EditText 实现搜索过滤
let filtered = all_items.iter()
    .filter(|item| item.contains(search_text))
    .collect();
```

### 3. 默认选择
```rust
// 设置默认选中项
send_and_read(&mut main_stream, &json!({
    "method": "setSelection",
    "params": {"aid": aid, "id": spinner_id, "index": 2}
}))?;
```

### 4. 图标支持
```rust
// 在选项文本中使用 Emoji
let items = vec!["📱 iPhone", "🤖 Android", "💻 PC"];
```

## 学习价值

### 初学者
✅ 理解 Spinner 的基本用法  
✅ 学习事件处理  
✅ 掌握状态管理

### 进阶开发者
✅ 实现联动选择  
✅ 动态列表更新  
✅ 复杂状态追踪

### 实战应用
✅ 完整的订购流程  
✅ 数据验证  
✅ 用户体验优化

## 常见问题

### Q1: Spinner 选项太多怎么办？
A: 可以结合搜索功能，或使用分组显示。

### Q2: 如何实现三级联动？
A: 添加第三个 Spinner，监听第二个的 itemselected 事件。

### Q3: 如何获取当前选中项？
A: 保存 index，通过索引访问选项数组。

### Q4: Spinner 可以为空吗？
A: 可以，但建议至少包含一个"请选择"提示项。

## 相关示例

- `input_demo.rs` - 输入框示例
- `radio_demo.rs` - 单选组示例
- `checkbox_demo.rs` - 复选框示例

---

**特色功能**: 品牌-型号联动选择 🔗  
**难度等级**: ⭐⭐⭐  
**代码行数**: ~550 行  
**推荐指数**: ⭐⭐⭐⭐⭐

Happy Coding! 🦀
